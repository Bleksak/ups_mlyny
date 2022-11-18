#pragma once

#include <array>
#include "player.hpp"

class Socket;
class Game {
    public:
        // Game();
        // ~Game();
        
        void notify_disconnect(Socket& sock);
        
    private:
        std::array<Player, 2> m_players;
};
