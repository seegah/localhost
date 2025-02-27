pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn parse(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let request = String::from_utf8_lossy(data);
        let mut lines = request.lines();
        
        let first_line = lines.next().ok_or("Empty request")?;
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err("Invalid request line".into());
        }

        let mut headers = Vec::new();
        let mut body_start = 0;
        for line in lines {
            body_start += line.len() + 2; // Include \r\n
            if line.is_empty() {
                break;
            }
            let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
            if header_parts.len() == 2 {
                headers.push((header_parts[0].to_string(), header_parts[1].to_string()));
            }
        }

        let body = if body_start < data.len() {
            data[body_start..].to_vec()
        } else {
            Vec::new()
        };

        Ok(Request {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
            headers,
            body,
        })
    }
}