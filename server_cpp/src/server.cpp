#include <asm-generic/errno-base.h>
#include <cstring>
#include <iostream>
#include <sstream>
#include <sys/select.h>
#include <sys/poll.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/ioctl.h>
#include <netinet/in.h>
#include <unistd.h>
#include <linux/sockios.h>

#include <memory>
#include <utility>
#include <algorithm>

#include "receiver.hpp"
#include "server.hpp"

Server::Server(std::uint16_t port) {
    m_socket = socket(AF_INET, SOCK_STREAM, 0);
    
    in_addr addr;
    std::memset(std::addressof(addr), 0, sizeof(addr));
    addr.s_addr = INADDR_ANY;
    
    sockaddr_in server_addr;
    std::memset(std::addressof(server_addr), 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(port);
    server_addr.sin_addr = addr;
    
    int on = 1;
    if (setsockopt(m_socket, SOL_SOCKET, SO_REUSEADDR, &on, sizeof(int)) < 0) {
        std::cerr << "Failed to reuse addr\n";
        std::exit(-1);
    }
    
    // if(ioctl(m_socket, FIONBIO, (char *)&on) < 0) {
    //     std::cerr << "ioctl error\n";
    //     exit(-1);
    // }

    int code = bind(m_socket, reinterpret_cast<struct sockaddr*>(&server_addr), sizeof(struct sockaddr_in));

    if (code != 0) {
        std::cerr << "Failed to bind socket\n";
        std::exit(-1);
    }

    code = listen(m_socket, queueSize);
    if(code != 0) {
        std::cerr << "Failed to listen\n";
        std::exit(-1);
    }
}

auto Server::start() -> std::thread {
    std::cout << "starting server!" << std::endl;
    return std::thread(Server::run, this);
}

auto Server::accept_client() -> int {
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
        
        auto item = std::find_if(m_players.begin(), m_players.end(), [&client] (const Player& player) {
            return player.socket() == client;
        });
        
        if(item == m_players.end()) {
            m_players.emplace_back(client);
            
            Message mesg(client, MessageType::PLAYER_INIT, 0, nullptr);
            sender().push_message(std::move(mesg));
            
            return client;
        }
    }
    
    return 0;
}

auto Server::sender() -> Sender& {
    return m_sender;
}

auto Server::receiver() -> Receiver& {
    return m_receiver;
}

auto Server::disconnect(int client) -> void {
    std::cout << "disconnecting: " << client << "\n";
    
    auto item = std::find_if(m_players.begin(), m_players.end(), [&client] (const Player& p) {
        return p.socket() == client;
    });
    
    if(item == m_players.end()) {
        return;
    }
    
    m_players.erase(item);
    
    // TODO: REMOVE THE CLOSE
    close(client);
}

auto Server::parse_messages(std::string message) -> void {
    std::istringstream iss(message);
    std::string msg;
    std::getline(iss, msg, '!');
}

auto Server::run(Server* server) -> void {
    
    std::vector<pollfd> fds;
    pollfd serverfd;
    serverfd.fd = server->m_socket;
    serverfd.events = POLLIN;
    
    fds.push_back(serverfd);
    
    // fd_set clients, readable;
    // FD_ZERO(&clients);
    // FD_SET(server->m_socket, &clients);
    
    while(true) {
        // std::memcpy(std::addressof(readable), std::addressof(clients), sizeof(fd_set));
        
        int edited = poll(&fds[0], fds.size(), -1);
        size_t client_count = fds.size();
        
        if(fds[0].revents & POLLIN) {
            int client = server->accept_client();
            pollfd fd;
            
            fd.fd = client;
            fd.events = POLLIN;
            fd.revents = 0;
            
            fds.push_back(fd);
            std::cout << "connected client: " << client << '\n';
            edited--;
        }
        
        for(size_t i = 1; i < client_count && edited > 0; ++i) {
            if(fds[i].revents & POLLIN) {
                // READ READ READ PLZ
                int bytes;
                
                ioctl(fds[i].fd, FIONREAD, &bytes);
                
                if(bytes > 0) {
                    std::vector<char> buffer;
                    buffer.resize(bytes);
                    recv(fds[i].fd, buffer.data(), buffer.size(), 0);
                    std::string msg(buffer.begin(), buffer.end());
                    
                    server->parse_messages(std::move(msg));
                    
                } else {
                    server->disconnect(i);
                }
                
                
                edited--;
            }
            fds[i].revents = 0;
        }
        
        // if(select(FD_SETSIZE, &readable, nullptr, nullptr, nullptr) < 0) {
        //     std::cerr << "select failed\n";
        //     std::exit(-1);
        // }
        
        
        
        // for(int i = 3; i < FD_SETSIZE; ++i) {
        // }
        
        std::this_thread::sleep_for(std::chrono::milliseconds(20));
    }
}

Server::~Server() {
//     close(m_socket);
}
