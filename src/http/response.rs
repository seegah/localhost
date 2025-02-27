use crate::http::status::Status;

pub struct Response {
    pub status: Status,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: Status) -> Self {
        Response {
            status,
            headers: vec![],
            body: vec![],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.code(),
            self.status.message()
        );
        
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        response.push_str("\r\n");
        let mut bytes = response.into_bytes();
        bytes.extend(&self.body);
        bytes
    }
}