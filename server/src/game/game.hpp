#pragma once

#include <array>
#include "player.hpp"

class Socket;
class Game {
    public:
        // Game();
        // ~Game();
        
        void notify_disconnect(Socket& sock);
        std::array<Player, 2>& players() {
            return m_players;
        }
        
    private:
        std::array<Player, 2> m_players;
};
