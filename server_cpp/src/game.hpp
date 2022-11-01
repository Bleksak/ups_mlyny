#pragma once

#include <array>
#include <mutex>

#include "board.hpp"

class Game {
    
    public:
        Game();
        ~Game();
    
    
    private:
        Board m_board;
        std::mutex m_mutex;
};