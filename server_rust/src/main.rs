mod server;
mod machine;
mod game;

fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    args.next();
    
    if let (Some(ip), Some(port)) = (args.next(), args.next()) {
        let port = port.parse().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Provide a valid port number"))?;
        
        if port == 0 {
            println!("Port 0 is not allowed!");
            return Ok(());
        }
        
        server::Server::start(ip, port)?;
    } else {
        println!("Please provide IP and port! For example: ./server 127.0.0.1 2000");
    }
    
    Ok(())
}
