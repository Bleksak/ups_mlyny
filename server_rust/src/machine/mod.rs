use std::sync::{Arc, Weak, Mutex, mpsc::Sender};

use crate::server::{message::Message, client::Client, receiver::MessageReceiver};

pub struct Machine {
    state: State,
    client: Weak<Mutex<Client>>,
    receiver: Arc<Mutex<MessageReceiver>>
}

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
                self.receiver.lock().unwrap().create_game(&username, self.client.clone());
                self.state = State::InLobby;
            }
            Message::JOIN(username) => {
                self.state = State::InLobby;
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
    
}
