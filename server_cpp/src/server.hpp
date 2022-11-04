#pragma once

#include <thread>
#include <vector>
#include <optional>

#include "player.hpp"

class Server {
    public:
        Server(std::uint16_t port);
        ~Server();
        
        auto accept_client() -> std::optional<std::thread>;
        auto run() -> void;
        
    private: 
        
        std::vector<Player> players;
        const static int queueSize = 10;
        const int m_socket;
};
