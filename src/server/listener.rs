use crate::http::{Request, Response};
use crate::server::route::route_request;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn start_listening(host: String, ports: Vec<u16>) {
    for port in ports {
        let address = format!("{}:{}", host, port);
        match TcpListener::bind(&address) {
            Ok(listener) => {
                println!("Server listening on {}", address);

                // Crée un thread pour chaque port
                thread::spawn(move || {
                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => handle_connection(stream),
                            Err(e) => eprintln!("Connection error: {}", e),
                        }
                    }
                });
            }
            Err(e) => eprintln!("Failed to bind to {}: {}", address, e),
        }
    }

    // Empêche le programme principal de se terminer
    loop {
        std::thread::park();
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Buffer pour lire les données de la requête
    stream.read(&mut buffer).unwrap();

    let raw_request = String::from_utf8_lossy(&buffer[..]); // Convertir les octets en chaîne
    let request = Request::parse(&raw_request); // Analyser la requête HTTP

    // Passe la requête au routeur pour obtenir une réponse
    let response = match request {
        Some(req) => route_request(req),
        None => Response::new(404, "404 Not Found".to_string()),
    };

    // Écrire la réponse sur le flux de la connexion
    stream.write_all(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Responded to client with status {}", response.status);
}
