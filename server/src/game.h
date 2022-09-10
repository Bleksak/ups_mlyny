#pragma once

typedef enum Occupation {
    None,
    Red,
    Blue,
} Occupation;

typedef struct Player {
    Occupation occupation;
    size_t board; // how many stones are on board
    size_t inventory; // how many stones are in inventory
} Player;

typedef struct GameState {
    size_t current_player;
    Player* players;
    Occupation* board;
} GameState;

typedef enum State {
    GAME_RUNNING,
    GAME_OVER_RED_WON,
    GAME_OVER_BLUE_WON,
    GAME_OVER_DRAW,
    GAME_ABORTED,
} State;

GameState* game_init(void);
State before_turn(GameState* state);
