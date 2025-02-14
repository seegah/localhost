use crate::http::headers::Headers;

pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: Headers, // Ajout des en-têtes HTTP
}

impl Response {
    pub fn new(status: u16, body: String) -> Self {
        let mut headers = Headers::new();
        headers.set("Content-Length", &body.len().to_string());
        headers.set("Content-Type", "text/html"); // Par défaut, texte HTML

        Response {
            status,
            body,
            headers,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status,
            self.status_text(),
            self.headers.to_string(),
            self.body
        )
    }

    fn status_text(&self) -> &str {
        match self.status {
            200 => "OK",
            404 => "Not Found",
            400 => "Bad Request",
            500 => "Internal Server Error",
            _ => "Unknown",
        }
    }
}
