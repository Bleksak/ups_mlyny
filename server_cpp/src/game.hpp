#pragma once

#include <array>
#include <mutex>

#include "board.hpp"

class Game {
    
    public:
        Game(std::string player) {
            m_players[0] = std::move(player);
            m_connected[0] = true;
        }
        
        ~Game() {
            
        }
        
        Game(Game&& other) {
            m_players = std::move(other.m_players);
            m_board = std::move(other.m_board);
            m_connected = std::move(m_connected);
        }
        
        auto players() const -> const std::array<std::string, 2>& {
            return m_players;
        }
        
        auto connected() const -> const std::array<bool, 2>& {
            return m_connected;
        }
        
        // game logic: player places stone
        // player moves stone
        // player removes enemy stone
        
    private:
        std::mutex m_mutex;
        Board m_board;
        
        // holds players' names
        std::array<Player, 2> m_players;
        std::array<bool, 2> m_connected;
};
