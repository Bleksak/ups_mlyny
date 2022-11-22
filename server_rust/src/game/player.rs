use std::sync::{Mutex, Weak};

use crate::server::client::Client;

use super::Game;

pub struct Player {
    username: Option<String>,
    game: Weak<Mutex<Game>>,
    client: Weak<Mutex<Client>>
}

impl Player {
    pub fn new() -> Self {
        Self {
            username: None,
            game: Weak::new(),
            client: Weak::new()
        }
    }
    
    pub fn name(&self) -> &Option<String> {
        &self.username
    }
    
    pub fn set_name(&mut self, username: String) {
        self.username = Some(username);
    }
    
    pub fn set_game(&mut self, game: Weak<Mutex<Game>>) {
        self.game = game;
    }
    
    pub fn bind(&mut self, client: Weak<Mutex<Client>>) {
        self.client = client;
    }
    
    pub fn client(&self) -> &Weak<Mutex<Client>> {
        &self.client
    }
    
    pub fn game(&self) -> &Weak<Mutex<Game>> {
        &self.game
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}
