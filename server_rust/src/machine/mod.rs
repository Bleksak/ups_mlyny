use std::sync::{Arc, Weak, Mutex, mpsc::Sender};

use crate::server::{message::Message, client::Client, receiver::MessageReceiver};
use crate::game::Game;

pub struct Machine {
    state: State,
    client: Weak<Mutex<Client>>,
    receiver: Arc<Mutex<MessageReceiver>>
}

#[derive(Debug)]
enum State {
    Init,
    InLobby,
    InGamePut,
    InGameTake,
    InGameMove,
    GameOver,
}

impl Machine {
    pub fn new(client: Weak<Mutex<Client>>, receiver: Arc<Mutex<MessageReceiver>>)-> Self {
        Self { state: State::Init, client, receiver }
    }
    
    pub fn handle_message(&mut self, message: Message) {
        println!("current state: {:?}", self.state);
        
        match self.state {
            State::Init => self.handle_init(message),
            State::InLobby => self.handle_in_lobby(message),
            State::InGamePut => self.handle_in_game_put(message),
            State::InGameTake => self.handle_in_game_take(message),
            State::InGameMove => self.handle_in_game_move(message),
            State::GameOver => self.handle_game_over(message),
        }
    }
    
    fn handle_init(&mut self, message: Message) {
        match message {
            // check if client is connected with player
            Message::CREATE(username) => {
                println!("Player {} wants to create a game", username);
                if let Some(_) = self.create_game(username, self.client.clone()) {
                    self.state = State::InLobby;
                }
            }
            Message::JOIN(username) => {
                println!("Player {} wants to join a game", username);
                if let Some(_) = self.join_game(username, self.client.clone()) {
                    self.state = State::InLobby;
                }
            },
            _ => {}
        }
    }
    
    fn handle_in_lobby(&mut self, message: Message) {
        
    }
    
    fn handle_in_game_put(&mut self, message: Message) {
        
    }
    
    fn handle_in_game_take(&mut self, message: Message) {
        
    }
    
    fn handle_in_game_move(&mut self, message: Message) {
        
    }
    
    fn handle_game_over(&mut self, message: Message) {
        
    }
    
    pub fn join_game(&mut self, username: String, client: Weak<Mutex<Client>>) -> Option<()> {
        // client cannot be in game
        // but username can
        println!("acquiring lock for receiver");
        let games = self.receiver.lock().unwrap().games();
        println!("done receiver");
        
        for game in games.lock().unwrap().iter() {
            println!("acquiring lock for game");
            let mut g = game.lock().unwrap();
            if let Some(player) = g.player(&username) {
                // player is in game, is he connected?
                println!("found player?");
                if let Some(_) = player.client().upgrade() {
                    // he's connected(upgraded succesfully), we dont allow it
                    println!("what the dog doin");
                    return None;
                } else {
                    // TODO: try to connect here
                    if g.connect(&username, client) {
                        return Some(());
                    }
                    
                    return None;
                }
            }
        }
        
        println!("first loop over");
        
        for game in games.lock().unwrap().iter() {
            if game.lock().unwrap().connect(&username, client.clone()) {
                return Some(());
            }
        }
        
        println!("second loop over");
        
        None
    }
    
    // TODO: check if user is in game.. if user is disconnected
    
    pub fn create_game(&mut self, username: String, client: Weak<Mutex<Client>>) -> Option<()> {
        // client cannot be in game
        // but username can
        
        let games = self.receiver.lock().unwrap().games();
        
        for game in games.lock().unwrap().iter() {
            if let Some(_) = game.lock().unwrap().player(&username) {
                // player is in game, we dont allow it
                return None;
            }
        }
        
        let mut game = Game::new();
        
        if game.connect(&username, client) {
            games.lock().unwrap().push(Arc::new(Mutex::new(game)));
            return Some(());
        }
        
        None
    }
    
}
