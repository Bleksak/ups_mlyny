#pragma once

#include <iostream>
#include <mutex>
#include <thread>
#include <vector>
#include <optional>
#include <unordered_map>
#include <sys/poll.h>

#include "player.hpp"
#include "sender.hpp"
#include "receiver.hpp"
#include "game.hpp"
#include "cvector.hpp"
#include "socket.hpp"

class Server {
    public:
        Server(std::uint16_t port);
        ~Server();
        
        auto sender() -> Sender&;
        auto receiver() -> Receiver&;
        auto start() -> std::thread;
        
        auto sockets() -> ConcurrentVector<Socket>&;
        auto games() -> ConcurrentVector<Game>&;
        
    private: 
        auto accept_client() -> int;
        auto disconnect(int client) -> void;
        auto parse_messages(int socket, std::vector<char> message) -> void;
        
        [[noreturn]]
        auto static run(Server* server) -> void;
        
        std::vector<pollfd> m_fds;
        
        Sender m_sender;
        Receiver m_receiver;
        
        ConcurrentVector<Socket> m_sockets;
        ConcurrentVector<Game> m_games;
        const static int queueSize = 10;
        int m_socket;
};
