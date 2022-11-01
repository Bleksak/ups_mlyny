#include "board.hpp"
#include <algorithm>
#include <mutex>

Board::Board() {
    
}

std::array<uint32_t, Board::BOARD_SIZE> Board::board_bytes() {
    const std::lock_guard<std::mutex> lock(m_mutex);
    std::array<uint32_t, Board::BOARD_SIZE> bytes;
    
    std::transform(m_board.begin(), m_board.end(), bytes.begin(), []() {
        
    });
    
    return std::move(bytes);
}
