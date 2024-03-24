CC = gcc

CFLAGS = -std=gnu99 -Wall -Wextra

traceroute: traceroute.o send.o
	$(CC) traceroute.o send.o -o traceroute

traceroute.o: traceroute.c
	$(CC) $(CFLAGS) -c traceroute.c

send.o: send.c
	$(CC) $(CFLAGS) -c send.c

clean: 
	rm *.o

distclean: clean
	rm traceroute
