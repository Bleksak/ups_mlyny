mod server;

fn main() -> Result<(), std::io::Error> {
    server::Server::new(2000)?.start();
    Ok(())
}
