mod abi;
mod agile_reference;
mod array;
mod as_impl;
pub(crate) mod bindings;
mod borrowed;
mod compose;
mod delay_load;
mod error;
mod event;
mod factory_cache;
mod generic_factory;
mod guid;
mod heap;
mod hresult;
mod inspectable;
mod interface;
mod param;
mod ref_count;
mod runtime_name;
mod runtime_type;
mod sha1;
mod strings;
mod unknown;
mod waiter;
mod weak;
mod weak_ref_count;

#[doc(hidden)]
pub use abi::*;
pub use agile_reference::*;
pub use array::*;
#[doc(hidden)]
pub use as_impl::*;
pub use borrowed::*;
#[doc(hidden)]
pub use compose::*;
pub(crate) use delay_load::*;
pub use error::*;
pub use event::*;
#[doc(hidden)]
pub use factory_cache::*;
#[doc(hidden)]
pub use generic_factory::*;
pub use guid::*;
pub(crate) use heap::*;
pub use hresult::*;
pub use inspectable::*;
pub use interface::*;
pub use param::*;
#[doc(hidden)]
pub use ref_count::*;
#[doc(hidden)]
pub use runtime_name::*;
#[doc(hidden)]
pub use runtime_type::*;
#[doc(hidden)]
pub use sha1::*;
pub use strings::*;
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

fn wide_trim_end(mut wide: &[u16]) -> &[u16] {
    while let Some(last) = wide.last() {
        match last {
            32 | 9..=13 => wide = &wide[..wide.len() - 1],
            _ => break,
        }
    }
    wide
}
