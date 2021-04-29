/*!
The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt)
of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic
way for Rust developers to call Windows APIs. The `windows` crate lets you call any Windows API past, present, and
future using code generated on the fly directly from the metadata describing the API and right into your Rust package
where you can call them as if they were just another Rust module.

Learn more here: <https://github.com/microsoft/windows-rs>
*/

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
pub use bindings::Windows::Win32::Com::IAgileObject;

#[doc(hidden)]
pub use interfaces::IActivationFactory;

pub use interfaces::{IInspectable, IUnknown};
pub use result::{Error, Result, HRESULT};
pub use runtime::{
    create_instance, factory, initialize_mta, initialize_sta, Array, FactoryCache, Guid, HString,
    Param, RefCount, Waiter, Weak, WeakRefCount,
};
pub use traits::{Abi, Interface, IntoParam, RuntimeName, RuntimeType};

#[cfg(feature = "macros")]
pub use windows_macros::{build, implement};

extern crate self as windows;

// TODO: rather than hiding, consider just removing
#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;

#[doc(hidden)]
pub use const_sha1::ConstBuffer;
