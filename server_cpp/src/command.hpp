#pragma once

#include "message.hpp"
#include <iostream>
#include <vector>

class Server;

class Command {
    public:
        virtual ~Command() = 0;
        static void player_init(Server& server, RecvMessage data);
        static void ping(Server& server, RecvMessage data);
};

