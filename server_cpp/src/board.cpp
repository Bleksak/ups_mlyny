#include "board.hpp"
#include <algorithm>
#include <mutex>

Board::Board() {
    
}

auto Board::board_bytes() -> std::array<uint32_t, Board::BOARD_SIZE> {
    const std::lock_guard<std::mutex> lock(m_mutex);
    std::array<uint32_t, Board::BOARD_SIZE> bytes;
    
    std::transform(m_board.begin(), m_board.end(), bytes.begin(), [](Color c) {
        return static_cast<uint32_t>(c);
    });
    
    return std::move(bytes);
}
