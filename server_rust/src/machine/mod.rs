use std::sync::{Arc, Weak, Mutex};

use crate::server::{message::Message, client::Client, receiver::MessageReceiver};
use crate::game::{self, Game, player::Player};

#[derive(Debug)]
pub struct Machine {
    state: Mutex<State>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Init = 0,
    InLobby = 1,
    InGamePut = 2,
    InGameTake = 3,
    InGameMove = 4,
    GameOver = 5,
}

impl TryInto<State> for u32 {
    type Error = ();
    
    fn try_into(self) -> Result<State, Self::Error> {
        match self {
            0 => Ok(State::Init),
            1 => Ok(State::InLobby),
            2 => Ok(State::InGamePut),
            3 => Ok(State::InGameTake),
            4 => Ok(State::InGameMove),
            5 => Ok(State::GameOver),
            _ => Err(()),
        }
    }
}

//smrdis hrutko
impl Machine {
    pub fn new() -> Self {
        Self { state: Mutex::new(State::InLobby) }
    }
    
    pub fn handle_message(&self, message: Message, receiver: &MessageReceiver, player: Arc<Player>) {
        println!("current state: {:?}", self.state);
        let state = self.state.lock().unwrap().clone();
        
        match state {
            State::Init => self.handle_init(message, receiver, player),
            State::InLobby => self.handle_in_lobby(message, receiver, player),
            State::InGamePut => self.handle_in_game_put(message, receiver, player),
            State::InGameTake => self.handle_in_game_take(message, receiver, player),
            State::InGameMove => self.handle_in_game_move(message, receiver, player),
            State::GameOver => self.handle_game_over(message, receiver, player),
        }
    }
    
    pub fn new_client_init(message: Message, receiver: &MessageReceiver, client: Weak<Client>) {
        match message {
            Message::Create(username) => {
                if let Some(_) = Self::create_game(username, receiver, client) {
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
    
    fn handle_init(&self, message: Message, receiver: &MessageReceiver, player: Arc<Player>) {
        match message {
            // check if client is connected with player
            // Message::Create(username) => {
            //     println!("Player {} wants to create a game", username);
            //     if let Some(_) = Self::create_game(username, receiver, player.client()) {
                    
            //     }
            // }
            // Message::Join(username) => {
            //     println!("Player {} wants to join a game", username);
            //     if let Some(_) = Self::join_game(username, receiver, player.client()) {
            //     }
            // },
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
    
    fn handle_in_game_put(&self, message: Message, receiver: &MessageReceiver, player: Arc<Player>) {
        if let (Message::Put(pos), Some(game)) = (message.clone(), player.game().upgrade()) {
            match game.put(player.clone(), pos) {
                Ok(opponent) => {
                    if let Some(opponent) = opponent.upgrade() {
                        if let Ok(_) = opponent.write(&message.serialize()) {
                            println!("opponent notified");
                        }
                    }
                    
                    if let Some(client) = player.client().upgrade() {
                        if let Ok(_) = client.write(&Message::Mok.serialize()) {
                            println!("clicker notified :D");
                        }
                    }
                },
                Err(err) => {
                    if let (Some(msg), Some(client))  = (err.to_string(), player.client().upgrade())  {
                        if let Ok(_) = client.write(&Message::Nok(Some(msg)).serialize()) {
                            println!("sent err to client");
                        }
                    }
                }
            }
        }
    }
    
    fn handle_in_game_take(&self, message: Message, receiver: &MessageReceiver, player: Arc<Player>) {
        if let Message::Take(pos) = message {
        }
    }
    
    fn handle_in_game_move(&self, message: Message, receiver: &MessageReceiver, client: Arc<Player>) {
        if let Message::Move(from, to) = message {
        }
    }
    
    fn handle_game_over(&self, message: Message, receiver: &MessageReceiver, client: Arc<Player>) {
        // TODO: add message: game_over_disconnect
        // delete game from receiver
        // that should delete players as well as they are stored as weak refs
        self.set_state(State::Init);
    }
    
    pub fn join_game(username: String, receiver: &MessageReceiver, client: Weak<Client>) -> Option<Arc<Game>> {
        // client cannot be in game
        // but username can
        let games = receiver.games();
        for game in games.lock().unwrap().iter() {
            if let Some(_) = game.try_join(&username, client.clone()) {
                println!("connected");
                return Some(game.clone());
            }
        }
        
        for game in games.lock().unwrap().iter() {
            if let Some(_) = game.force_join(&username, receiver, client.clone()) {
                return Some(game.clone());
            }
        }
        
        None
    }
    
    pub fn create_game(username: String, receiver: &MessageReceiver, client: Weak<Client>) -> Option<()> {
        // client cannot be in game
        // but username can
        
        let games = receiver.games();
        
        for game in games.lock().unwrap().iter() {
            if game.has_player(&username) {
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
