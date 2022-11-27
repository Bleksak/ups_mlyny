use std::sync::{Arc, Weak};

use crate::{server::client::Client, machine::Machine};

use super::{color::Color, Game};

#[derive(Debug)]
pub struct Player {
    username: Option<String>,
    client: Weak<Client>,
    color: Color,
    inventory_cnt: usize,
    board_cnt: usize,
    machine: Arc<Machine>,
    game: Weak<Game>
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            username: None,
            client: Weak::new(),
            color,
            inventory_cnt: 9,
            board_cnt: 0,
            machine: Arc::new(Machine::new()),
            game: Weak::new(),
        }
    }
    
    pub fn machine(&self) -> Arc<Machine> {
        self.machine.clone()
    }
    
    pub fn game(&self) -> &Weak<Game> {
        &self.game
    }
    pub fn set_game(&mut self, game: Weak<Game>) {
        self.game = game;
    }
    
    pub fn name(&self) -> &Option<String> {
        &self.username
    }
    
    pub fn set_name(&mut self, username: String) {
        self.username = Some(username);
    }
    
    pub fn bind(&mut self, client: Weak<Client>) {
        self.client = client;
    }
    
    pub fn client(&self) -> Weak<Client> {
        self.client.clone()
    }
    
    pub fn color(&self) -> Color {
        self.color
    }
    
    pub fn put(&mut self) -> bool {
        if self.inventory_cnt <= 0 {
            return false;
        }
        
        self.board_cnt += 1;
        self.inventory_cnt -= 1;
        
        return true;
    }
    
    pub fn take(&mut self) -> bool {
        if self.board_cnt <= 0 {
            return false;
        }
        
        self.board_cnt -= 1;
        
        return true;
    }
    
}
