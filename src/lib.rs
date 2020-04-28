#![allow(dead_code)]

mod activation;
mod array;
mod com_interface;
mod com_ptr;
mod error;
mod events;
mod guid;
mod hstring;
mod object;
mod param;
mod ref_count;
mod runtime;
mod runtime_name;
mod runtime_type;
mod try_into;
mod unknown;
mod weak;

pub use activation::*;
pub use array::*;
pub use com_interface::*;
pub use com_ptr::*;
pub use error::*;
pub use events::*;
pub use guid::*;
pub use hstring::*;
pub use object::*;
pub use param::*;
pub use ref_count::*;
pub use runtime::*;
pub use runtime_name::*;
pub use runtime_type::*;
pub use try_into::*;
pub use unknown::*;
pub use winrt_macros::*;

pub type RawPtr = *mut std::ffi::c_void;
