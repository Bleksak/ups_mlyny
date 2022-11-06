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
        Board m_board;
        std::mutex m_mutex;
};
