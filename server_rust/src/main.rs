use server::receiver::MessageReceiver;
use std::sync::mpsc;

mod server;
mod machine;
mod game;

fn main() -> Result<(), std::io::Error> {
    let receiver = MessageReceiver::new(mpsc::channel());
    server::Server::new(2000, receiver)?.start();
    
    Ok(())
}
