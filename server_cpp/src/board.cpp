#include "board.hpp"
#include <algorithm>
#include <mutex>
#include <iostream>

static const std::vector<std::vector<size_t>> neighbors({
    {1, 6}, // 0 
    {0, 2, 4}, // 1
    {1, 14}, // 2
    {4, 10}, // 3
    {1, 3, 5, 7}, // 4
    {4, 13}, // 5
    {7, 11}, // 6
    {4, 6, 8}, // 7
    {7, 12}, // 8
    {0, 10, 21}, // 9
    {3, 9, 11, 18}, // 10
    {6, 10, 15}, // 11
    {8, 13, 17}, // 12
    {5, 12, 14, 20}, // 13
    {2, 13, 23}, // 14
    {11, 16}, // 15
    {15, 17, 19}, // 16
    {12, 16}, // 17
    {10, 19}, // 18
    {13, 18, 20, 22}, // 19
    {13, 19}, //20
    {9, 22}, // 21
    {19, 21, 23}, // 22
    {14, 22} // 23
});

Board::Board() {
    
}

Board::~Board() {
    
}

Board& Board::operator=(Board&& other) {
    m_board = std::move(other.m_board);
    return *this;
}

auto Board::get_neighbors(size_t index) -> const std::vector<size_t>& {
    return neighbors[index];
}

auto Board::occupy(Player &player, size_t index) -> int {
    const std::lock_guard<std::mutex> lock(m_mutex);
    
    if(index >= m_board.size()) {
        std::cout << "player attempted to occupy a non existing block\n";
        return -1;
    }
    
    Color& c = m_board.at(index);
    
    if(c != Color::NONE) {
        std::cout << "player attempted to occupy an already occupied block\n";
        return -1;
    }
    
    if(!player.move_to_board()) {
        std::cout << "player attempted to occupy with an empty inventory\n";
        return -1;
    }
    
    c = player.color();
    return 0;
}
        
auto Board::move(Player& player, size_t from, size_t to) -> int {
    const std::lock_guard<std::mutex> lock(m_mutex);
    
    if(from >= m_board.size() || to >= m_board.size()) {
        std::cout << "player attempted to move from/to a non existing block";
        return -1;
    }
    
    const std::vector<size_t>& neighbors = get_neighbors(from);
    if(std::find(neighbors.begin(), neighbors.end(), to) == neighbors.end()) {
        std::cout << "player attempted to move to a non neighbor\n";
        return -1;
    }
    
    Color& src = m_board.at(from);
    Color& dest = m_board.at(from);
    
    if(src != player.color()) {
        std::cout << "player attempted to move from a block that's not his\n";
        return -1;
    }
    
    if(dest != Color::NONE) {
        std::cout << "player attempted to move to an occupied block\n";
        return -1;
    }
    
    dest = src;
    src = Color::NONE;
    
    return 0;
}

auto Board::remove(Player& player, Player& opponent, size_t index) -> int {
    const std::lock_guard<std::mutex> lock(m_mutex);
    
    if(index >= m_board.size()) {
        std::cout << "player attempted to remove from a non existing block\n";
        return -1;
    }
    
    Color& c = m_board.at(index);
    if(c != opponent.color()) {
        std::cout << "player attempted to remove from a block that is not his opponent's color\n";
        return -1;
    }
    
    if(!opponent.decrease_board()) {
        std::cout << "opponent has empty board\n";
        return -1;
    }
    
    c = NONE;
    
    return 0;
}

auto Board::board() -> std::array<uint32_t, Board::BOARD_SIZE> {
    const std::lock_guard<std::mutex> lock(m_mutex);
    std::array<uint32_t, Board::BOARD_SIZE> bytes;
    
    std::transform(m_board.begin(), m_board.end(), bytes.begin(), [](Color c) {
        return static_cast<uint32_t>(c);
    });
    
    return bytes;
}
