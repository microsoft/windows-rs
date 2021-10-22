#[macro_use]
mod macros;

mod abi;
mod activation_factory;
mod array;
pub(crate) mod bindings;
mod compose;
mod delay_load;
mod error;
mod factory_cache;
mod guid;
mod handle;
mod heap;
mod hresult;
mod hstring;
mod inspectable;
mod interface;
mod into_param;
mod param;
mod ref_count;
mod runtime_name;
mod runtime_type;
mod sha1;
mod to_impl;
mod unknown;
mod waiter;
mod weak;
mod weak_ref_count;

pub use abi::*;
pub use activation_factory::*;
pub use array::*;
pub use compose::*;
pub use delay_load::*;
pub use error::*;
pub use factory_cache::*;
pub use guid::*;
pub use handle::*;
pub use heap::*;
pub use hresult::*;
pub use hstring::*;
pub use inspectable::*;
pub use interface::*;
pub use into_param::*;
pub use param::*;
pub use ref_count::*;
pub use runtime_name::*;
pub use runtime_type::*;
pub use sha1::*;
pub use to_impl::*;
pub use unknown::*;
pub use waiter::*;
pub use weak::*;
pub use weak_ref_count::*;

// A [`Result`] type that provides Windows error information.
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

pub use bindings::Windows::Win32::System::Com::IAgileObject;

// TODO: rather than hiding, consider just removing
#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;

#[cfg(feature = "macros")]
pub use windows_macros::{build, generate, implement, include_bindings};

pub use windows_macros::{StructDerive};

// TODO: remove this
#[cfg(feature = "macros")]
pub use windows_reader::workspace_dir;

extern "C" {
    pub fn memcmp(left: *const std::ffi::c_void, right: *const std::ffi::c_void, len: usize) -> i32;
}
