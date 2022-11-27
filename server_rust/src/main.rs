mod server;
mod machine;
mod game;

fn main() -> Result<(), std::io::Error> {
    server::Server::start(2000)?;
    Ok(())
}
