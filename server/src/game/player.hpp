#pragma once

#include <string>

class Socket;
class Game;
class Player {
    public:
        enum Color {
            NONE,
            RED,
            BLUE,
        };
        // Player();
        // ~Player();
        
        std::string& username() {
            return m_username;
        }
        
        Socket* socket() {
            return m_socket;
        }
        
        size_t inventory() {
            return m_inventory;
        }
        
        Color color() {
            return m_color;
        }
    
    private:
        Socket* m_socket;
        std::string m_username;
        size_t m_inventory;
        Color m_color;
};
