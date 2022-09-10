#include "game.h"
#include "board.h"

#include <stdbool.h>
#include <stdlib.h>

GameState* game_init(void) {
    GameState* state = calloc(1, sizeof(GameState));

    state->board = make_board();
    state->players = calloc(2, sizeof(Player));
    state->players[0].inventory = 9;
    state->players[0].occupation = Red;

    state->players[1].inventory = 9;
    state->players[1].occupation = Blue;

    return state;
}

State before_turn(GameState* state) {
    // TODO: send current game state to the client
    // TODO: check if player has lost
    Player* current_player = &state->players[state->current_player];
    if(current_player->inventory + current_player->board < 3) {
        // player has lost
    }

    return GAME_RUNNING;
}
