mod server;
mod handlers;
mod http;

use server::config::Config;
use server::listener::start_listening;

fn main() {
    // Charger la configuration depuis `config/server.conf`
    let config_path = "config/server.conf";
    let config = match Config::load(config_path) {
        Ok(cfg) => {
            println!("Configuration loaded: {:?}", cfg);
            cfg
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            return;
        }
    };

    // Démarrer l'écoute sur les ports configurés
    println!("Starting server...");
    start_listening(config.host, config.ports);
}
