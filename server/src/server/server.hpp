#pragma once

#include <iostream>
#include <mutex>
#include <thread>
#include <vector>
#include <optional>
#include <unordered_map>
#include <sys/poll.h>

// #include "player.hpp"
#include "sender.hpp"
#include "receiver.hpp"
// #include "game.hpp"
#include "../container/cvector.hpp"
#include "socket.hpp"
// #include "game_destroyer.hpp"

class Server {
    public:
        Server(std::uint16_t port);
        ~Server();
        
        auto sender() -> Sender&;
        auto receiver() -> Receiver&;
        // auto destroyer() -> GameDestroyer&;
        
        std::thread start();
        ConcurrentVector<Socket>& sockets();
        
        // auto games() -> ConcurrentVector<Game>&;
        
    private: 
        int accept_client();
        void disconnect(int client);
        void parse_messages(int socket, std::vector<char> message);
        
        [[noreturn]]
        void static run(Server* server);
        
        std::vector<pollfd> m_fds;
        
        Sender m_sender;
        Receiver m_receiver;
        // GameDestroyer m_destroyer;
        
        ConcurrentVector<Socket> m_sockets;
        // ConcurrentVector<Game> m_games;
        const static int queueSize = 10;
        int m_socket;
};
