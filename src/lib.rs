extern crate self as winrt;

#[macro_export]
macro_rules! include_bindings {
    () => {
        include!(concat!(env!("OUT_DIR"), "\\winrt.rs"));
    };
}

mod bindings {
    #![allow(unused_variables)]
    include_bindings!();
}

#[doc(hidden)]
pub use bindings::windows::foundation;

pub type RawPtr = *mut std::ffi::c_void;
pub type RawComPtr = std::ptr::NonNull<std::ffi::c_void>;

#[doc(hidden)]
pub use const_sha1::ConstBuffer;

mod interfaces;
mod result;
mod runtime;
mod strings;
mod traits;

use interfaces::*;
use runtime::*;
use strings::*;

pub use interfaces::{IActivationFactory, IAgileObject, IUnknown, Object};
pub use result::{Error, ErrorCode, Result};
pub use runtime::{Array, FactoryCache, Guid, Param, RefCount, Waiter};
pub use strings::HString;
pub use traits::{Abi, Interface, RuntimeName, RuntimeType};
pub use winrt_macros::{build, implement};
