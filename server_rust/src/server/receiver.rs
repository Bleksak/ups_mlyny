use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use super::client::Client;
use super::message::Message;

pub struct MessageReceiver {
    receiver: Receiver<(Client, Message)>
}

impl MessageReceiver {
    pub fn new(receiver: Receiver<(Client, Message)>) -> Self {
        Self { receiver }
    }
    
    pub fn run(self) {
        loop {
            while let Ok((client, msg)) = self.receiver.try_recv() {
                println!("Got message {:?}", msg);
                
                match msg {
                    Message::OK => {},
                    Message::NOK(_) => {},
                    Message::CREATE(_) => {},
                    Message::JOIN(_) => {},
                    Message::READY => {},
                    Message::PUT(_) => {},
                    Message::TAKE(_) => {},
                    Message::MOVE(_, _) => {},
                    Message::OVER => {},
                }
            }
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    pub fn start(self) -> JoinHandle<()> {
        std::thread::spawn(move|| {
            self.run();
        })
    }
}


