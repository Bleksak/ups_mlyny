#pragma once

#include "message.hpp"
#include <iostream>
#include <vector>

class Server;

class Command {
    public:
        static void player_init_create(Server& server, RecvMessage data);
        static void ping(Server& server, RecvMessage data);
};

