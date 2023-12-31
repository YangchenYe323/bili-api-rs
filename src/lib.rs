pub mod ureq;
pub mod apis;

use serde::de::DeserializeOwned;

/// Implemented by specific HTTP request types of any specific http library.
pub trait BiliApiRequest {
  type Response: BiliApiResponse;
  type Error: std::error::Error;

  fn execute(self) -> std::result::Result<Self::Response, Self::Error>;

  fn execute_post_form(self, form: &[(&str, &str)]) -> std::result::Result<Self::Response, Self::Error>;

  fn set_header(self, header: &str, value: &str) -> Self;
}

/// Implemented by specific HTTP response types of any specific
/// http library users want to use to access Bilibili's API, as long as it can be turned into
/// a json type (in a synchronous fashion...No promise any async variant will be supported lol)
pub trait BiliApiResponse {
  type Error: std::error::Error;
  fn deserialize_json<T: DeserializeOwned>(self) -> std::result::Result<T, Self::Error>;
}

/// Implemented by HTTP client types specific http library. We use this to generate
/// the request we need.
pub trait BiliClient {
  type Request: BiliApiRequest;
  fn create_request(&self, method: &str, url: &str) -> Self::Request;
}