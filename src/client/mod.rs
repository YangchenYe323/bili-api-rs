//! This module implements canonical client type used for accessing the
//! Bilibili API

use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use reqwest::{
  blocking::{Request, RequestBuilder, Response},
  IntoUrl,
};

use self::{config::LiveMsgConfig, rate_limiting::RateLimiter};

mod config;
mod rate_limiting;

#[derive(Clone)]
pub struct Client {
  inner: ClientInner,
  pub(crate) live_msg_config: LiveMsgConfig,
}

impl Default for Client {
  fn default() -> Self {
    Self::new()
  }
}

impl Client {
  pub fn new() -> Self {
    const DEFAULT_LIMIT: Duration = Duration::from_millis(1000);
    Self::with_rate_limit(DEFAULT_LIMIT)
  }

  /// 自定义直播弹幕API限流
  pub fn with_rate_limit(limit: Duration) -> Self {
    let rate_limiter = RateLimiter::new(limit);
    let inner = ClientInner {
      client: reqwest::blocking::Client::new(),
      rate_limiter: Arc::new(Mutex::new(rate_limiter)),
    };

    Self {
      inner,
      live_msg_config: LiveMsgConfig::with_duration_and_retry(Duration::from_secs(3), 1),
    }
  }
}

#[derive(Clone)]
struct ClientInner {
  client: reqwest::blocking::Client,
  rate_limiter: Arc<Mutex<RateLimiter>>,
}

impl Client {
  pub(crate) fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
    self.inner.client.get(url)
  }

  pub(crate) fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
    self.inner.client.post(url)
  }

  pub(crate) fn execute(&self, request: Request) -> reqwest::Result<Response> {
    self.inner.client.execute(request)
  }

  pub(crate) fn block_till_ready(&self) {
    let mut guard = self.inner.rate_limiter.lock().expect("Lock poisoned");
    guard.block_till_ready();
  }
}

#[cfg(test)]
mod tests {
  use crate::client::Client;

  #[test]
  pub fn test_client_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Client>();
  }

  #[test]
  pub fn test_client_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Client>();
  }
}
