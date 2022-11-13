#include <asm-generic/errno-base.h>
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

Server::Server(std::uint16_t port) : m_receiver(*this) {
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
        
        Player p(client);
        
        if(m_players.push_back_if(std::move(p), [&client](const Player& player) {
            return player.socket() == client;
        })) {
            Message msg(client, MessageType::PLAYER_INIT, 0, nullptr);
            sender().push_message(std::move(msg));
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

auto Server::disconnect(int index) -> void {
    m_fds.erase(m_fds.begin() + index);
    
    m_players.find_and_erase([this, &index] (const Player& p) {
        return p.socket() == m_fds[index].fd;
    });
}

auto Server::games() -> ConcurrentVector<Game>& {
    return m_games;
}

auto Server::parse_messages(int socket, std::vector<char> message) -> void {
    
    // find \n
    // read (N-msg_type) bytes from message
    // push to receiver()
    // repeat
    
    auto search_start = message.begin();
    
    while(true) {
        if(search_start == message.end()) {
            std::cout << "END?\n";
            break;
        }
        
        if(search_start + 4 >= message.end()) {
            std::cout << "packet too short\n";
            break;
        }
        
        uint32_t msg_len = *reinterpret_cast<uint32_t*>(&*search_start);
        
        if(search_start + msg_len > message.end()) {
            std::cout << "invalid msg len\n";
            break;
        }
        
        auto endl = std::find(search_start + 4, message.end(), '\n');
        if(endl == message.end()) {
            std::cout << "END?";
            break;
        }
        
        std::string msg_type(search_start + 4, endl+1);
        
        std::vector<char> msg_data(endl+1, search_start + msg_len);
        search_start = search_start + msg_len;
        
        MessageType type = RecvMessage::get_type(msg_type);
        
        if(type == MessageType::INVALID) {
            std::cout << "invalid message\n";
            continue;
        }
        
        RecvMessage msg(socket, type, std::move(msg_data));
        receiver().push_message(std::move(msg));
    }
}
        
// auto Server::find_player(int socket) -> Player* {
//     const std::lock_guard<std::mutex> lock(m_player_mutex);
  
//     auto it = std::find_if(m_players.begin(), m_players.end(), [&socket] (const Player& player) {
//         return player.socket() == socket;
//     });
    
//     if(it == m_players.end()) {
//         return nullptr;
//     }
    
//     return std::addressof(*it);
// }

auto Server::players() -> ConcurrentVector<Player>& {
    return m_players;
}

// auto Server::find_player(std::string& name) -> Player* {
//     const std::lock_guard<std::mutex> lock(m_player_mutex);
    
//     auto it = std::find_if(m_players.begin(), m_players.end(), [&name] (const Player& player) {
//         return player.name() == name;
//     });
    
//     if(it == players().end()) {
//         return nullptr;
//     }
    
//     return std::addressof(*it);
// }

auto Server::run(Server* server) -> void {
    // fd_set clients, readable;
    // FD_ZERO(&clients);
    // FD_SET(server->m_socket, &clients);
    std::queue<int> disconnected;
    
    while(true) {
        // std::memcpy(std::addressof(readable), std::addressof(clients), sizeof(fd_set));
        int edited = poll(&server->m_fds[0], server->m_fds.size(), -1);
        size_t client_count = server->m_fds.size();
        
        if(server->m_fds[0].revents & POLLIN) {
            int client = server->accept_client();
            
            if(client >= 0) {
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
