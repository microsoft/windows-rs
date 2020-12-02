/*!
Rust/WinRT follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt)
of building language projections for the Windows Runtime using standard languages and compilers,
providing a natural and idiomatic way for Rust developers to call Windows APIs. Rust/WinRT lets you
call any WinRT API past, present, and future using code generated on the fly directly from the
metadata describing the API and right into your Rust package where you can call them as if they were
just another Rust module.

The Windows Runtime is based on Component Object Model (COM) APIs under the hood and is designed to
be accessed through language projections like C++/WinRT and Rust/WinRT. Those language projections
take the metadata describing various APIs and provide natural bindings for the target programming
language. As you can imagine, this allows developers to more easily build apps and components for
Windows using their desired language. You can then use those Windows APIs to build desktop apps,
store apps, or something more unique like a component, NT service, or device driver.
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
use strings::*;

pub use interfaces::{IActivationFactory, IAgileObject, IUnknown, Object};
pub use result::{Error, ErrorCode, Result};
pub use runtime::{Array, FactoryCache, Guid, Param, RefCount, Waiter};
pub use strings::HString;
pub use traits::{Abi, Interface, RuntimeName, RuntimeType};
pub use winrt_macros::{build, implement};

extern crate self as winrt;

mod bindings {
    include_bindings!();
}

#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;

#[doc(hidden)]
pub use bindings::windows::foundation;

#[doc(hidden)]
pub use const_sha1::ConstBuffer;
