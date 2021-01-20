mod bool32;
mod error;
mod error_code;

pub use bool32::*;
pub use error::*;
pub use error_code::*;

/// A `Result` type that provides Windows error information.
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;
