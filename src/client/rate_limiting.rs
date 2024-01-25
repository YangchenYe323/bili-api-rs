use std::time::{Duration, Instant};

/// A Simple rate limiter which just enforces a configurable Duration between each consecutive calls.
#[derive(Clone)]
pub struct RateLimiter {
  time_limit: Duration,
  last_call: Option<Instant>,
}

impl RateLimiter {
  pub fn new(time_limit: Duration) -> Self {
    Self {
      time_limit,
      last_call: None,
    }
  }
}

impl RateLimiter {
  /// Blocks until next available
  pub fn block_till_ready(&mut self) {
    match self.last_call.as_mut() {
      Some(last_call) => {
        let mut elapsed = last_call.elapsed();
        while elapsed < self.time_limit {
          std::thread::sleep(self.time_limit - elapsed);
          elapsed = last_call.elapsed();
        }
      }
      None => (),
    }

    self.last_call = Some(Instant::now());
  }
}
