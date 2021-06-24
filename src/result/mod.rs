mod error;
mod hresult;

pub use error::*;
pub use hresult::*;

/// A [`Result`] type that provides Windows error information.
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;
