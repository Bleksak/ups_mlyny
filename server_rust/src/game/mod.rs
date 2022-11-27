use crate::{server::{client::Client, message::Message, receiver::MessageReceiver}, game::color::Color, machine::State};

use self::{player::Player, board::Board};
use std::sync::{Mutex, Weak, RwLock, Arc};

pub mod player;
pub mod color;
pub mod board;

#[derive(Debug)]
pub struct Game {
    turn: RwLock<usize>,
    players: [Arc<Mutex<Player>>; 2],
    board: Board,
}

pub enum GameError {
    PlayerNotInGame,
    NotYourTurn,
    CannotMove,
    BadPosition,
    FieldTaken,
    InvalidField,
}

impl GameError {
    pub fn to_string(self) -> Option<String> {
        match self {
            GameError::PlayerNotInGame => None,
            GameError::NotYourTurn => Some("It's not your turn".to_string()),
            GameError::CannotMove => Some("Cannot move".to_string()),
            GameError::BadPosition => Some("Invalid position".to_string()),
            GameError::FieldTaken => Some("Field is already taken".to_string()),
            GameError::InvalidField => Some("Invalid field chosen".to_lowercase()),
        }
    }
}

impl Game {
    pub fn new() -> Arc<Self> {
        let s = Arc::new(Self {
            turn: RwLock::new(0),
            players: [Arc::new(Mutex::new(Player::new(Color::Red))), Arc::new(Mutex::new(Player::new(Color::Blue)))],
            board: Board::new(),
        });
        
        for p in s.players.iter() {
            p.lock().unwrap().set_game(Arc::downgrade(&s));
        }
        
        s
    }
    
    fn notify_join(&self) {
        let count = self.players.iter().filter_map(|p| p.lock().unwrap().client().upgrade()).count();
        
        for player in self.players.iter() {
            let lock = player.lock().unwrap();
            if lock.machine().state() == State::InLobby && count == 2 {
                lock.machine().set_state(State::InGamePut);
            }
                
            if let Some(client) = lock.client().upgrade() {
                if let Ok(_) = client.write(&Message::PlayerJoined(lock.machine().state()).serialize()) {
                    println!("player {} notified", lock.name().as_ref().unwrap());
                }
            }
        }
    }
    
    pub fn try_join(&self, username: &str, client: Weak<Client>) -> Option<()> {
        let mut val = None;
        
        for player in self.players.iter() {
            let mut guard = player.lock().unwrap();
            
            if let Some(name) = guard.name() {
                if name == username {
                    if guard.client().upgrade().is_none() {
                        println!("Connecting {}", username);
                        guard.bind(client.clone());
                        println!("bound");
                        val = Some(());
                        break;
                    }
                }
            }
        }
        
        println!("notifying");
        if let Some(_) = val {
            self.notify_join();
        }
        
        val
    }
    
    pub fn force_join(&self, username: &str, receiver: &MessageReceiver, client: Weak<Client>) -> Option<()> {
        let mut val = None;
        
        for player in self.players.iter() {
            let mut guard = player.lock().unwrap();
            
            if guard.name().is_none() {
                if let Some(client) = client.upgrade() {
                    println!("Connecting {}", username);
                    guard.set_name(username.to_string());
                    guard.bind(Arc::downgrade(&client));
                    let mut plock = receiver.players().lock().unwrap();
                    let entry = plock.entry(client.sock_fd());
                    entry.or_insert(Arc::downgrade(player));
                    val = Some(());
                    break;
                }
            }
        };
        
        println!("notifying");
        if let Some(_) = val {
            self.notify_join();
       }
       
       val
    }
    
    pub fn has_player(&self, username: &str) -> bool {
        for player in self.players.iter() {
            if let Some(name) = player.lock().unwrap().name() {
                if name == username {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn get_player(&self, client: Weak<Client>) -> Option<(usize, Color)> {
        self.players.iter().enumerate().find(|(_,player)| {
            if let (Some(a), Some(b)) = (player.lock().unwrap().client().upgrade(), client.upgrade()) {
                return a == b;
            }
            
            false
        }).map(|x| (x.0, x.1.lock().unwrap().color()))
    }
    
    pub fn put(&self, client: Weak<Client>, pos: usize) -> Result<Weak<Client>, GameError> {
        let (index, color) = self.get_player(client).ok_or(GameError::PlayerNotInGame)?;
        
        if index != *self.turn.read().unwrap() {
            return Err(GameError::NotYourTurn);
        }
        
        let opponent_index = (index + 1) % 2;
        
        self.board.put(pos, color)?;
        // Check for mill
        
        let mut turn = self.turn.write().unwrap();
        *turn = (*turn + 1) % 2;
        
        Ok(self.players.get(opponent_index).unwrap().lock().unwrap().client().to_owned())
    }
    
    pub fn mmove(&self, client: Weak<Client>, pos: (usize, usize)) -> Result<Weak<Client>, GameError> {
        let (index, color) = self.get_player(client).ok_or(GameError::PlayerNotInGame)?;
        
        if index != *self.turn.read().unwrap() {
            return Err(GameError::NotYourTurn);
        }
        
        if color != self.board.get(pos.0)? || Color::Neutral != self.board.get(pos.1)? {
            return Err(GameError::CannotMove);
        }
        
        let opponent_index = (index + 1) % 2;
        
        self.board.mmove(pos)?;
        
        let mut turn = self.turn.write().unwrap();
        *turn = (*turn + 1) % 2;
        
        Ok(self.players.get(opponent_index).unwrap().lock().unwrap().client().clone())
    }
    
    pub fn take(&self, client: Weak<Client>, pos: usize) -> Result<Weak<Client>, GameError> {
        let (index, _) = self.get_player(client).ok_or(GameError::PlayerNotInGame)?;
        
        if index != *self.turn.read().unwrap() {
            return Err(GameError::NotYourTurn);
        }
        
        let opponent_index = (index + 1) % 2;
        let players_guard = &self.players;
        let opponent = players_guard.get(opponent_index).unwrap();
        
        if self.board.get(pos)? != opponent.lock().unwrap().color() {
            return Err(GameError::InvalidField);
        }
        
        self.board.take(pos)?;
        
        let mut turn = self.turn.write().unwrap();
        *turn = (*turn + 1) % 2;
        
        Ok(opponent.lock().unwrap().client().clone())
    }
}
