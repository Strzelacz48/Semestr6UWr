#ifndef CHECKSUM_H
#define CHECKSUM_H
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

u_int16_t compute_icmp_checksum (const void *buff, int length);
#endif