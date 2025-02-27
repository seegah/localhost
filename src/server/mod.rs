pub mod epoll;
pub mod config;
pub mod listener;
pub mod route;

use config::Config;
use listener::Listener;
use epoll::Epoll;
use route::Router;
use crate::http::Request;
use crate::utils::timeout::Timeout;
use std::path::Path;

pub struct Server {
    config: Config,
    listeners: Vec<Listener>,
    epoll: Epoll,
    router: Router,
}

impl Server {
    pub fn new(config_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::from_file(config_path)?;
        let epoll = Epoll::new()?;
        let mut listeners = Vec::new();
        
        for server_config in &config.servers {
            for &port in &server_config.ports {
                listeners.push(Listener::new(&server_config.host, port)?);
            }
        }
        
        let router = Router::new(&config);
        
        Ok(Server {
            config,
            listeners,
            epoll,
            router,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for listener in &self.listeners {
            self.epoll.register(listener.fd())?;
        }
        
        let mut buffer = [0; 1024];
        let timeout = Timeout::new(30); // 30-second timeout
        
        loop {
            if timeout.is_expired() {
                println!("Server timeout reached");
                break;
            }

            let events = self.epoll.wait()?;
            for event in events {
                let fd = event.fd();
                if self.listeners.iter().any(|l| l.fd() == fd) {
                    let client_fd = self.listeners.iter()
                        .find(|l| l.fd() == fd)
                        .unwrap()
                        .accept()?;
                    self.epoll.register(client_fd)?;
                } else {
                    let request = self.read_request(fd, &mut buffer)?;
                    let response = self.router.route(&request);
                    self.write_response(fd, response)?;
                    unsafe { libc::close(fd) };
                    self.epoll.unregister(fd)?;
                }
            }
        }
        Ok(())
    }

    fn read_request(&self, fd: i32, buffer: &mut [u8]) -> Result<Request, Box<dyn std::error::Error>> {
        let bytes_read = unsafe { libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()) };
        if bytes_read <= 0 {
            return Err("Failed to read request".into());
        }
        Request::parse(&buffer[..bytes_read as usize])
    }

    fn write_response(&self, fd: i32, response: crate::http::Response) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = response.to_bytes();
        let bytes_written = unsafe {
            libc::write(fd, bytes.as_ptr() as *const libc::c_void, bytes.len())
        };
        if bytes_written < 0 {
            return Err("Failed to write response".into());
        }
        Ok(())
    }
}