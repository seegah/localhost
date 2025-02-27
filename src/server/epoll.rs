use libc::{epoll_create1, epoll_ctl, epoll_wait, epoll_event, EPOLLIN, EPOLL_CTL_ADD, EPOLL_CTL_DEL};

pub struct Epoll {
    fd: i32,
}

impl Epoll {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let fd = unsafe { epoll_create1(0) };
        if fd < 0 {
            return Err("Failed to create epoll instance".into());
        }
        Ok(Epoll { fd })
    }

    pub fn register(&self, fd: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut event = epoll_event {
            events: EPOLLIN as u32,
            u64: fd as u64,
        };
        let result = unsafe { epoll_ctl(self.fd, EPOLL_CTL_ADD, fd, &mut event) };
        if result < 0 {
            return Err("Failed to register fd with epoll".into());
        }
        Ok(())
    }

    pub fn unregister(&self, fd: i32) -> Result<(), Box<dyn std::error::Error>> {
        let result = unsafe { epoll_ctl(self.fd, EPOLL_CTL_DEL, fd, std::ptr::null_mut()) };
        if result < 0 {
            return Err("Failed to unregister fd from epoll".into());
        }
        Ok(())
    }

    pub fn wait(&self) -> Result<Vec<EpollEvent>, Box<dyn std::error::Error>> {
        let mut events = vec![epoll_event { events: 0, u64: 0 }; 10];
        let count = unsafe { epoll_wait(self.fd, events.as_mut_ptr(), 10, 1000) };
        if count < 0 {
            return Err("Epoll wait failed".into());
        }
        Ok(events[..count as usize].iter().map(|e| EpollEvent { fd: e.u64 as i32 }).collect())
    }
}

pub struct EpollEvent {
    fd: i32,
}

impl EpollEvent {
    pub fn fd(&self) -> i32 { self.fd }
}

impl Drop for Epoll {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}