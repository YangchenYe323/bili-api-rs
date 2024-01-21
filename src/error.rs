use std::fmt;

/// 请求错误，常见于登陆信息错误
pub const REQUEST_ERROR: i32 = -400;
/// 请求的直播间不存在
pub const ROOM_NOT_EXIST: i32 = 19002001;

/// The crate's canonical error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Reqwest error, this includes I/O error or HTTP status error.
  #[error("{0}")]
  Reqwest(reqwest::Error),
  /// Bilibili's API response error. This happens where the HTTP status code
  /// is OK but the response specific code implies an error.
  #[error("{0}")]
  Api(ApiError),
  /// We encode Bilibili's API response in static Rust type, and this error is returned
  /// if the API responded with success code but we cannot deserialize its body into the
  /// Rust type we have.
  #[error("{0}")]
  Deserialize(serde_json::Error),
}

/// This Error represents business logic error thrown by Bilibili's API server,
/// e.g., querying a live streamer with non-existent uid. The code is normally non-zero
/// with api-specific meanings. Refer to API document for interpreting the error in specific
/// scenarios.
#[derive(Debug, thiserror::Error)]
pub struct ApiError {
  code: i32,
  message: String,
}

impl ApiError {
  pub fn new(code: i32, message: String) -> Self {
    Self { code, message }
  }

  pub fn code(&self) -> i32 {
    self.code
  }

  pub fn message(&self) -> &str {
    &self.message
  }
}

impl fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Bilibili API Error (code: {}, message: {})",
      self.code, &self.message
    )
  }
}

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Self::Reqwest(value)
  }
}

impl From<ApiError> for Error {
  fn from(value: ApiError) -> Self {
    Self::Api(value)
  }
}

impl From<serde_json::Error> for Error {
  fn from(value: serde_json::Error) -> Self {
    Self::Deserialize(value)
  }
}
