#include <iostream>
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <unistd.h>
#include <memory>

#include "server.hpp"

Server::Server(std::uint16_t port):m_socket( socket(AF_INET, 0, sizeof(struct sockaddr_in)) ) {
    int returnValue;
    sockaddr_in server_addr = {
        .sin_family = AF_INET,
        .sin_port = htons(port),
        .sin_addr = {
            .s_addr = INADDR_ANY,
        },
        .sin_zero = {0},
    };

    int enable = 1;
    if (setsockopt(m_socket, SOL_SOCKET, SO_REUSEADDR, &enable, sizeof(int)) < 0) {
        std::cerr << "Failed to reuse addr\n";
        std::exit(-1);
    }

    returnValue = bind(m_socket, (struct sockaddr *) &server_addr, sizeof(struct sockaddr_in));

    if (returnValue != 0) {
        std::cerr << "Failed to bind a socket\n";
        std::exit(-1);
    }

    listen(m_socket, queueSize);
}
        
auto Server::accept_client() -> std::optional<std::thread> {
    sockaddr addr;
    socklen_t len;
    int client = accept(m_socket, std::addressof(addr), std::addressof(len));
    
    if(client > 0) {
        
        // check if client is connected
        int val = 1;
        if(setsockopt(client, SOL_SOCKET, SO_KEEPALIVE, &val, sizeof(val)) < 0) {
            std::cerr << "Failed to set KEEP ALIVE\n";
            close(client);
            std::exit(-1);
        }
        
        return std::optional<std::thread>(std::thread());
    }
    
    return std::optional<std::thread>();
}

auto Server::run() -> void {
    while(true) {
        
    }
}

Server::~Server() {
    close(m_socket);
}
