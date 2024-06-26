/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(
    windows_debugger_visualizer,
    debugger_visualizer(natvis_file = "../.natvis")
)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(not(windows), allow(unused_imports))]

extern crate alloc;

#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

mod bindings;
use bindings::*;

#[cfg(all(windows, not(windows_slim_errors)))]
mod com;

#[cfg(windows)]
mod strings;
#[cfg(windows)]
use strings::*;

#[cfg(all(windows, not(windows_slim_errors)))]
mod bstr;

mod error;
pub use error::*;

mod hresult;
pub use hresult::HRESULT;

/// A specialized [`Result`] type that provides Windows error information.
pub type Result<T> = core::result::Result<T, Error>;
