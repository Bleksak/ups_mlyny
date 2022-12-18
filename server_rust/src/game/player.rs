use std::{sync::{Arc, Mutex, Weak}, time::Instant};

use crate::{
    machine::{Machine, State},
    server::client::Client,
};

use super::{color::Color, Game};

#[derive(Debug)]
pub struct Player {
    username: Mutex<Option<String>>,
    client: Mutex<Weak<Client>>,
    color: Color,
    inventory_cnt: Mutex<usize>,
    board_cnt: Mutex<usize>,
    machine: Arc<Machine>,
    game: Mutex<Weak<Game>>,
    ping_timer: Mutex<Instant>,
    rest_timer: Mutex<Option<Instant>>,
}

impl Player {
    pub fn new(color: Color, state: State) -> Self {
        Self {
            username: Mutex::new(None),
            client: Mutex::new(Weak::new()),
            color,
            inventory_cnt: Mutex::new(9),
            board_cnt: Mutex::new(0),
            machine: Arc::new(Machine::new(state)),
            game: Mutex::new(Weak::new()),
            ping_timer: Mutex::new(Instant::now()),
            rest_timer: Mutex::new(Some(Instant::now())),
        }
    }
    
    pub fn rest_timer(&self) -> Option<Instant> {
        self.rest_timer.lock().unwrap().clone()
    }
    
    pub fn rest(&self) {
        *self.rest_timer.lock().unwrap() = Some(Instant::now());
    }
    
    pub fn ping_timer(&self) -> Instant {
        self.ping_timer.lock().unwrap().clone()
    }
    
    pub fn update_ping_timer(&self) {
        *self.ping_timer.lock().unwrap() = Instant::now();
        *self.rest_timer.lock().unwrap() = None;
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
