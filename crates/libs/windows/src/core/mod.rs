mod abi;
mod activation_factory;
mod agile_reference;
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
mod pcstr;
mod pcwstr;
mod pstr;
mod pwstr;
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
pub use agile_reference::*;
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
pub use inspectable::*;
pub use interface::*;
#[doc(hidden)]
pub use into_param::*;
#[doc(hidden)]
pub use param::*;
pub use pcstr::*;
pub use pcwstr::*;
pub use pstr::*;
pub use pwstr::*;
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
pub use weak::*;
#[doc(hidden)]
pub use weak_ref_count::*;

/// A specialized [`Result`] type that provides Windows error information.
pub type Result<T> = core::result::Result<T, Error>;

#[doc(hidden)]
pub use bindings::IAgileObject;

// TODO: rather than hiding, consider just removing
#[doc(hidden)]
pub type RawPtr = *mut core::ffi::c_void;

#[doc(hidden)]
#[cfg(feature = "implement")]
pub use windows_implement::implement;

#[doc(hidden)]
#[cfg(feature = "interface")]
pub use windows_interface::interface;

extern "C" {
    #[doc(hidden)]
    pub fn memcmp(left: *const core::ffi::c_void, right: *const core::ffi::c_void, len: usize) -> i32;
}

#[doc(hidden)]
pub extern crate alloc;

#[doc(hidden)]
pub fn as_ptr_or_null<T>(value: &[T]) -> *const T {
    if value.is_empty() {
        core::ptr::null()
    } else {
        value.as_ptr()
    }
}

#[doc(hidden)]
pub fn as_mut_ptr_or_null<T>(value: &mut [T]) -> *mut T {
    if value.is_empty() {
        core::ptr::null_mut()
    } else {
        value.as_mut_ptr()
    }
}
