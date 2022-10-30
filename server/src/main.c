#include "app.h"
#include <stdbool.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <unistd.h>
#include <netinet/in.h>
#include <pthread.h>
#include <stdio.h>
#include "server_messages.h"



int main(int argc, const char* argv[]) {
    int server_socket = socket(AF_INET, SOCK_STREAM, 0);
    
    int yes=1;

    if (setsockopt(server_socket, SOL_SOCKET, SO_REUSEADDR, &yes, sizeof(yes)) == -1) {
        perror("setsockopt");
        exit(1);
    }
    
    if(server_socket <= 0) {
        puts("Failed to start a server");
        return -1;
    }
    
    struct sockaddr_in local_addr = {0};
    
    local_addr.sin_family = AF_INET;
    local_addr.sin_port = htons(2000);
    local_addr.sin_addr.s_addr = INADDR_ANY;
    
    int code = bind(server_socket, (struct sockaddr*) &local_addr, sizeof(local_addr));
    
    if(code != 0) {
        printf("bind error");
        return -1;
    }
    
    printf("bind ok\n");
    
    code = listen(server_socket, 24);
    
    Application* app = app_new(server_socket);
    
    if(code != 0) {
        printf("listen error");
        return -1;
    }
    
    printf("listening!\n");
    
    while(true) {
        socklen_t client_addr_len;
        struct sockaddr_in client_addr = {0};
        int client_socket = accept(server_socket, (struct sockaddr*) &client_addr, &client_addr_len);
        if(client_socket > 0) {
            puts("got a connection!");
            RequestData* data = malloc(sizeof(RequestData));
            data->app = app;
            data->client_socket = client_socket;
            pthread_t thread;
            pthread_create(&thread, NULL, serve_request, data);
        }
    }
    
    return 0;
}
