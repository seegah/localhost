use std::time::{Duration, Instant};

pub struct Timeout {
    start: Instant,
    duration: Duration,
}

impl Timeout {
    pub fn new(seconds: u64) -> Self {
        Timeout {
            start: Instant::now(),
            duration: Duration::from_secs(seconds),
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.start) > self.duration
    }
}