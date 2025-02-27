use crate::http::{Response, Status};
use crate::server::config::ServerConfig;
use std::fs;

pub fn get_error_page(code: u16, config: &ServerConfig) -> Response {
    let mut response = Response::new(Status::new(code));
    
    if let Some((_, path)) = config.error_pages.iter().find(|(c, _)| *c == code) {
        if let Ok(content) = fs::read(path) {
            response.body = content;
            response.headers.push(("Content-Type".to_string(), "text/html".to_string()));
            return response;
        }
    }

    response.body = format!(
        "<html><body><h1>{} {}</h1></body></html>",
        code,
        response.status.message()
    ).into_bytes();
    response.headers.push(("Content-Type".to_string(), "text/html".to_string()));
    response
}