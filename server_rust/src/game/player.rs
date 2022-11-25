use std::sync::Weak;

use crate::server::client::Client;

use super::color::Color;

pub struct Player {
    username: Option<String>,
    client: Weak<Client>,
    color: Color
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            username: None,
            client: Weak::new(),
            color
        }
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
    
    pub fn client(&self) -> &Weak<Client> {
        &self.client
    }
    
    pub fn color(&self) -> Color {
        self.color
    }
    
}

// impl PartialEq for Player {
//     fn eq(&self, other: &Self) -> bool {
//         self.username == other.username
//     }
// }
