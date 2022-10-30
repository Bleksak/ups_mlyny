#include "server_messages.h"
#include "game.h"
#include <sys/types.h>
#include <sys/socket.h>
#include <unistd.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdbool.h>

static char* msg_strings[] = {
    "INVALID MESSAGE",
    "CREATE A GAME",
    "PUT STONE",
    "MOVE STONE",
};

typedef void (*handlers)(RequestData* data);

void game_create_handler(RequestData* data) {
    puts("creating a game");
    
    Game* game = game_init(data->app->game_identifier++);
    size_t new_count = data->app->game_count + 1;
    data->app->games = realloc(data->app->games, sizeof(Game*) * new_count);
    data->app->games[new_count - 1] = game;
    data->app->game_count = new_count;
    
    game->players[0].socket = data->client_socket;
    game->players[0].identifier = data->app->player_identifier++;
    
    int32_t msg = OK;
    send(data->client_socket, &msg, sizeof(msg), 0);
    send(data->client_socket, &game->identifier, sizeof(game->identifier), 0);
    send(data->client_socket, &game->players[0].identifier, sizeof(game->players[0].identifier), 0);
}

void game_join_handler(RequestData* data) {
    // hledame hru podle game id
    // pokud mame i player id =>
    // zjistime jestli je v ni i player (podle id)
    // zjistime jestli je disconnected (socket = 0)
    // pokud ano => povolime join, posleme notifikaci zpet + protihraci
    
   // input => game identifier, player identifier
   size_t game_identifier, player_identifier;
   
   ssize_t read = recv(data->client_socket, &game_identifier, sizeof(game_identifier), 0);
   
   if(read != sizeof(game_identifier)) {
       // invalid arg count
   }
   
   Game* game = NULL;
   for(size_t i = 0; i < data->app->game_count; ++i) {
       if(data->app->games[i]->identifier == game_identifier) {
           game = data->app->games[i];
           break;
       }
   }
   
   if(game == NULL) {
       // game not found
       int result = FAIL;
       send(data->client_socket, &result, sizeof(result), 0);
       return;
   }
   
   if(recv(data->client_socket, &player_identifier, sizeof(player_identifier), 0) == sizeof(player_identifier)) {
    if(game->players[0].identifier == player_identifier && game->players[0].socket == 0) {
       // OK player == p0
       game->players[0].socket = data->client_socket;
       // send OK + notify other player (p[1] (if he exists))
       int result = OK;
       send(data->client_socket, &result, sizeof(result), 0);
       
       if(game->players[1].socket) {
           result = GAME_OPPONENT_JOINED;
           send(game->players[1].socket, &result, sizeof(result), 0);
       }
       
       return;
    }
   
    if(game->players[1].identifier == player_identifier && game->players[1].socket == 0) {
       game->players[1].socket = data->client_socket;
       // OK player == p1
       
       // send OK + notify other player (p[1] (if he exists))
       int result = OK;
       send(data->client_socket, &result, sizeof(result), 0);
       
       if(game->players[0].socket) {
           result = GAME_OPPONENT_JOINED;
           send(game->players[0].socket, &result, sizeof(result), 0);
       }
       
       return;
    }
   }
   
   if(game->players[1].identifier == 0 && game->players[1].socket == 0) {
       game->players[1].socket = data->client_socket;
       // OK player == p1
       
       // send OK + notify other player (p[1] (if he exists))
       int result = OK;
       send(data->client_socket, &result, sizeof(result), 0);
       
       if(game->players[0].socket) {
           result = GAME_OPPONENT_JOINED;
           send(game->players[0].socket, &result, sizeof(result), 0);
       }
       
        return;
   }
   
   // ERR player not found in the game
   int result = FAIL;
   send(data->client_socket, &result, sizeof(result), 0);
}


static handlers msg_handlers[] = {
    NULL,
    game_create_handler,
    game_join_handler,
};

static void handle_message(RequestData* data, ServerMessage msg) {
    if(msg >= SERVER_MESSAGES_COUNT || msg == MSG_INVALID) {
        return;
    }
    
    msg_handlers[msg](data);
}

void* serve_request(void* arg) {
    RequestData* data = (RequestData*) arg;
    uint32_t msg_type;
    
    while(true) {
        ssize_t bytes_read = recv(data->client_socket, &msg_type, sizeof(msg_type), 0);
        if(bytes_read != sizeof(msg_type)) {
            continue;
        }
        
        handle_message(data, msg_type);
    }
    
    close(data->client_socket);
    free(data);
    return 0;
}

char* get_msg_str(ServerMessage msg) {
    if(msg >= SERVER_MESSAGES_COUNT) {
        return msg_strings[MSG_INVALID];
    }
    
    return msg_strings[msg];
}
