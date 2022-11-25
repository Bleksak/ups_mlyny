use crate::{server::{client::Client, message::Message}, game::color::Color};

use self::{player::Player, board::Board};
use std::sync::{Mutex, Weak, RwLock};

pub mod player;
pub mod color;
pub mod board;

pub struct Game {
    turn: RwLock<usize>,
    players: Mutex<[Player; 2]>,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        println!("Creating a game");
        Self {
            turn: RwLock::new(0),
            players: Mutex::new([Player::new(Color::Red), Player::new(Color::Blue)]),
            board: Board::new(),
        }
    }
    
    fn notify_join(&self) -> usize {
        for player in self.players.lock().unwrap().iter_mut() {
            if let Some(client) = player.client().upgrade() {
                if let Ok(_) = client.write(&Message::PLAYER_JOINED.serialize()) {
                    println!("player {} notified", player.name().as_ref().unwrap());
                }
            }
        }
        
        self.players.lock().unwrap().iter().filter_map(|p| p.client().upgrade()).count()
    }
    
    pub fn connect(&self, username: &String, client: Weak<Client>) -> Option<usize> {
        // 1. check if username is connected to the game
        // 2. find first empty and join
        println!("Trying to connect {}", username);
        self.try_join(username, client.clone()).or_else(|| self.force_join(username, client))
    }
    
    pub fn try_join(&self, username: &str, client: Weak<Client>) -> Option<usize> {
        for player in self.players.lock().unwrap().iter_mut() {
            if let Some(name) = player.name() {
                if name == username {
                    if player.client().upgrade().is_none() {
                        println!("Connecting {}", username);
                        player.bind(client.clone());
                        let player_count = self.notify_join();
                        return Some(player_count);
                    }
                }
            }
        }
        
        None
    }
    
    pub fn force_join(&self, username: &str, client: Weak<Client>) -> Option<usize> {
        for player in self.players.lock().unwrap().iter_mut() {
            if player.name().is_none() {
                println!("Connecting {}", username);
                player.set_name(username.to_string());
                player.bind(client.clone());
                let player_count = self.notify_join();
                return Some(player_count);
            }
        }
        
        None
    }
    
    pub fn has_player(&self, username: &str) -> bool {
        for player in self.players.lock().unwrap().iter() {
            if let Some(name) = player.name() {
                if name == username {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn get_player(&self, client: Weak<Client>) -> Option<(usize, Color)> {
        self.players.lock().unwrap().iter().enumerate().find(|(_,player)| {
            if let Some(a) = player.client().upgrade() {
                if let Some(b) = client.upgrade() {
                    return a == b;
                }
            }
            
            false
        }).map(|x| (x.0, x.1.color()))
    }
    
    pub fn put(&self, client: Weak<Client>, pos: usize) -> Result<(), String> {
        let (index, color) = self.get_player(client).ok_or("Player is not in the game")?;
        
        if index != *self.turn.read().unwrap() {
            return Err("It's not your turn".to_string());
        }
        
        self.board.put(pos, color)
    }
    
    pub fn mmove(&self, client: Weak<Client>, pos: (usize, usize)) -> Result<(), String> {
        let (index, color) = self.get_player(client).ok_or("Player is not in the game")?;
        
        if index != *self.turn.read().unwrap() {
            return Err("It's not your turn".to_string());
        }
        
        self.board.mmove(pos)
        
    }
    
}