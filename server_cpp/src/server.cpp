#include "server.hpp"
#include <sys/socket.h>

Server::Server(std::uint16_t port) {
    int returnValue;

    m_socket = socket(AF_INET, SOCK_STREAM, 0);

    memset(&myAddr, 0, sizeof(struct sockaddr_in));

    myAddr.sin_family = AF_INET;
    myAddr.sin_port = htons(listeningPort);
    myAddr.sin_addr.s_addr = INADDR_ANY;

    int enable = 1;
    if (setsockopt(serverSocket, SOL_SOCKET, SO_REUSEADDR, &enable, sizeof(int)) < 0) {
        
    }

    returnValue = bind(serverSocket, (struct sockaddr *) &myAddr, sizeof(struct sockaddr_in));

    if (returnValue != 0) {
        std::exit(-1);
    }

    listen(serverSocket, queueSize);
}
