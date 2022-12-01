use std::sync::{Arc, Weak, Mutex};

use crate::{server::client::Client, machine::Machine};

use super::{color::Color, Game};

#[derive(Debug)]
pub struct Player {
    username: Mutex<Option<String>>,
    client: Mutex<Weak<Client>>,
    color: Color,
    inventory_cnt: Mutex<usize>,
    board_cnt: Mutex<usize>,
    machine: Arc<Machine>,
    game: Mutex<Weak<Game>>
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            username: Mutex::new(None),
            client: Mutex::new(Weak::new()),
            color,
            inventory_cnt: Mutex::new(9),
            board_cnt: Mutex::new(0),
            machine: Arc::new(Machine::new()),
            game: Mutex::new(Weak::new()),
        }
    }
    
    pub fn machine(&self) -> Arc<Machine> {
        self.machine.clone()
    }
    
    pub fn game(&self) -> Weak<Game> {
        self.game.lock().unwrap().clone()
    }
    pub fn set_game(&self, game: Weak<Game>) {
        *self.game.lock().unwrap() = game;
    }
    
    pub fn name(&self) -> Option<String> {
        self.username.lock().unwrap().clone()
    }
    
    pub fn set_name(&self, username: String) {
        *self.username.lock().unwrap() = Some(username);
    }
    
    pub fn bind(&self, client: Weak<Client>) {
        *self.client.lock().unwrap() = client;
    }
    
    pub fn client(&self) -> Weak<Client> {
        self.client.lock().unwrap().clone()
    }
    
    pub fn color(&self) -> Color {
        self.color
    }
    
    pub fn inventory(&self) -> usize {
        *self.inventory_cnt.lock().unwrap()
    }
    
    pub fn board(&self) -> usize {
        *self.board_cnt.lock().unwrap()
    }
    
    pub fn put(&self) -> bool {
        
        let mut inv_cnt = self.inventory_cnt.lock().unwrap();
        let mut board_cnt = self.board_cnt.lock().unwrap();
        
        if *inv_cnt <= 0 {
            return false;
        }
        
        *inv_cnt -= 1;
        *board_cnt += 1;
        
        return true;
    }
    
    pub fn take(&self) -> bool {
        let mut board_cnt = self.board_cnt.lock().unwrap();
        
        if *board_cnt <= 0 {
            return false;
        }
        
        *board_cnt -= 1;
        
        return true;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        if self.name().is_some() {
            self.name() == other.name()
        } else {
            false
        }
    }
}