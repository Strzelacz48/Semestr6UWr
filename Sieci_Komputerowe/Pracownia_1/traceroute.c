#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <netinet/ip_icmp.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <unistd.h>

#define MAX_HOPS 64
#define PACKET_SIZE 4096
#define PORT_NUM 33434
#define TIMEOUT_SEC 1

unsigned short checksum(void *b, int len) {
    unsigned short *buf = b;
    unsigned int sum = 0;
    unsigned short result;

    for (sum = 0; len > 1; len -= 2)
        sum += *buf++;
    if (len == 1)
        sum += *(unsigned char *)buf;
    sum = (sum >> 16) + (sum & 0xFFFF);
    sum += (sum >> 16);
    result = ~sum;
    return result;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Usage: %s <destination_ip>\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    char *dest_addr = argv[1];
    struct sockaddr_in dest;
    int sockfd;
    int ttl = 1;
    struct timeval timeout;
    timeout.tv_sec = TIMEOUT_SEC;
    timeout.tv_usec = 0;

    // Create socket
    sockfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if (sockfd < 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    // Set socket options
    setsockopt(sockfd, SOL_SOCKET, SO_RCVTIMEO, (char *)&timeout, sizeof(struct timeval));

    // Fill in destination details
    bzero(&dest, sizeof(dest));
    dest.sin_family = AF_INET;
    dest.sin_port = htons(PORT_NUM);
    if (inet_aton(dest_addr, &dest.sin_addr) == 0) {
        printf("Invalid address: %s\n", dest_addr);
        exit(EXIT_FAILURE);
    }

    // Main loop
    while (ttl <= MAX_HOPS) {
        int recv_bytes;
        char packet[PACKET_SIZE];
        struct iphdr *ip_header = (struct iphdr *)packet;
        struct icmphdr *icmp_header = (struct icmphdr *)(packet + sizeof(struct iphdr));

        // Set TTL
        setsockopt(sockfd, IPPROTO_IP, IP_TTL, &ttl, sizeof(int));

        // Construct ICMP packet
        memset(packet, 0, PACKET_SIZE);
        icmp_header->type = ICMP_ECHO;
        icmp_header->code = 0;
        icmp_header->un.echo.id = getpid();
        icmp_header->un.echo.sequence = ttl;
        icmp_header->checksum = checksum(icmp_header, sizeof(struct icmphdr));

        // Send packet
        if (sendto(sockfd, packet, sizeof(struct icmphdr), 0, (struct sockaddr *)&dest, sizeof(dest)) <= 0) {
            perror("Sendto failed");
            exit(EXIT_FAILURE);
        }

        // Receive packet
        recv_bytes = recvfrom(sockfd, packet, PACKET_SIZE, 0, NULL, NULL);
        if (recv_bytes < 0) {
            printf("%d\t*\n", ttl);
        } else {
            struct sockaddr_in sender;
            char sender_ip_str[INET_ADDRSTRLEN];
            inet_ntop(AF_INET, &(sender.sin_addr), sender_ip_str, INET_ADDRSTRLEN);
            printf("%d\t%s\n", ttl, sender_ip_str);
        }

        // Check if the destination is reached
        if (ip_header->daddr == dest.sin_addr.s_addr) {
            printf("Destination reached.\n");
            break;
        }

        // Increment TTL for the next iteration
        ttl++;
    }

    // Close socket
    close(sockfd);

    return 0;
}
