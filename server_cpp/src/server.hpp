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

class Server {
    public:
        Server(std::uint16_t port);
        ~Server();
        
        auto sender() -> Sender&;
        auto receiver() -> Receiver&;
        auto start() -> std::thread;
        
        auto find_player(int socket) -> Player*;
        auto find_player(std::string& name) -> Player*;
        
        auto players() -> std::vector<Player>&;
        
    private: 
        auto accept_client() -> int;
        auto disconnect(int client) -> void;
        auto parse_messages(int socket, std::vector<char> message) -> void;
        
        [[noreturn]]
        auto static run(Server* server) -> void;
        
        std::vector<pollfd> m_fds;
        
        Sender m_sender;
        Receiver m_receiver;
        
        std::vector<Player> m_players;
        const static int queueSize = 10;
        int m_socket;
        
        std::mutex m_player_mutex;
};
