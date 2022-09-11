#pragma once

#include "game.h"

Occupation* make_board(void);
int move(const GameState* state, Player* player, size_t start_index, size_t end_index);
int occupy(const GameState* state, Player* player, size_t index);
