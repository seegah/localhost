use crate::http::{Request, Response};
use crate::handlers::{static_file, error_pages};

/// Route une requête HTTP vers le gestionnaire approprié
pub fn route_request(request: Request) -> Response {
    match request.method.as_str() {
        "GET" => {
            match request.path.as_str() {
                "/" => static_file::serve_file("static/index.html"),
                path if path.starts_with("/static/") => static_file::serve_file(&path[1..]),
                _ => error_pages::not_found(),
            }
        }
        "POST" => {
            Response::new(405, "405 Method Not Allowed".to_string()) // POST non pris en charge pour le moment
        }
        _ => Response::new(405, "405 Method Not Allowed".to_string()), // Méthode inconnue
    }
}
