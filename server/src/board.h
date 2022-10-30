#pragma once

#include "game.h"

Occupation* make_board(void);
int move(const Game* state, Player* player, size_t start_index, size_t end_index);
int occupy(const Game* state, Player* player, size_t index);
