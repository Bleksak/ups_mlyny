#pragma once

#include <array>
#include <vector>
#include <mutex>

enum Color {
    NONE = 0,
    RED,
    BLUE
};

#include "player.hpp"

class Board {
    public:
        
        Board();
        ~Board();
        Board& operator=(Board&& other);
        
        const static size_t BOARD_SIZE = 24;
        
        auto board() -> std::array<uint32_t, BOARD_SIZE>;
        
        auto occupy(Player& player, size_t index) -> int;
        auto move(Player& player, size_t from, size_t to) -> int;
        auto remove(Player& player, Player& opponent, size_t index) -> int;
    private:
        auto get_neighbors(size_t index) -> const std::vector<size_t>&;
        
        std::array<Color, BOARD_SIZE> m_board;
        std::mutex m_mutex;
};
