#include "game.hpp"

Game::Game() {
    const std::lock_guard<std::mutex> lock(m_mutex);
}

Game::~Game() {
    
}
