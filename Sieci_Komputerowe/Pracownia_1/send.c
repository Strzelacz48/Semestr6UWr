#ifndef SEND_H
#define SEND_H
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
#include "send.h"
#endif

u_int16_t compute_icmp_checksum (const void *buff, int length)
{
    const u_int16_t* ptr = buff;
    u_int32_t sum = 0;
    assert (length % 2 == 0);
    for (; length > 0; length -= 2)
        sum += *ptr++;
    sum = (sum >> 16U) + (sum & 0xffffU);
    return (u_int16_t)(~(sum + (sum >> 16U)));
}

struct icmp create_icmp_header(u_int16_t seq) {
    struct icmp header;
    header.icmp_type = ICMP_ECHO;
    header.icmp_code = 0;
    header.icmp_hun.ih_idseq.icd_id = htons(getpid());
    header.icmp_hun.ih_idseq.icd_seq = htons(seq);
    header.icmp_cksum = 0;
    header.icmp_cksum = compute_icmp_checksum (
    (u_int16_t*)&header, sizeof(header));
    return header;
}

int send_packet(int sockfd, int ttl, char* dest_addr, int packets){
        
    struct icmp header = create_icmp_header(ttl * packets);

    struct sockaddr_in recipient;
    bzero (&recipient, sizeof(recipient));
    recipient.sin_family = AF_INET;
    if(inet_pton(AF_INET, dest_addr, &recipient.sin_addr)){
        perror("inet_pton error");
        exit(EXIT_FAILURE);
    }

    //set ttl
    if (setsockopt(sockfd, IPPROTO_IP, IP_TTL, &ttl, sizeof(int)) != 0) {
    fprintf(stderr, "setsockopt error: %s\n", strerror(errno));
    return EXIT_FAILURE;
    }

    // send packets
  for (int i = 0; i < packets; i++) {
    ssize_t bytes_sent = sendto(sockfd, &header, sizeof(header), 0, 
      (struct sockaddr*)&recipient, sizeof(recipient));

    if (bytes_sent < 0) {
      fprintf(stderr, "sendto error: %s\n", strerror(errno));
      return EXIT_FAILURE;
    }

    // aktualizacja numeru sekwencyjnego
    //printf("Wysłano id = %d | seq = %d\n", ntohs(header.icmp_hun.ih_idseq.icd_id), ntohs(header.icmp_hun.ih_idseq.icd_seq));
    header.icmp_hun.ih_idseq.icd_seq = htons(ntohs(header.icmp_hun.ih_idseq.icd_seq) + 1);
  }

  return EXIT_SUCCESS;

}

/*
    //old code
    ssize_t recv_bytes;
    char packet[IP_MAXPACKET];
    struct iphdr *ip_header = (struct iphdr *)packet;
    struct icmp header;

    // Set TTL
    setsockopt(sockfd, IPPROTO_IP, IP_TTL, &ttl, sizeof(int));

    // Construct ICMP packet
    memset(packet, 0, IP_MAXPACKET);
    header = create_icmp_header(ttl);
    
    struct sockaddr_in recipient;
    bzero (&recipient, sizeof(recipient));
    recipient.sin_family = AF_INET;
    inet_pton(AF_INET, dest_addr, &recipient.sin_addr);

    // Send packet
    if (sendto(sockfd, &header, sizeof(struct icmp), 0, (struct sockaddr *)&dest, sizeof(dest)) <= 0) {
        perror("Sendto failed");
        exit(EXIT_FAILURE);
    }*/