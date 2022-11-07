#pragma once

#include <iostream>
#include <type_traits>
#include <unistd.h>
class Player;

#include "message.hpp"
#include "board.hpp"

class Player {
    
    public:
        Player(int socket) : m_socket(socket) {
            // TODO: player has no color and game, we need to ask him
        }
        
        auto color() -> const Color& {
            return m_color;
        }
        
        ~Player() {
            close(m_socket);
        }
        
        Player& operator=(const Player&& p) {
            m_board_count = std::move(p.m_board_count);
            m_inventory_count = std::move(p.m_inventory_count);
            m_color = std::move(p.m_color);
            m_socket = std::move(p.m_socket);
            
            return *this;
        }
        
        Player(Player&& p) {
            std::swap(p.m_board_count, m_board_count);
            std::swap(p.m_inventory_count, m_inventory_count);
            std::swap(p.m_color, m_color);
            std::swap(p.m_socket, m_socket);
        }
        
        auto board_count() -> size_t {
            return m_board_count;
        }
        
        auto inventory_count() -> size_t {
            return m_inventory_count;
        }
        
        auto decrease_board() -> bool {
            const std::lock_guard<std::mutex> lock(m_mutex);
            if(m_board_count != 0) {
                return false;
            }
            
            m_board_count -= 1;
            return true;
        }
        
        auto move_to_board() -> bool {
            const std::lock_guard<std::mutex> lock(m_mutex);
            
            if(m_inventory_count == 0) {
                return false;
            }
            
            m_board_count += 1;
            m_inventory_count -= 1;
            
            return true;
        }
        
        auto socket() const -> int {
            return m_socket;
        }
    
    private:
        std::mutex m_mutex;
        int m_socket;
        Color m_color;
        size_t m_board_count;
        size_t m_inventory_count;
};