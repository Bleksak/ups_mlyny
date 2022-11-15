#pragma once

#include <thread>
#include <vector>
#include "game.hpp"

class Server;
class GameDestroyer {
    public:
        GameDestroyer(Server& server) : m_server(server) {
            std::thread(GameDestroyer::run, this).detach();
        }
        
        ~GameDestroyer() {}
    
    private:
        
        static const uint64_t TIMEOUT = 30000;
        static void run(GameDestroyer* destroyer);
        
        Server& m_server;
};