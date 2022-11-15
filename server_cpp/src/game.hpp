#pragma once

#include <array>
#include <chrono>
#include <mutex>
#include <optional>

#include "board.hpp"

class Game {
    
    public:
        Game(Player&& player) {
            m_players[0] = std::move(player);
            m_players[0].set_color(RED);
            m_players[1].set_color(BLUE);
            // we only set connected flag for first connection
            m_connected[0] = true;
        }
        
        ~Game() {}
        
        Game(Game&& other) {
            m_players = std::move(other.m_players);
            m_board = std::move(other.m_board);
            m_connected = std::move(other.m_connected);
            m_disconnected_timer = std::move(other.m_disconnected_timer);
        }
        
        Game& operator=(Game&& other) {
            m_players = std::move(other.m_players);
            m_board = std::move(other.m_board);
            m_connected = std::move(other.m_connected);
            m_disconnected_timer = std::move(other.m_disconnected_timer);
            
            return *this;
        }
        auto players() const -> const std::array<Player, 2>& {
            return m_players;
        }
        
        auto connected() const -> const std::array<bool, 2>& {
            return m_connected;
        }
        
        auto connect(size_t index) -> void {
            m_connected[index] = true;
            if(m_connected[0] && m_connected[1]) {
                m_disconnected_timer = std::nullopt;
            }
        }
        
        auto disconnect(size_t index) -> void {
            m_connected[index] = false;
            m_disconnected_timer = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch());
        }
        
        auto timer() -> std::optional<std::chrono::milliseconds>& {
            return m_disconnected_timer;
        }
        
        // game logic: player places stone
        // player moves stone
        // player removes enemy stone
        
    private:
        std::mutex m_mutex;
        Board m_board;
        
        std::array<Player, 2> m_players;
        std::array<bool, 2> m_connected;
        std::optional<std::chrono::milliseconds> m_disconnected_timer = std::nullopt;
};
