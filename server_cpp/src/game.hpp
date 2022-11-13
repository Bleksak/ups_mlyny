#pragma once

#include <array>
#include <mutex>

#include "board.hpp"

class Game {
    
    public:
        Game();
        ~Game();
        
        // game logic: player places stone
        // player moves stone
        // player removes enemy stone
        
    private:
        // holds players' names
        std::array<std::string, 2> m_players;
        Board m_board;
        std::mutex m_mutex;
};
