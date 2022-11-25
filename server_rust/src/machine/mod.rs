use std::sync::{Arc, Weak, Mutex};

use crate::server::{message::Message, client::Client, receiver::MessageReceiver};
use crate::game::Game;

pub struct Machine {
    state: Mutex<State>,
}

#[derive(Debug, Clone)]
enum State {
    Init,
    InLobby,
    InGamePut,
    InGameTake,
    InGameMove,
    GameOver,
}
//smrdis hrutko
impl Machine {
    pub fn new() -> Self {
        Self { state: Mutex::new(State::Init) }
    }
    
    pub fn handle_message(&mut self, message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        println!("current state: {:?}", self.state);
        let state = self.state.lock().unwrap().clone();
        
        match state {
            State::Init => self.handle_init(message, receiver, client),
            State::InLobby => self.handle_in_lobby(message, receiver, client),
            State::InGamePut => self.handle_in_game_put(message, receiver, client),
            State::InGameTake => self.handle_in_game_take(message, receiver, client),
            State::InGameMove => self.handle_in_game_move(message, receiver, client),
            State::GameOver => self.handle_game_over(message, receiver, client),
        }
    }
    
    fn handle_init(&mut self, message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        match message {
            // check if client is connected with player
            Message::CREATE(username) => {
                println!("Player {} wants to create a game", username);
                if let Some(_) = self.create_game(username, receiver, client) {
                    *self.state.lock().unwrap() = State::InLobby;
                }
            }
            Message::JOIN(username) => {
                println!("Player {} wants to join a game", username);
                if let Some((game, n)) = self.join_game(username, receiver, client) {
                    if n == 2 {
                        *self.state.lock().unwrap() = State::InGamePut;
                        // for player in game.lock().unwrap().players_mut().iter_mut().filter(|player| player.client() !=) {
                        //     println!("Running game for player {}", player.name().as_ref().unwrap());
                            
                        //     player.client().upgrade().unwrap().lock().unwrap().machine().lock().unwrap().set_state(State::InGamePut);
                        // }
                    } else {
                        *self.state.lock().unwrap() = State::InLobby;
                    }
                }
            },
            _ => {}
        }
    }
    
    fn set_state(&mut self, state: State) {
        *self.state.lock().unwrap() = state;
    }
    
    #[allow(unused_variables)]
    fn handle_in_lobby(&mut self, message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        // ignore everything
    }
    
    fn handle_in_game_put(&mut self, message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        
    }
    
    fn handle_in_game_take(&mut self, message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        
    }
    
    fn handle_in_game_move(&mut self, message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        
    }
    
    fn handle_game_over(&mut self, message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        
    }
    
    pub fn join_game(&mut self, username: String, receiver: &MessageReceiver, client: Weak<Client>) -> Option<(Arc<Game>, usize)> {
        // client cannot be in game
        // but username can
        let games = receiver.games();
        for game in games.lock().unwrap().iter() {
            if let Some(n) = game.try_join(&username, client.clone()) {
                return Some((game.clone(), n));
            }
        }
        
        for game in games.lock().unwrap().iter() {
            if let Some(n) = game.force_join(&username, client.clone()) {
                return Some((game.clone(), n));
            }
        }
        
        None
    }
    
    // TODO: check if user is in game.. if user is disconnected
    
    pub fn create_game(&mut self, username: String, receiver: &MessageReceiver, client: Weak<Client>) -> Option<()> {
        // client cannot be in game
        // but username can
        
        let games = receiver.games();
        
        for game in games.lock().unwrap().iter() {
            if game.has_player(&username) {
                return None;
            }
        }
        
        let game = Game::new();
        
        if let Some(_) = game.connect(&username, client) {
            games.lock().unwrap().push(Arc::new(game));
            return Some(());
        }
        
        None
    }
}
