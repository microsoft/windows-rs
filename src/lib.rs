#![allow(dead_code)]

mod activation;
mod array;
mod error;
mod events;
mod guid;
mod hstring;
mod object;
mod param;
mod ref_count;
mod runtime;
mod traits;
mod unknown;
mod weak;

pub use activation::*;
pub use array::*;
pub use error::*;
pub use events::*;
pub use guid::*;
pub use hstring::*;
pub use object::*;
pub use param::*;
pub use ref_count::*;
pub use runtime::*;
pub use traits::*;
pub use unknown::*;
pub use winrt_macros::*;

pub type RawPtr = *mut std::ffi::c_void;
