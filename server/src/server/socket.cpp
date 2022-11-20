#include "socket.hpp"

#include "../game/game.hpp"
#include "server.hpp"
#include <memory>

Socket::Socket(Socket&& o) {
    m_machine = o.m_machine;
    m_socket = o.m_socket;
    m_identifier = std::move(o.m_identifier);
    m_player = o.m_player;
    m_game = o.m_game;
    m_server = o.m_server;
    
    o.m_socket = -1;
    o.m_machine = nullptr;
    o.m_server = nullptr;
    o.m_game = nullptr;
    o.m_player = nullptr;
    
    m_machine->update_socket(this);
}
        
Socket& Socket::operator=(Socket&& o) {
    m_machine = o.m_machine;
    m_socket = o.m_socket;
    m_identifier = std::move(o.m_identifier);
    m_player = o.m_player;
    m_game = o.m_game;
    m_server = o.m_server;
    
    o.m_socket = -1;
    o.m_machine = nullptr;
    o.m_server = nullptr;
    o.m_game = nullptr;
    o.m_player = nullptr;
    
    m_machine->update_socket(this);
    return *this;
}

Socket::~Socket() {
    if(m_socket != -1) {
        if(m_game) {
            m_game->notify_disconnect(*this);
        }
        
        delete m_machine;
        close(m_socket);
    }
            
}
Player* Socket::player() {
    // todo: find a way to associate socket with a player
    if(!m_player) {
        if(m_identifier.empty()) {
            return nullptr;
        }
                
        // loop through all games in server
        // if we find a player with the same name (who's disconnected), we assoc
        m_server->games().atomic_op<void>([this] (std::vector<Game>& games) {
            for(Game& g : games) {
                for(Player& p : g.players()) {
                    if(p.socket() != nullptr && p.username() == m_identifier) {
                        m_player = std::addressof(p);
                        m_game = std::addressof(g);
                        return;
                    }
                }
            }
        });
    }
            
    return m_player;
}
