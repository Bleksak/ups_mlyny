use server::receiver::MessageReceiver;
use std::sync::mpsc;

mod server;

fn main() -> Result<(), std::io::Error> {
    let recv_channel = mpsc::channel();
    let recv_thread = MessageReceiver::new(recv_channel.1).start();
    server::Server::new(2000, recv_channel.0)?.start();
    recv_thread.join().unwrap();
    Ok(())
}
