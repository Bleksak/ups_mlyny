#pragma once

#include <string>
#include <unistd.h>
#include "../machine/machine.hpp"

class Game;
class Player;
class Socket {
    public:
        Socket(int sock) : m_socket(sock), m_machine(*this) {}
        
        // nenavidim c++ kvuli takovymhle srackam
        Socket(Socket&& o) : m_machine(std::move(o.m_machine)) {
            m_socket = o.m_socket;
            m_identifier = std::move(o.m_identifier);
            m_player = o.m_player;
            m_game = o.m_game;
            o.m_socket = -1;
            
            m_machine.update_socket(*this);
        }
        
        // nenavidim c++ kvuli takovymhle srackam
        Socket& operator=(Socket&& o) {
            m_socket = o.m_socket;
            m_identifier = std::move(o.m_identifier);
            m_machine = std::move(o.m_machine);
            m_player = o.m_player;
            m_game = o.m_game;
            o.m_socket = -1;
            
            m_machine.update_socket(*this);
            return *this;
        }
        
        Game* game() {
            return m_game;
        }
        
        Player* player() {
            return m_player;
        }
        
        void set_game(Game* game) {
            m_game = game;
        }
        
        auto bind(std::string id) -> void {
            m_identifier = std::move(id);
        }
        
        ~Socket();
        
    private:
        int m_socket;
        std::string m_identifier;
        Machine m_machine;
        Game* m_game;
        Player* m_player;
};
