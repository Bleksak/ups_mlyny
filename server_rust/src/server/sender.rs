use std::sync::mpsc::Receiver;
use std::sync::Arc;
use crate::server::{Client, Message};

pub struct MessageSender {}

impl MessageSender {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn run(&self, channel: Receiver<(Arc<Client>, Message)>) {
        loop {
            while let Ok((client, message)) = channel.try_recv() {
                if let Ok(_) = client.write(&message.serialize()) {
                    println!("Message sent ok")
                } else {
                    println!("Failed to send message");
                }
            }
            
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
}
