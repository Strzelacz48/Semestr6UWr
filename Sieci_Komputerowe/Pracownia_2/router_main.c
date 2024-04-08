#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <signal.h>
#include <stdbool.h>
#include <sys/time.h>

#define MAX_INTERFACES 10
#define MAX_BUFFER_SIZE 1024
#define BROADCAST_PORT 54321
#define MAX_ROUTES 100
#define INFINITY 9999
#define MAX_FAIL_TURNS 3

struct Interface {
    char ip_cidr[INET_ADDRSTRLEN + 3]; // IP w notacji CIDR (np. "192.168.1.1/24")
    char distance[10]; // Odległość do tej sieci
    int fail_turns; // Licznik tur bez odpowiedzi od sąsiada
};

struct Route {
    char network[INET_ADDRSTRLEN];
    char next_hop[INET_ADDRSTRLEN];
    int distance;
};

struct Interface interfaces[MAX_INTERFACES];
int num_interfaces = 0;
struct Route routing_table[MAX_ROUTES];
int num_routes = 0;

int sockfd;
struct sockaddr_in broadcast_addr;

// Funkcja obsługująca sygnał SIGALRM do cyklicznego wysyłania wektorów odległości
void handle_alarm(int sig) {
    // Wyślij wektor odległości na adresy rozgłoszeniowe wszystkich sąsiednich sieci
    for (int i = 0; i < num_interfaces; i++) {
        if (interfaces[i].fail_turns <= MAX_FAIL_TURNS) { // Sprawdź, czy sąsiad jest dostępny
            char message[MAX_BUFFER_SIZE];
            for (int j = 0; j < num_routes; j++) {
                if (strcmp(interfaces[i].ip_cidr, routing_table[j].next_hop) == 0) {
                    sprintf(message, "%s %d", routing_table[j].network, routing_table[j].distance);
                    sendto(sockfd, message, strlen(message), 0, (struct sockaddr *)&broadcast_addr, sizeof(broadcast_addr));
                    break;
                }
            }
        }
    }

    // Zwiększ licznik tur bez odpowiedzi od sąsiadów
    for (int i = 0; i < num_interfaces; i++) {
        if (interfaces[i].fail_turns > 0) {
            interfaces[i].fail_turns++;
            if (interfaces[i].fail_turns > MAX_FAIL_TURNS) { // Aktualizuj odległości do nieskończoności po przekroczeniu limitu tur
                for (int j = 0; j < num_routes; j++) {
                    if (strcmp(interfaces[i].ip_cidr, routing_table[j].next_hop) == 0) {
                        routing_table[j].distance = INFINITY;
                    }
                }
            }
        }
    }
}

// Funkcja wyświetlająca tablicę routingu
void display_routing_table() {
    printf("\nRouting Table:\n");
    printf("%-20s %-20s %-10s\n", "Network", "Next Hop", "Distance");
    for (int i = 0; i < num_routes; i++) {
        printf("%-20s %-20s %-10d\n", routing_table[i].network, routing_table[i].next_hop, routing_table[i].distance);
    }
}

// Funkcja aktualizująca tablicę routingu na podstawie odebranego wektora odległości
void update_routing_table(char *message) {
    char *token = strtok(message, " ");
    char network[INET_ADDRSTRLEN];
    strcpy(network, token);
    token = strtok(NULL, " ");
    int distance = atoi(token);

    for (int i = 0; i < num_routes; i++) {
        if (strcmp(routing_table[i].network, network) == 0) {
            routing_table[i].distance = distance;
            return;
        }
    }

    strcpy(routing_table[num_routes].network, network);
    strcpy(routing_table[num_routes].next_hop, "UNKNOWN");
    routing_table[num_routes].distance = distance;
    num_routes++;
}

// Funkcja obsługująca sygnał SIGINT do zakończenia programu
void handle_sigint(int sig) {
    printf("Exiting...\n");
    close(sockfd);
    exit(EXIT_SUCCESS);
}

int main() {
    // Ustawienie obsługi sygnałów
    signal(SIGALRM, handle_alarm);
    signal(SIGINT, handle_sigint);

    // Odczyt konfiguracji
    scanf("%d", &num_interfaces);
    for (int i = 0; i < num_interfaces; i++) {
        scanf("%s %s %s", interfaces[i].ip_cidr, interfaces[i].distance, interfaces[i].distance);
        interfaces[i].fail_turns = 0;
    }
    
    printf("data entered - num_interfaces: %d\n", num_interfaces);


    // Inicjalizacja gniazda
    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        perror("socket");
        exit(EXIT_FAILURE);
    }
    printf("socket created\n");
    // Ustawienia adresu rozgłoszeniowego
    memset(&broadcast_addr, 0, sizeof(broadcast_addr));
    broadcast_addr.sin_family = AF_INET;
    broadcast_addr.sin_addr.s_addr = htonl(INADDR_BROADCAST);
    broadcast_addr.sin_port = htons(BROADCAST_PORT);

    printf("broadcast address set\n");
    // Ustawienie alarmu do cyklicznego wysyłania wektorów odległości
    int interval = rand() % 16 + 15; // Losowy czas od 15 do 30 sekund
    struct itimerval timer;
    timer.it_value.tv_sec = interval;
    timer.it_value.tv_usec = 0;
    timer.it_interval = timer.it_value;
    setitimer(ITIMER_REAL, &timer, NULL);
    printf("timer set\n");
    // Główna pętla programu
    while (1) {
        // Odbierz i przetwórz wektor odległości
        char buffer[MAX_BUFFER_SIZE];
        printf("about to receive\n");
        ssize_t recv_len = recvfrom(sockfd, buffer, MAX_BUFFER_SIZE, 0, NULL, NULL);
        printf("received\n");
        if (recv_len < 0) {
            perror("recvfrom");
            continue;
        }
        buffer[recv_len] = '\0';
        printf("preupdate\n");
        update_routing_table(buffer);

        printf("\nReceived: %s\n", buffer);
        // Wyświetl tablicę routingu
        display_routing_table();

        // Wyczekaj na kolejny sygnał
        pause();
    }

    return 0;
}
