use std::time::Duration;

#[derive(Clone)]
pub(crate) struct LiveMsgConfig {
  pub retry_after_rate_limit: Duration,
  pub max_retry: u32,
}

impl LiveMsgConfig {
  pub fn with_duration_and_retry(duration: Duration, max_retry: u32) -> Self {
    Self {
      retry_after_rate_limit: duration,
      max_retry,
    }
  }
}
