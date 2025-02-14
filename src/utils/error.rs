use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ServerError {
    IoError(std::io::Error),
    ParseError,
    ConfigError(Box<dyn Error>), // Nouveau variant pour gérer les erreurs de configuration
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::IoError(err) => write!(f, "IO Error: {}", err),
            ServerError::ParseError => write!(f, "Parse Error"),
            ServerError::ConfigError(err) => write!(f, "Config Error: {}", err),
        }
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IoError(err)
    }
}

// Implémente From pour convertir Box<dyn Error> en ServerError
impl From<Box<dyn Error>> for ServerError {
    fn from(err: Box<dyn Error>) -> Self {
        ServerError::ConfigError(err)
    }
}