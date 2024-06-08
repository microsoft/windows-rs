/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(
    windows_debugger_visualizer,
    debugger_visualizer(natvis_file = "../.natvis")
)]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(not(windows), allow(unused_imports))]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

mod bindings;
use bindings::*;

#[cfg(windows)]
mod com;
#[cfg(windows)]
use com::*;

#[cfg(windows)]
mod strings;
#[cfg(windows)]
use strings::*;

mod error;
pub use error::Error;

mod hresult;
pub use hresult::HRESULT;

/// A specialized [`Result`] type that provides Windows error information.
pub type Result<T> = core::result::Result<T, Error>;
