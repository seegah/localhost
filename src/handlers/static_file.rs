use crate::http::{Response, Status};
use std::path::Path;
use crate::utils::file;

pub fn serve_file(path: &Path) -> Result<Response, Box<dyn std::error::Error>> {
    let content = file::read_file(path)?;
    let mut response = Response::new(Status::new(200));
    response.headers.push(("Content-Type".to_string(), get_content_type(path)));
    response.headers.push(("Content-Length".to_string(), content.len().to_string()));
    response.body = content;
    Ok(response)
}

fn get_content_type(path: &Path) -> String {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    }.to_string()
}