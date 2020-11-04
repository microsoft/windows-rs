mod error_code;
mod error;

pub use error_code::*;
pub use error::*;

#[must_use]
pub type Result<T> = std::result::Result<T, Error>;
