use crate::http::Response;
use std::fs;

pub fn serve_file(file_path: &str) -> Response {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let mut response = Response::new(200, content);
            let mime_type = get_mime_type(file_path);
            response.headers.set("Content-Type", &mime_type); // Définir le type MIME
            response
        }
        Err(_) => Response::new(404, "404 Not Found".to_string()),
    }
}

/// Retourne le type MIME basé sur l'extension de fichier
fn get_mime_type(file_path: &str) -> String {
    if file_path.ends_with(".html") {
        "text/html".to_string()
    } else if file_path.ends_with(".css") {
        "text/css".to_string()
    } else if file_path.ends_with(".js") {
        "application/javascript".to_string()
    } else if file_path.ends_with(".png") {
        "image/png".to_string()
    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
        "image/jpeg".to_string()
    } else {
        "application/octet-stream".to_string() // Par défaut
    }
}
