#include <stdlib.h>
#include <stdbool.h>

#include "game.h"

#define BOARD_SIZE 24

// const int default_node_positions_x[] = {0, 3, 6, 1, 3, 5, 3, 4, 5, 0, 1, 2, 4, 5, 6, 3, 4, 5, 1, 3, 5, 0, 3, 6};
// const int default_node_positions_y[] = {0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6};

const static size_t neighbors_count[] = {2, 3, 2, 2, 4, 2, 2, 3, 2, 3, 4, 3, 3, 4, 3, 2, 3, 2, 2, 4, 2, 2, 3, 2};

const static size_t neighbors[][BOARD_SIZE] = {
    {1, 6}, // 0 
    {0, 2, 4}, // 1
    {1, 14}, // 2
    {4, 10}, // 3
    {1, 3, 5, 7}, // 4
    {4, 13}, // 5
    {7, 11}, // 6
    {4, 6, 8}, // 7
    {7, 12}, // 8
    {0, 10, 21}, // 9
    {3, 9, 11, 18}, // 10
    {6, 10, 15}, // 11
    {8, 13, 17}, // 12
    {5, 12, 14, 20}, // 13
    {2, 13, 23}, // 14
    {11, 16}, // 15
    {15, 17, 19}, // 16
    {12, 16}, // 17
    {10, 19}, // 18
    {13, 18, 20, 22}, // 19
    {13, 19}, //20
    {9, 22}, // 21
    {19, 21, 23}, // 22
    {14, 22} // 23
};

size_t* get_neighbors(size_t index, size_t* count) {
    *count = neighbors_count[index];
    return neighbors[index];
}

int occupy(const GameState* state, Player* player, size_t index) {
    if(player != &state->players[state->current_player]) {
        // TODO: RETURN PROPER ERROR ENUM
        return -1;
    }

    if(index >= BOARD_SIZE) {
        // TODO: RETURN PROPER ERROR ENUM
        return -1;
    }

    if(!player->inventory) {
        // TODO: RETURN PROPER ERROR ENUM
        return -1;
    }

    state->board[index] = player->occupation;
    player->board += 1;
    player->inventory -= 1;

    return 0;
}

static bool is_neighbor(size_t a, size_t b) {
    size_t neighbor_count;
    size_t* neigh = get_neighbors(a, &neighbor_count);

    for(size_t i = 0; i < neighbor_count; ++i) {
        if(neigh[i] == b) {
            return true;
        }
    }

    return false;
}

int move(const GameState* state, Player* player, size_t start_index, size_t end_index) {

    // CHECK IF ITS PLAYERS TURN
    if(player != &state->players[state->current_player]) {
        // TODO: RETURN PROPER ERROR ENUM
        return -1;
    }

    // CHECK IF START POSITION CONTAINS A BLOCK OF PLAYER
    if(state->board[start_index] != player->occupation) {
        // TODO: RETURN PROPER ERROR ENUM
        return -1;
    }

    // CHECK IF END POSITION IS EMPTY
    if(state->board[end_index] != None) {
        // TODO: RETURN PROPER ERROR ENUM
        return -1;
    }

    // CHECK IF PLAYER CAN JUMP (board + inventory == 3)
    if(player->board + player->inventory == 3) {
        state->board[start_index] = None;
        state->board[end_index] = player->occupation;
        return 0;
    } 
    
    // CHECK IF END POSITION IS NEIGHBOR OF START POSITION
    if(!is_neighbor(start_index, end_index)) {
        // TODO: RETURN PROPER ERROR ENUM
        return -1;
    }

    state->board[start_index] = None;
    state->board[end_index] = player->occupation;

    return 0;
}

Occupation* make_board(void) {
    return calloc(BOARD_SIZE, sizeof(Player));
}