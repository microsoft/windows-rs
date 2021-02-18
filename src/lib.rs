/*!
The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt)
of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic
way for Rust developers to call Windows APIs. The `windows` crate lets you call any Windows API past, present, and
future using code generated on the fly directly from the metadata describing the API and right into your Rust package
where you can call them as if they were just another Rust module.

Learn more here: [https://github.com/microsoft/windows-rs](https://github.com/microsoft/windows-rs)
*/

#[macro_use]
mod macros;

mod interfaces;
mod result;
mod runtime;
mod strings;
mod traits;

use interfaces::*;
use runtime::*;

pub use interfaces::{IActivationFactory, IAgileObject, IUnknown, Object};
pub use result::{Error, ErrorCode, Result, BOOL, FALSE, TRUE};
pub use runtime::{
    create_instance, factory, initialize_mta, initialize_sta, Array, FactoryCache, Guid, Param,
    RefCount, Waiter,
};
pub use strings::{BString, CoString, HString};
pub use traits::{Abi, Interface, RuntimeName, RuntimeType};
pub use windows_macros::{build, implement};

extern crate self as windows;

mod bindings {
    include_bindings!();
}

#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;

#[doc(hidden)]
pub use bindings::windows::foundation;

#[doc(hidden)]
pub use const_sha1::ConstBuffer;

/// A stand-in for a type which is not yet fully supported by the `windows` crate.
///
/// There should be tracking issues for each one of these types as they will eventually be supported.
/// This type is not constructible as it should never be used. It is merely marking that support
/// needs to be added to the `windows` crate before this functionality becomes available.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(transparent)]
pub struct NOT_YET_SUPPORTED_TYPE {
    /// ensures the type cannot be constructed
    _priv: u8,
}
