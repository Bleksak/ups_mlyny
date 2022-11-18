#include "socket.hpp"

#include "../game/game.hpp"

Socket::~Socket() {
    if(m_socket != -1) {
        if(m_game) {
            m_game->notify_disconnect(*this);
        }
        
        close(m_socket);
    }
            
}