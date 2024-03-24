#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <netinet/ip_icmp.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <sys/types.h>
#include <unistd.h>
#include <errno.h>
//#include "icmp_checksum.h"
#include "send.c"

#define MAX_HOPS 30
#define PACKET_SIZE 4096
#define PORT_NUM 33434
#define TIMEOUT_SEC 1


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
/*
    // Main loop
    while (ttl <= MAX_HOPS) {
        ssize_t recv_bytes;
        char packet[IP_MAXPACKET];
        struct iphdr *ip_header = (struct iphdr *)packet;
        struct icmphdr *icmp_header = (struct icmphdr *)(packet + sizeof(struct iphdr));

        // Set TTL
        setsockopt(sockfd, IPPROTO_IP, IP_TTL, &ttl, sizeof(int));

        // Construct ICMP packet
        memset(packet, 0, IP_MAXPACKET);
        *icmp_header = create_icmp_header(ttl);
        
        struct sockaddr_in recipient;
        bzero (&recipient, sizeof(recipient));
        recipient.sin_family = AF_INET;
        inet_pton(AF_INET, dest_addr, &recipient.sin_addr);

        // Send packet
        if (sendto(sockfd, &icmp_header, sizeof(struct icmphdr), 0, (struct sockaddr *)&dest, sizeof(dest)) <= 0) {
            perror("Sendto failed");
            exit(EXIT_FAILURE);
        }

        // Receive packet
        struct sockaddr_in sender;
        socklen_t sender_len = sizeof(sender);

        recv_bytes = recvfrom(sockfd, &icmp_header, IP_MAXPACKET, 0, (struct sockaddr*)&sender, &sender_len);
        printf("recv_bytes: %ld\n", recv_bytes);
        if (recv_bytes < 0) {
            printf("%d\t*\n", ttl);
        } else {
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
*/
    // Close socket
    close(sockfd);

    return 0;
}

