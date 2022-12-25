use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, Weak};
use std::thread::spawn;
use std::time::Instant;

use super::client::Client;
use super::message::{TextMessage, Serializable};
use crate::game::{player::Player, Game};
use crate::machine::Machine;

pub struct MessageReceiver {
    games: Arc<Mutex<Vec<Arc<Game>>>>,
    players: Arc<Mutex<HashMap<i32, Weak<Player>>>>,
    disconnect_channel: Sender<Weak<Client>>,
    last_deletion_cycle: Instant,
    ping_cycle: Instant,
}

impl MessageReceiver {
    pub fn new(disconnect_channel: Sender<Weak<Client>>) -> Self {
        Self {
            games: Arc::new(Mutex::new(vec![])),
            players: Arc::new(Mutex::new(HashMap::new())),
            disconnect_channel,
            last_deletion_cycle: Instant::now(),
            ping_cycle: Instant::now(),
        }
    }

    fn find_player(&self, fd: i32) -> Option<Arc<Player>> {
        let mut lock = self.players.lock().unwrap();
        if let Some(player) = lock.get(&fd).and_then(|p| p.upgrade()) {
            if player.client().upgrade().is_none() {
                lock.remove(&fd);
                return None;
            } else {
                return Some(player);
            }
        }

        None
    }
    
    fn ping_all(players: Arc<Mutex<HashMap<i32, Weak<Player>>>>) {
        for (_, player) in players.lock().unwrap().iter() {
            if let Some(player) = player.upgrade() {
                
                if let Some(rest_timer) = player.rest_timer() {
                    if rest_timer.elapsed().as_secs() >= 3 {
                        if let Some(client) = player.client().upgrade() {
                            if let Ok(_) = client.write(&TextMessage::Ping.serialize()) {
                                println!("pinged!");
                                player.update_ping_timer();
                            }
                        }
                    }
                }
            }
        }
    }

    fn deletion_cycle_proc(games: Arc<Mutex<Vec<Arc<Game>>>>, players: Arc<Mutex<HashMap<i32, Weak<Player>>>>) {
        let mut indices_games = vec![];
        let mut indices_players = vec![];
        let mut games = games.lock().unwrap();
        for (index, game) in games.iter().enumerate() {
            if game.can_delete() {
                indices_games.push(index);
                
                for player in game.players() {
                    if let Some(client) = player.client().upgrade() {
                        indices_players.push(client.sock_fd());
                    }
                }
            }
        }
        
        for index in indices_games.iter().rev() {
            games.remove(*index);
        }
        
        let mut lock = players.lock().unwrap();
        for player in indices_players {
            lock.remove(&player);
        }
    }
    
    fn deletion_cycle_players(disconnect_channel: Sender<Weak<Client>>, players: Arc<Mutex<HashMap<i32, Weak<Player>>>>) {
        for (_, player) in players.lock().unwrap().iter() {
            if let Some(player) = player.upgrade() {
                let timer = player.ping_timer();
                if timer.elapsed().as_secs() >= 5 {
                    if let (Some(game), Some(client)) = (player.game().upgrade(), player.client().upgrade()) {
                        disconnect_channel.send(player.client()).unwrap();
                        game.notify_disconnect(client);
                    }
                }
            }
        }
    }

    pub fn disconnect(&self, client: Weak<Client>) {
        self.disconnect_channel.send(client).unwrap();
    }

    pub fn run(&mut self, channel: Receiver<(Arc<Client>, TextMessage)>) {
        loop {
            
            if self.ping_cycle.elapsed().as_millis() >= 100 {
                let players = self.players.clone();
                spawn(|| Self::ping_all(players));
                self.ping_cycle = Instant::now();
            }
            
            if self.last_deletion_cycle.elapsed().as_secs() >= 5 {
                let games = self.games.clone();
                let players = self.players.clone();
                spawn(|| Self::deletion_cycle_proc(games, players));
                let players = self.players().clone();
                let channel = self.disconnect_channel.clone();
                spawn(|| Self::deletion_cycle_players(channel, players));
                self.last_deletion_cycle = Instant::now();
            }

            while let Ok((client, msg)) = channel.try_recv() {
                match msg {
                    TextMessage::Ping => {
                        if let Ok(_) = client.write(&TextMessage::Pong.serialize()) {
                            println!("sent pong");
                            
                            if let Some(player) = self.find_player(client.sock_fd()) {
                                player.update_ping_timer();
                            }
                            
                        }
                    },
                    TextMessage::Pong => {
                        if let Some(player) = self.find_player(client.sock_fd()) {
                            player.update_ping_timer();
                        }
                    },
                    TextMessage::Disconnect => {
                        if let Some(player) = self.find_player(client.sock_fd()) {
                            if let Some(game) = player.game().upgrade() {
                                game.notify_disconnect(client);
                            }
                        }
                    }

                    _ => {
                        println!("got message: {:?}", msg);
                        // 1. try to find player for given client
                        if let Some(player) = self.find_player(client.sock_fd()) {
                            let machine = player.machine().clone();
                            machine.handle_message(msg, &self, player);
                        } else {
                            Machine::new_client_init(msg, &self, Arc::downgrade(&client));
                        }
                    }
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn games(&self) -> &Mutex<Vec<Arc<Game>>> {
        &self.games
    }

    pub fn players(&self) -> Arc<Mutex<HashMap<i32, Weak<Player>>>> {
        self.players.clone()
    }
}
