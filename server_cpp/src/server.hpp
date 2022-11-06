#pragma once

#include <iostream>
#include <thread>
#include <vector>
#include <optional>
#include <unordered_map>

#include "player.hpp"
#include "sender.hpp"

class Server {
    public:
        Server(std::uint16_t port);
        ~Server();
        
        auto sender() -> Sender&;
        auto receiver();
        auto start() -> std::thread;
        
    private: 
        auto accept_client() -> int;
        
        [[noreturn]]
        auto static run(Server* server) -> void;
        
        Sender m_sender;
        std::vector<Player> m_players;
        const static int queueSize = 10;
        int m_socket;
};
