#pragma once

#include <string>

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
    
    private:
        std::string username;
        size_t m_inventory;
        Color m_color;
};
