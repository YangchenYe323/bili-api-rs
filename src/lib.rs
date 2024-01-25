mod utils;

pub mod apis;
pub mod client;
pub mod credential;
pub mod error;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

pub use client::Client;
pub use error::Error;
