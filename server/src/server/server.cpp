#include <cstring>
#include <iostream>
#include <mutex>
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
#include <unordered_set>

#include "message.hpp"
#include "receiver.hpp"
#include "server.hpp"
#include "socket.hpp"

Server::Server(std::uint16_t port) : m_receiver(this) /*m_destroyer(*this)*/ {
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
    
    pollfd serverfd;
    serverfd.fd = m_socket;
    serverfd.events = POLLIN;
    
    m_fds.push_back(serverfd);
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
        int val = 1;
        if(setsockopt(client, SOL_SOCKET, SO_KEEPALIVE, &val, sizeof(val)) < 0) {
            std::cerr << "Failed to set KEEP ALIVE\n";
            close(client);
            std::exit(-1);
        }
        
        m_sockets.put(client, Socket(client, this));
        
        Message msg(client, Message::Type::INIT, 0, nullptr);
        sender().push_message(std::move(msg));
        
        return client;
    }
    
    return 0;
}

auto Server::sender() -> Sender& {
    return m_sender;
}

auto Server::receiver() -> Receiver& {
    return m_receiver;
}

auto Server::disconnect(int index) -> void {
    int sock = m_fds[index].fd;
    
    sockets().erase(sock);
    m_fds.erase(m_fds.begin() + index);
}

auto Server::games() -> ConcurrentVector<Game>& {
    return m_games;
}

auto Server::parse_messages(int socket, std::vector<char> message) -> void {
    
    // read N = sizeof(uint32_t) (message length)
    // read sizeof(uint32_t) (message type)
    // read (N-2*sizeof(uint32_t)) bytes from message
    // push to receiver()
    // repeat
    
    auto search_start = message.begin();
    
    while(true) {
        if(search_start == message.end()) {
            std::cout << "END?\n";
            break;
        }
        
        if(search_start + sizeof(uint32_t) >= message.end()) {
            std::cout << "packet too short\n";
            break;
        }
        
        uint32_t msg_len = ntohl(*reinterpret_cast<uint32_t*>(&*search_start));
        
        std::cout << "len:" << msg_len << std::endl;
        
        if(search_start + msg_len > message.end()) {
            std::cout << "invalid msg len\n";
            break;
        }
        
        uint32_t msg_type = ntohl(*reinterpret_cast<uint32_t*>(&*(search_start + sizeof(uint32_t))));
        
        std::vector<char> msg_data;
        if(msg_len > 2 * sizeof(uint32_t)) {
            msg_data = std::vector<char>(search_start + 2 * sizeof(uint32_t), search_start + msg_len);
        }
        
        search_start = search_start + msg_len;
        Message msg(socket, static_cast<Message::Type>(msg_type), std::move(msg_data));
        receiver().push_message(std::move(msg));
    }
}

ConcurrentUnorderedMap<int, Socket>& Server::sockets() {
    return m_sockets;
}

auto Server::run(Server* server) -> void {
    std::queue<int> disconnected;
    
    while(true) {
        int edited = poll(&server->m_fds[0], server->m_fds.size(), -1);
        size_t client_count = server->m_fds.size();
        
        if(server->m_fds[0].revents & POLLIN) {
            int client = server->accept_client();
            
            if(client > 0) {
                pollfd fd;
            
                fd.fd = client;
                fd.events = POLLIN;
                fd.revents = 0;
            
                server->m_fds.push_back(fd);
                std::cout << "connected client: " << client << '\n';
            }
            
            edited--;
        }
        
        for(size_t i = 1; i < client_count && edited > 0; ++i) {
            if(server->m_fds[i].revents & POLLIN) {
                std::cout << "client: " << server->m_fds[i].fd << " can read\n";
                int bytes;
                
                ioctl(server->m_fds[i].fd, FIONREAD, &bytes);
                
                if(bytes > 0) {
                    std::vector<char> buffer;
                    buffer.resize(bytes);
                    recv(server->m_fds[i].fd, buffer.data(), buffer.size(), 0);
                    
                    server->parse_messages(server->m_fds[i].fd, std::move(buffer));
                } else {
                    std::cout << "disconnecting " << server->m_fds[i].fd << std::endl;
                    disconnected.push(i);
                }
                
                edited--;
            }
            
            server->m_fds[i].revents = 0;
        }
        
        while(!disconnected.empty()) {
            server->disconnect(disconnected.back());
            disconnected.pop();
        }
        
        std::this_thread::sleep_for(std::chrono::milliseconds(20));
    }
}

Server::~Server() {
//     close(m_socket);
}
