use crate::http::Response;

pub fn not_found() -> Response {
    Response::new(404, "404 Not Found".to_string())

}
