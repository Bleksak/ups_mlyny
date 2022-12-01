use std::sync::{Arc, Weak, Mutex};

use crate::server::{message::Message, client::Client, receiver::MessageReceiver};
use crate::game::{Game, player::Player};

#[derive(Debug)]
pub struct Machine {
    state: Mutex<State>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Init = 0,
    InLobby = 1,
    InGamePut = 2,
    InGamePutOpponent = 3,
    InGameTake = 4,
    InGameTakeOpponent = 5,
    InGameMove = 6,
    InGameMoveOpponent = 7,
    GameOver = 8,
}

impl TryInto<State> for u32 {
    type Error = ();
    
    fn try_into(self) -> Result<State, Self::Error> {
        match self {
            0 => Ok(State::Init),
            1 => Ok(State::InLobby),
            2 => Ok(State::InGamePut),
            3 => Ok(State::InGamePutOpponent),
            4 => Ok(State::InGameTake),
            5 => Ok(State::InGameTakeOpponent),
            6 => Ok(State::InGameMove),
            7 => Ok(State::InGameMoveOpponent),
            8 => Ok(State::GameOver),
            _ => Err(()),
        }
    }
}

impl Machine {
    pub fn new(state: State) -> Self {
        Self { state: Mutex::new(state) }
    }
    
    pub fn handle_message(&self, message: Message, receiver: &MessageReceiver, player: Arc<Player>) {
        println!("current state: {:?}", self.state);
        let state = self.state.lock().unwrap().clone();
        
        match state {
            // State::Init => self.handle_init(message, receiver, player),
            State::InLobby => self.handle_in_lobby(message, receiver, player),
            State::InGamePut => self.handle_in_game_put(message, receiver, player),
            State::InGameTake => self.handle_in_game_take(message, receiver, player),
            State::InGameMove => self.handle_in_game_move(message, receiver, player),
            State::GameOver => self.handle_game_over(message, receiver, player),
            _ => { /* ignore opponent's move */ }
        }
    }
    
    pub fn new_client_init(message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        match message {
            Message::Create(username) => {
                if let Some(_) = Self::create_game(username, receiver, client.clone()) {
                    println!("Sucessfully created a game");
                }
            }
            Message::Join(username) => {
                if let Some(_) = Self::join_game(username, receiver, client) {
                    println!("Sucessfully joined a game");
                }
                
            },
            _ => {}
        }
    }
    
    pub fn state(&self) -> State {
        self.state.lock().unwrap().clone()
    }
    
    pub fn set_state(&self, state: State) {
        *self.state.lock().unwrap() = state;
    }
    
    #[allow(unused_variables)]
    fn handle_in_lobby(&self, message: Message, receiver: &MessageReceiver, player: Arc<Player>) {
        // ignore everything
    }
    
    fn handle_in_game_put(&self, message: Message, _: &MessageReceiver, player: Arc<Player>) {
        if let (Message::Put(pos), Some(game)) = (message.clone(), player.game().upgrade()) {
            match game.put(player.clone(), pos) {
                Ok(opponent) => {
                    let opp_state = opponent.machine().state();
                    
                    if let Some(opponent) = opponent.client().upgrade() {
                        if let Ok(_) = opponent.write(&message.serialize()) {
                            println!("opponent notified");
                        }
                        
                        if let Ok(_) = opponent.write(&Message::GameState(opp_state).serialize()) {
                            println!("opponent's state changed");
                        }
                    }
                    
                    if let Some(client) = player.client().upgrade() {
                        if let Ok(_) = client.write(&Message::Mok.serialize()) {
                            println!("clicker notified :D");
                        }
                        
                        if let Ok(_) = client.write(&Message::GameState(self.state()).serialize()) {
                            println!("opponent's state changed");
                        }
                    }
                },
                Err(err) => {
                    if let Some(client)  = player.client().upgrade()  {
                        if let Ok(_) = client.write(&Message::Nok(Some(err.to_string())).serialize()) {
                            println!("sent err to client");
                        }
                    }
                }
            }
        }
    }
    
    fn handle_in_game_take(&self, message: Message, _: &MessageReceiver, player: Arc<Player>) {
        if let Message::Take(pos) = message {
            if let Some(game) = player.game().upgrade() {
                match game.take(player.clone(), pos) {
                    Ok(opponent) => {
                        let opp_state = opponent.machine().state();
                        if opp_state == State::GameOver {
                            println!("game over");
                        }
                    
                        if let Some(opponent) = opponent.client().upgrade() {
                            if let Ok(_) = opponent.write(&message.serialize()) {
                                println!("opponent notified");
                            }
                        
                            if let Ok(_) = opponent.write(&Message::GameState(opp_state.clone()).serialize()) {
                                println!("opponent's state changed");
                            }
                            
                            if opp_state == State::GameOver {
                                println!("game over");
                                if let Ok(_) = opponent.write(&Message::Over.serialize()) {
                                    println!("sent game over");
                                }
                            }
                            
                        }
                    
                        if let Some(client) = player.client().upgrade() {
                            if let Ok(_) = client.write(&Message::Mok.serialize()) {
                                println!("clicker notified :D");
                            }
                        
                            if let Ok(_) = client.write(&Message::GameState(self.state()).serialize()) {
                                println!("opponent's state changed");
                            }
                            if opp_state == State::GameOver {
                                println!("game over");
                                if let Ok(_) = client.write(&Message::Over.serialize()) {
                                    println!("sent game over");
                                }
                            }
                        }
                        
                    },
                    Err(err) => {
                        if let Some(client)  = player.client().upgrade()  {
                            if let Ok(_) = client.write(&Message::Nok(Some(err.to_string())).serialize()) {
                                println!("sent err to client");
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn handle_in_game_move(&self, message: Message, _: &MessageReceiver, player: Arc<Player>) {
        if let Message::Move(from, to) = message {
            if let Some(game) = player.game().upgrade() {
                match game.mmove(player.clone(), (from, to)) {
                    Ok(opponent) => {
                        let opp_state = opponent.machine().state();
                    
                        if let Some(opponent) = opponent.client().upgrade() {
                            if let Ok(_) = opponent.write(&message.serialize()) {
                                println!("opponent notified");
                            }
                        
                            if let Ok(_) = opponent.write(&Message::GameState(opp_state).serialize()) {
                                println!("opponent's state changed");
                            }
                        }
                    
                        if let Some(client) = player.client().upgrade() {
                            if let Ok(_) = client.write(&Message::Mok.serialize()) {
                                println!("clicker notified :D");
                            }
                        
                            if let Ok(_) = client.write(&Message::GameState(self.state()).serialize()) {
                                println!("opponent's state changed");
                            }
                        }
                    },
                    Err(err) => {
                        if let Some(client)  = player.client().upgrade()  {
                            if let Ok(_) = client.write(&Message::Nok(Some(err.to_string())).serialize()) {
                                println!("sent err to client");
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn handle_game_over(&self, _: Message, _: &MessageReceiver, _: Arc<Player>) {
        // TODO: add message: game_over_disconnect
        // delete game from receiver
        // that should delete players as well as they are stored as weak refs
        println!("got msg after game over");
        // self.set_state(State::Init);
    }
    
    pub fn join_game(username: String, receiver: &MessageReceiver, client: Weak<Client>) -> Option<Arc<Game>> {
        // client cannot be in game
        // but username can
        let games = receiver.games();
        
        for game in games.lock().unwrap().iter() {
            if let Some(player) = game.has_player(&username) {
                if let (Some(_), Some(client)) = (player.client().upgrade(), client.upgrade()) {
                    if let Ok(_) = client.write(&Message::Nok(Some("Username is already taken!".to_string())).serialize()) {
                    }
                    return None;
                }
            }
            
            if let Some(_) = game.try_join(&username, receiver, client.clone()) {
                println!("joined with try");
                return Some(game.clone());
            }
        }
        
        for game in games.lock().unwrap().iter() {
            if let Some(_) = game.force_join(&username, receiver, client.clone()) {
                println!("joined with force");
                return Some(game.clone());
            }
        }
        
        if let Some(client) = client.upgrade() {
            if let Ok(_) = client.write(&Message::Nok(Some("There are no open lobbies!".to_string())).serialize()) {
            }
        }
        
        None
    }
    
    pub fn create_game(username: String, receiver: &MessageReceiver, client: Weak<Client>) -> Option<()> {
        // client cannot be in game
        // but username can
        
        let games = receiver.games();
        
        for game in games.lock().unwrap().iter() {
            if let Some(_) = game.has_player(&username) {
                if let Some(client) = client.upgrade() {
                    if let Ok(_) = client.write(&Message::Nok(Some("Username is already taken!".to_string())).serialize()) {
                    }
                }
                return None;
            }
        }
        
        let game = Game::new();
        
        println!("trying to connect player");
        if let Some(_) = game.force_join(&username, receiver, client) {
            games.lock().unwrap().push(game);
            return Some(());
        }
        
        None
    }
}
