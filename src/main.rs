mod server;
mod http;
mod handlers;
mod utils;

use server::Server;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new("config/server.conf");
    let mut server = Server::new(config_path)?;
    server.run()?;
    Ok(())
}