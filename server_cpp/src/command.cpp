#include "command.hpp"
#include "message.hpp"
#include "server.hpp"
#include <iostream>
#include "validator.hpp"
#include "game.hpp"

void Command::player_init_create(Server& server, RecvMessage data) {
    std::string username(data.data().begin(), data.data().end());
    if(!Validator::username(username)) {
        // SEND PACKET THAT USERNAME IS NOT OK
        server.sender().push_message(Message(data.socket(), MessageType::PLAYER_INIT_USERNAME_INVALID, 0, nullptr));
        return;
    }
    
    // 1. find player by username
    // Player* player_name = server.find_player(username);
    
    Game* game = server.games().find([&username](const Game& game) {
        return game.players()[0] == username || game.players()[1] == username;
    });
    
    if(game) {
        // user is already a part of a game
    }
    
    Player* player_name = server.players().find([&username] (const Player& player) {
        return player.name() == username;
    });
    
    if(player_name) {
        if(player_name->socket() == data.socket()) {
            // TODO: client is connected to a game, we should probably ignore this request
            // this is probably the same request as on line 40
            std::cout << "got a request to create game from a client who's connected to a game\n";
            return;
        }
        
        // another user with the same name exists
        std::cout << "username is in use\n";
        // SEND SOCKET THAT USERNAME IS IN USE
        server.sender().push_message(Message(data.socket(), MessageType::PLAYER_INIT_USERNAME_USED, 0, nullptr));
        return;
    }
    // 2. find player by socket
    Player* player_socket = server.players().find([&data] (const Player& player) {
        return player.socket() == data.socket();
    });
    
    if(!player_socket) {
        // something very bad happened, the socket should exist here(because we create player on accept)
        std::cout << "something very bad happened";
        return;
    }
    
    // check if: player is in a game (if yes, terminate it) (otherwise we will create a mem leak + 2 games with the same player):
    if(!player_socket->name().empty()) {
        // we are renaming him, suspicious
        
    }
    
    // happy day scenario:
    
    // init user
    player_socket->set_name(username);
    player_socket->set_color(Color::RED);
    player_socket->reset();
    // create game
    
    Game g(username);
    
    server.games().push_back(std::move(g));
    server.sender().push_message(Message(data.socket(), MessageType::OK, 0, nullptr));
    
    
    // TODO: ITS STUPID THAT PLAYER HOLDS HIS INVENTORY/BOARD, THE GAME SHOULD HOLD HIS INVENTORY
    // PLAYER SHOULD HOLD HIS NAME/SOCKET ONLY
}

void Command::ping(Server& server, RecvMessage data) {
    std::cout << "PONG!" << std::endl;
    server.sender().push_message(Message(data.socket(), MessageType::PONG, 0, nullptr));
}
