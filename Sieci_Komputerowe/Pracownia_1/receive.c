#include <arpa/inet.h>
#include <errno.h>
#include <netinet/ip.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


void print_as_bytes (unsigned char* buff, ssize_t length)
{
	for (ssize_t i = 0; i < length; i++, buff++)
		printf ("%.2x ", *buff);	
}


int main()
{
	int sock_fd = socket(AF_INET, SOCK_RAW, IPPROTO_ICMP);
	if (sock_fd < 0) {
		fprintf(stderr, "socket error: %s\n", strerror(errno)); 
		return EXIT_FAILURE;
	}

	for (;;) {
		struct sockaddr_in sender;
		socklen_t sender_len = sizeof(sender);
		u_int8_t buffer[IP_MAXPACKET];

		ssize_t packet_len = recvfrom (sock_fd, buffer, IP_MAXPACKET, 0, (struct sockaddr*)&sender, &sender_len);
		if (packet_len < 0) {
			fprintf(stderr, "recvfrom error: %s\n", strerror(errno));
			return EXIT_FAILURE;
		}

		char sender_ip_str[20]; 
		inet_ntop(AF_INET, &(sender.sin_addr), sender_ip_str, sizeof(sender_ip_str));
		printf ("Received IP packet with ICMP content from: %s\n", sender_ip_str);

		struct ip* ip_header = (struct ip*) buffer;
		ssize_t	ip_header_len = 4 * (ssize_t)(ip_header->ip_hl);

		printf("IP header: ");
		print_as_bytes(buffer, ip_header_len);
		printf("\n");

		printf("IP data:   ");
		print_as_bytes(buffer + ip_header_len, packet_len - ip_header_len);
		printf("\n\n");
	}
}

int receive(){
	// Receive packet
    struct sockaddr_in sender;
    socklen_t sender_len = sizeof(sender);

    recv_bytes = recvfrom(sockfd, &header, IP_MAXPACKET, 0, (struct sockaddr*)&sender, &sender_len);
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
        return 1;
        //break;
    }
}