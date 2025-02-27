use libc::{socket, bind, listen, sockaddr_in, AF_INET, SOCK_STREAM, INADDR_ANY};

pub struct Listener {
    fd: i32,
}

impl Listener {
    pub fn new(host: &str, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let fd = unsafe { socket(AF_INET, SOCK_STREAM, 0) };
        if fd < 0 {
            return Err("Failed to create socket".into());
        }

        let addr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: if host == "127.0.0.1" { libc::in_addr { s_addr: INADDR_ANY } }  else { libc::in_addr { s_addr: 0 }}, // Simplified for localhost
            sin_zero: [0; 8],
        };

        let result = unsafe {
            bind(fd, &addr as *const _ as *const _, std::mem::size_of::<sockaddr_in>() as u32)
        };
        if result < 0 {
            return Err("Failed to bind socket".into());
        }

        let result = unsafe { listen(fd, 128) };
        if result < 0 {
            return Err("Failed to listen on socket".into());
        }

        Ok(Listener { fd })
    }

    pub fn fd(&self) -> i32 { self.fd }

    pub fn accept(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let client_fd = unsafe { libc::accept(self.fd, std::ptr::null_mut(), std::ptr::null_mut()) };
        if client_fd < 0 {
            return Err("Failed to accept connection".into());
        }
        Ok(client_fd)
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}