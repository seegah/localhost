#[derive(Debug)]
pub struct ServerError {
    message: String,
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Server error: {}", self.message)
    }
}

impl std::error::Error for ServerError {}

impl From<&str> for ServerError {
    fn from(message: &str) -> Self {
        ServerError { message: message.to_string() }
    }
}