#pragma once

#include <string>
#include <unistd.h>
#include "../machine/machine.hpp"

class Game;
class Player;
class Server;
class Socket {
    public:
        Socket(int sock, Server* server) : m_machine(new Machine(this)), m_socket(sock), m_server(server) {}
        
        // nenavidim c++ kvuli takovymhle srackam
        Socket(Socket&& o);
        
        // nenavidim c++ kvuli takovymhle srackam
        Socket& operator=(Socket&& o);
        
        Player* player();
        
        Game* game() {
            return m_game;
        }
        
        void set_game(Game* game) {
            m_game = game;
        }
        
        auto bind(std::string id) -> void {
            m_identifier = std::move(id);
        }
        
        Machine* machine() {
            return m_machine;
        }
        
        Server* server() {
            return m_server;
        }
        
        ~Socket();
        
    private:
        Machine* m_machine;
        int m_socket;
        std::string m_identifier;
        Server* m_server;
        Game* m_game;
        Player* m_player;
};
