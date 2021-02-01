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
pub use runtime::{factory, Array, FactoryCache, Guid, Param, RefCount, Waiter};
pub use strings::{BString, HString};
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
