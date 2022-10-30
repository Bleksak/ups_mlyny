#pragma once
#include <stdlib.h>

typedef struct Player Player;
#include "game.h"

typedef struct Application {
    Player** players;
    Game** games;
    size_t player_count;
    size_t game_count;
    size_t game_identifier;
    size_t player_identifier;
    int server_socket;
} Application;

Application* app_new(int server_socket);
