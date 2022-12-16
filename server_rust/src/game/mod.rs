use crate::{
    game::color::Color,
    machine::State,
    server::{client::Client, message::Message, receiver::MessageReceiver},
};

use self::{board::Board, player::Player};
use std::{
    sync::{Arc, RwLock, Weak},
    time::{self, Instant},
};

pub mod board;
pub mod color;
pub mod player;

#[derive(Debug)]
pub struct Game {
    turn: RwLock<usize>,
    players: [Arc<Player>; 2],
    board: Board,
    destroy_timer_start: RwLock<Option<time::Instant>>,
}

pub enum GameError {
    // PlayerNotInGame,
    NotYourTurn,
    CannotMove,
    BadPosition,
    FieldTaken,
    InvalidField,
}

impl GameError {
    pub fn to_string(self) -> String {
        match self {
            // GameError::PlayerNotInGame => None,
            GameError::NotYourTurn => "It's not your turn".to_string(),
            GameError::CannotMove => "Cannot move".to_string(),
            GameError::BadPosition => "Invalid position".to_string(),
            GameError::FieldTaken => "Field is already taken".to_string(),
            GameError::InvalidField => "Invalid field chosen".to_lowercase(),
        }
    }
}

impl Game {
    pub fn new() -> Arc<Self> {
        let s = Arc::new(Self {
            turn: RwLock::new(0),
            players: [
                Arc::new(Player::new(Color::Red, State::InGamePut)),
                Arc::new(Player::new(Color::Blue, State::InGamePutOpponent)),
            ],
            board: Board::new(),
            destroy_timer_start: RwLock::new(None),
        });

        for p in s.players.iter() {
            p.set_game(Arc::downgrade(&s));
        }

        s
    }

    pub fn can_delete(&self) -> bool {
        let s = self.destroy_timer_start.read().unwrap();
        if let Some(timer) = *s {
            if timer.elapsed().as_secs() >= 30 {
                return true;
            }
        }

        false
    }

    pub fn notify_disconnect(&self, client: Arc<Client>) {
        let connected_cnt = self
            .players
            .iter()
            .filter_map(|pl| pl.client().upgrade())
            .count();
        if connected_cnt == 0 {
            *self.destroy_timer_start.write().unwrap() = Some(Instant::now());
        } else {
            for player in self
                .players
                .iter()
                .filter_map(|player| player.client().upgrade())
            {
                if client == player {
                    continue;
                }

                if let Ok(_) = player.write(&Message::Disconnect.serialize()) {
                    println!("disconnect notification sent");
                }
            }
        }
    }

    fn notify_join(&self) {
        let count = self
            .players
            .iter()
            .filter_map(|p| p.client().upgrade())
            .count();

        for (idx, player) in self.players.iter().enumerate() {
            if let Some(client) = player.client().upgrade() {
                let msg = if count == 2 {
                    Message::Ready(
                        player.machine().state(),
                        player.color(),
                        self.board.serialize(),
                        self.players[(idx + 1) % 2].name().unwrap(),
                    )
                } else {
                    Message::PlayerJoined
                };
                if let Ok(_) = client.write(&msg.serialize()) {
                    println!("player {} notified", player.name().as_ref().unwrap());
                }
            }
        }
    }

    pub fn try_join(
        &self,
        username: &str,
        receiver: &MessageReceiver,
        client: Weak<Client>,
    ) -> Option<()> {
        let mut val = None;

        if let Some(client) = client.upgrade() {
            for player in self.players.iter() {
                if let Some(name) = player.name() {
                    if name == username {
                        if player.client().upgrade().is_none() {
                            player.bind(Arc::downgrade(&client));

                            let mut plock = receiver.players().lock().unwrap();
                            let entry = plock.entry(client.sock_fd());
                            entry.or_insert(Arc::downgrade(player));

                            val = Some(());
                            break;
                        }
                    }
                }
            }
        }

        if let Some(_) = val {
            self.notify_join();
        }

        val
    }

    pub fn force_join(
        &self,
        username: &str,
        receiver: &MessageReceiver,
        client: Weak<Client>,
    ) -> Option<()> {
        let mut val = None;

        if let Some(_) = self
            .players
            .iter()
            .filter_map(|p| p.name())
            .find(|p| p == username)
        {
            return None;
        }

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
        }

        if let Some(_) = val {
            self.notify_join();
        }

        val
    }

    pub fn has_player(&self, username: &str) -> Option<Arc<Player>> {
        for player in self.players.iter() {
            if let Some(name) = player.name() {
                if name == username {
                    return Some(player.clone());
                }
            }
        }

        None
    }

    pub fn put(&self, player: Arc<Player>, pos: usize) -> Result<Arc<Player>, GameError> {
        let turn = *self.turn.read().unwrap();
        let player_turn = self.players[turn].clone();

        if player != player_turn {
            return Err(GameError::NotYourTurn);
        }

        let opponent_index = (turn + 1) % 2;

        self.board.put(pos, player.color())?;
        player.put();

        let opponent = self.players.get(opponent_index).unwrap();

        if self.board.check_mill_vertical(pos, None) || self.board.check_mill_horizontal(pos, None)
        {
            opponent.machine().set_state(State::InGameTakeOpponent);
            player.machine().set_state(State::InGameTake);
        } else {
            let inv_count: usize = self.players.iter().map(|p| p.inventory()).sum();
            if inv_count == 0 {
                opponent.machine().set_state(State::InGameMove);
                player.machine().set_state(State::InGameMoveOpponent);
            } else {
                opponent.machine().set_state(State::InGamePut);
                player.machine().set_state(State::InGamePutOpponent);
            }
            let mut turn = self.turn.write().unwrap();
            *turn = (*turn + 1) % 2;
        }

        Ok(opponent.clone())
    }

    pub fn mmove(
        &self,
        player: Arc<Player>,
        pos: (usize, usize),
    ) -> Result<Arc<Player>, GameError> {
        let turn = *self.turn.read().unwrap();
        let player_turn = self.players[turn].clone();

        if player != player_turn {
            return Err(GameError::NotYourTurn);
        }

        if player.color() != self.board.get(pos.0)? || Color::Neutral != self.board.get(pos.1)? {
            return Err(GameError::CannotMove);
        }

        let opponent_index = (turn + 1) % 2;
        self.board.mmove(pos, player.board() < 3)?;
        let opponent = self.players.get(opponent_index).unwrap();

        if self.board.check_mill_vertical(pos.1, None)
            || self.board.check_mill_horizontal(pos.1, None)
        {
            opponent.machine().set_state(State::InGameTakeOpponent);
            player.machine().set_state(State::InGameTake);
        } else {
            if self.board.check_draw(player.color()) {
                // TODO: draw
                opponent.machine().set_state(State::GameOver);
                player.machine().set_state(State::GameOver);
            } else {
                opponent.machine().set_state(State::InGameMove);
                player.machine().set_state(State::InGameMoveOpponent);
                let mut turn = self.turn.write().unwrap();
                *turn = (*turn + 1) % 2;
            }
        }

        Ok(opponent.clone())
    }

    pub fn take(&self, player: Arc<Player>, pos: usize) -> Result<Arc<Player>, GameError> {
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
        opponent.take();

        let inv_cnt: usize = self.players.iter().map(|p| p.inventory()).sum();

        if opponent.board() + opponent.inventory() < 3 {
            opponent.machine().set_state(State::GameOver);
            player.machine().set_state(State::GameOver);
        } else if inv_cnt == 0 {
            opponent.machine().set_state(State::InGameMove);
            player.machine().set_state(State::InGameMoveOpponent);
        } else {
            opponent.machine().set_state(State::InGamePut);
            player.machine().set_state(State::InGamePutOpponent);
        }

        let mut turn = self.turn.write().unwrap();
        *turn = (*turn + 1) % 2;

        Ok(opponent.clone())
    }

    pub fn game_over(&self, receiver: &MessageReceiver) {
        //1. disconnect players

        for player in self.players.iter() {
            receiver.disconnect(player.client());
        }

        //2. destroy game

        let mut lock = receiver.games().lock().unwrap();

        let result = lock.iter().enumerate().find(|g| {
            *g.1.turn.read().unwrap() == *self.turn.read().unwrap()
                && g.1.board == self.board
                && g.1.players == self.players
        });

        if let Some((index, _)) = result {
            lock.swap_remove(index);
            println!("game removed!");
        }
    }
}
