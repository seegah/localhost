pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
}

impl Request {
    /// Analyse une chaîne brute pour en extraire une requête HTTP
    pub fn parse(raw_request: &str) -> Option<Self> {
        let lines: Vec<&str> = raw_request.split("\r\n").collect();
        let request_line = lines.get(0)?; // La première ligne contient la méthode, le chemin et la version HTTP
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() != 3 {
            return None;
        }

        Some(Request {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            http_version: parts[2].to_string(),
        })
    }
}
