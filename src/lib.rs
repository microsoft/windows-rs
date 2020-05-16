//! A crate for providing native and familiar support for the Windows Runtime for Rust developers.
//!
//! To use, start by importing the modules that you need.
//!
//! ```rust
//! use winrt::import;
//!
//! import!(
//!     dependencies
//!         os
//!     modules
//!         "windows.foundation"
//!         "windows.ui"
//! );
//!
//! // Make use of any WinRT APIs as needed.
//! // For example, here is an example of using the Windows.Foundation.Uri class:
//!
//! fn main() -> winrt::Result<()> {
//!     use windows::foundation::Uri;
//!
//!     let uri = Uri::create_uri("http://kennykerr.ca")?;
//!     println!("domain: {}", uri.domain()?);
//!     println!("port: {}", uri.port()?);
//!     println!("string: {}", uri.to_string()?);
//!
//!     Ok(())
//! }
//! ```
//!
//! This program will print the following output:
//!
//! ```text
//! domain: kennykerr.ca
//! port: 80
//! string: http://kennykerr.ca/
//! ```

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod activation_factory;
mod agile_object;
mod array;
mod com_interface;
mod com_ptr;
mod error;
#[doc(hidden)]
pub mod factory;
mod guid;
mod hstring;
mod object;
mod param;
mod ref_count;
pub mod runtime;
mod runtime_name;
mod runtime_type;
mod try_into;
mod unknown;

#[doc(inline)]
pub use activation_factory::IActivationFactory;
pub use agile_object::IAgileObject;
pub use array::Array;
pub use com_interface::{ComInterface, RawComPtr};
pub use com_ptr::ComPtr;
pub use error::*;
pub use factory::factory;
pub use guid::Guid;
pub use hstring::HString;
pub use object::Object;
pub use param::Param;
pub use ref_count::RefCount;
pub use runtime_name::RuntimeName;
pub use runtime_type::RuntimeType;
pub use try_into::TryInto;
pub use unknown::IUnknown;
pub use winrt_macros::import;

/// A convenient alias of a void pointer
pub type RawPtr = *mut std::ffi::c_void;
