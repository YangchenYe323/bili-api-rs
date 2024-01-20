#![deny(missing_debug_implementations)]

mod utils;

pub mod apis;
pub mod credential;
pub mod error;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

pub use error::Error;
