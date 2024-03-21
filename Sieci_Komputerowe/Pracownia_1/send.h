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

int send_packets(int sockfd, int ttl, char* dest_addr, int id, int packets);

#endif  // SEND_H