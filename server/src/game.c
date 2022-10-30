#include "game.h"

#include "app.h"
#include "board.h"

#include <stdbool.h>
#include <stdlib.h>

Player* player_init(size_t identifier, Occupation occ) {
    Player* p = calloc(1, sizeof(Player));
    
    p->occupation = occ;
    p->identifier = identifier;
    p->inventory = 9;
    p->board = 0;
    
    return p;
}

Game* game_init(size_t identifier) {
    Game* state = calloc(1, sizeof(Game));

    state->board = make_board();
    state->players[0].inventory = 9;
    state->players[0].occupation = RED;

    state->players[1].inventory = 9;
    state->players[1].occupation = BLUE;
    
    state->identifier = identifier;
    
    return state;
}

State before_turn(Game* state) {
    // TODO: send current game state to the client
    // TODO: check if player has lost
    Player* current_player = &state->players[state->current_player];
    if(current_player->inventory + current_player->board < 3) {
        // player has lost
    }

    return GAME_RUNNING;
}
