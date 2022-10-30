#pragma once

typedef struct Game Game;

#include "app.h"
#include <stdlib.h>


typedef enum Occupation {
    NONE,
    RED,
    BLUE,
} Occupation;

typedef struct Player {
    Game* game;
    Occupation occupation;
    size_t identifier;
    size_t board; // how many stones are on board
    size_t inventory; // how many stones are in inventory
    int socket; // client socket
} Player;

typedef struct Game{
    Occupation* board;
    Player players[2];
    size_t player_count;
    size_t current_player;
    size_t identifier;
} Game;

typedef enum State {
    GAME_RUNNING,
    GAME_OVER_RED_WON,
    GAME_OVER_BLUE_WON,
    GAME_OVER_DRAW,
    GAME_ABORTED,
} State;

Player* player_init(size_t identifier, Occupation occ);
Game* game_init(size_t identifier);
State before_turn(Game* state);
