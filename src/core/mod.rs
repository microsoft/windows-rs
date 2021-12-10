mod abi;
mod activation_factory;
mod array;
pub(crate) mod bindings;
mod compose;
mod delay_load;
mod error;
mod factory_cache;
mod guid;
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

#[doc(hidden)]
pub use abi::*;
#[doc(hidden)]
pub use activation_factory::*;
pub use array::*;
#[doc(hidden)]
pub use compose::*;
pub(crate) use delay_load::*;
pub use error::*;
#[doc(hidden)]
pub use factory_cache::*;
pub use guid::*;
#[doc(hidden)]
pub use heap::*;
pub use hresult::*;
pub use hstring::*;
#[doc(hidden)]
pub use inspectable::*;
#[doc(hidden)]
pub use interface::*;
#[doc(hidden)]
pub use into_param::*;
#[doc(hidden)]
pub use param::*;
#[doc(hidden)]
pub use ref_count::*;
#[doc(hidden)]
pub use runtime_name::*;
#[doc(hidden)]
pub use runtime_type::*;
#[doc(hidden)]
pub use sha1::*;
#[doc(hidden)]
pub use to_impl::*;
pub use unknown::*;
#[doc(hidden)]
pub use waiter::*;
#[doc(hidden)]
pub use weak::*;
#[doc(hidden)]
pub use weak_ref_count::*;

// A [`Result`] type that provides Windows error information.
#[must_use]
pub type Result<T> = core::result::Result<T, Error>;

#[doc(hidden)]
pub use bindings::IAgileObject;

// TODO: rather than hiding, consider just removing
#[doc(hidden)]
pub type RawPtr = *mut core::ffi::c_void;

#[cfg(feature = "build")]
pub use windows_macros::{build, build_legacy, generate, implement, include_bindings};

// TODO: remove this
#[cfg(feature = "build")]
pub use windows_reader::workspace_dir;

extern "C" {
    #[doc(hidden)]
    pub fn memcmp(left: *const core::ffi::c_void, right: *const core::ffi::c_void, len: usize) -> i32;
}

#[doc(hidden)]
pub extern crate alloc;
