pub struct Status {
    code: u16,
    message: &'static str,
}

impl Status {
    pub fn new(code: u16) -> Self {
        match code {
            200 => Status { code: 200, message: "OK" },
            400 => Status { code: 400, message: "Bad Request" },
            403 => Status { code: 403, message: "Forbidden" },
            404 => Status { code: 404, message: "Not Found" },
            405 => Status { code: 405, message: "Method Not Allowed" },
            413 => Status { code: 413, message: "Payload Too Large" },
            500 => Status { code: 500, message: "Internal Server Error" },
            _ => Status { code: 500, message: "Internal Server Error" },
        }
    }

    pub fn code(&self) -> u16 { self.code }
    pub fn message(&self) -> &str { self.message }
}