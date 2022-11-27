use crate::{server::{client::Client, message::Message, receiver::MessageReceiver}, game::color::Color, machine::State};

use self::{player::Player, board::Board};
use std::sync::{Weak, RwLock, Arc};

pub mod player;
pub mod color;
pub mod board;

#[derive(Debug)]
pub struct Game {
    turn: RwLock<usize>,
    players: [Arc<Player>; 2],
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
            players: [Arc::new(Player::new(Color::Red)), Arc::new(Player::new(Color::Blue))],
            board: Board::new(),
        });
        
        for p in s.players.iter() {
            p.set_game(Arc::downgrade(&s));
        }
        
        s
    }
    
    fn notify_join(&self) {
        let count = self.players.iter().filter_map(|p| p.client().upgrade()).count();
        
        for player in self.players.iter() {
            if player.machine().state() == State::InLobby && count == 2 {
                player.machine().set_state(State::InGamePut);
            }
                
            if let Some(client) = player.client().upgrade() {
                if let Ok(_) = client.write(&Message::PlayerJoined(player.machine().state(), player.color(), self.board.serialize()).serialize()) {
                    println!("player {} notified", player.name().as_ref().unwrap());
                }
            }
        }
    }
    
    pub fn try_join(&self, username: &str, client: Weak<Client>) -> Option<()> {
        let mut val = None;
        
        for player in self.players.iter() {
            if let Some(name) = player.name() {
                if name == username {
                    if player.client().upgrade().is_none() {
                        println!("Connecting {}", username);
                        player.bind(client.clone());
                        println!("bound");
                        val = Some(());
                        break;
                    }
                }
            }
        }
        
        if let Some(_) = val {
            self.notify_join();
        }
        
        val
    }
    
    pub fn force_join(&self, username: &str, receiver: &MessageReceiver, client: Weak<Client>) -> Option<()> {
        let mut val = None;
        
        for player in self.players.iter() {
            if player.name().is_none() {
                if let Some(client) = client.upgrade() {
                    println!("Connecting {}", username);
                    player.set_name(username.to_string());
                    player.bind(Arc::downgrade(&client));
                    
                    let mut plock = receiver.players().lock().unwrap();
                    let entry = plock.entry(client.sock_fd());
                    entry.or_insert(Arc::downgrade(player));
                    
                    val = Some(());
                    break;
                }
            }
        };
        
        if let Some(_) = val {
            self.notify_join();
       }
       
       val
    }
    
    pub fn has_player(&self, username: &str) -> bool {
        for player in self.players.iter() {
            if let Some(name) = player.name() {
                if name == username {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn put(&self, player: Arc<Player>, pos: usize) -> Result<Weak<Client>, GameError> {
        let turn = *self.turn.read().unwrap();
        let player_turn = self.players[turn].clone();
        
        if player != player_turn {
            return Err(GameError::NotYourTurn);
        }
        
        let opponent_index = (turn + 1) % 2;
        
        self.board.put(pos, player.color())?;
        // Check for mill
        
        let mut turn = self.turn.write().unwrap();
        *turn = (*turn + 1) % 2;
        
        Ok(self.players.get(opponent_index).unwrap().client().to_owned())
    }
    
    pub fn mmove(&self, player: Arc<Player>, pos: (usize, usize)) -> Result<Weak<Client>, GameError> {
        let turn = *self.turn.read().unwrap();
        let player_turn = self.players[turn].clone();
        
        if player != player_turn {
            return Err(GameError::NotYourTurn);
        }
        
        if player.color() != self.board.get(pos.0)? || Color::Neutral != self.board.get(pos.1)? {
            return Err(GameError::CannotMove);
        }
        
        let opponent_index = (turn + 1) % 2;
        
        self.board.mmove(pos)?;
        
        let mut turn = self.turn.write().unwrap();
        *turn = (*turn + 1) % 2;
        
        Ok(self.players.get(opponent_index).unwrap().client().clone())
    }
    
    pub fn take(&self, player: Arc<Player>, pos: usize) -> Result<Weak<Client>, GameError> {
        let turn = *self.turn.read().unwrap();
        let player_turn = self.players[turn].clone();
        
        if player != player_turn {
            return Err(GameError::NotYourTurn);
        }
        
        let opponent_index = (turn + 1) % 2;
        let players_guard = &self.players;
        let opponent = players_guard.get(opponent_index).unwrap();
        
        if self.board.get(pos)? != opponent.color() {
            return Err(GameError::InvalidField);
        }
        
        self.board.take(pos)?;
        
        let mut turn = self.turn.write().unwrap();
        *turn = (*turn + 1) % 2;
        
        Ok(opponent.client().clone())
    }
}
