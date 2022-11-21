use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver};
use std::thread::JoinHandle;

use super::message::Message;

pub struct MessageReceiver {
    receiver: Receiver<(TcpStream, Message)>
}

impl MessageReceiver {
    pub fn new(receiver: Receiver<(TcpStream, Message)>) -> Self {
        Self { receiver }
    }
    
    pub fn run(self) {
        loop {
            while let Ok(msg) = self.receiver.try_recv() {
                println!("got message!");
                println!("i ma recever");
            }
        }
    }
    
    pub fn start(self) -> JoinHandle<()> {
        std::thread::spawn(move|| {
            self.run();
        })
    }
}
