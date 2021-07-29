#![doc = include_str!("../.github/readme.md")]

#[macro_use]
mod macros;

mod bindings;
mod interfaces;
mod result;
mod runtime;
mod traits;

use interfaces::*;
use runtime::*;

#[doc(hidden)]
pub use bindings::Windows::Win32::System::Com::IAgileObject;

#[doc(hidden)]
pub use interfaces::{IActivationFactory, IInspectable_abi};

pub use interfaces::{IInspectable, IUnknown};
pub use result::{Error, Result, HRESULT};
pub use runtime::{
    factory, Array, FactoryCache, Guid, Param, RefCount, Waiter, Weak, WeakRefCount, HSTRING,
};
pub use traits::*;

#[cfg(feature = "macros")]
pub use windows_macros::{build, implement};

extern crate self as windows;

// TODO: rather than hiding, consider just removing
#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;

#[doc(hidden)]
pub use const_sha1::ConstBuffer;
