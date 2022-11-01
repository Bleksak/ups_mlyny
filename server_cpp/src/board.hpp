#pragma once

#include <array>
#include <mutex>

enum Color {
    NONE = 0,
    RED,
    BLUE
};

class Board {
    public:
        const static size_t BOARD_SIZE = 24;
        
        std::array<uint32_t, BOARD_SIZE> board_bytes();
        Board();
    private:
        std::array<Color, BOARD_SIZE> m_board;
        std::mutex m_mutex;
};