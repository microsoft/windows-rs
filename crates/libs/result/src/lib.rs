#![doc = include_str!("../readme.md")]
#![debugger_visualizer(natvis_file = "../windows-result.natvis")]
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

mod bool;
pub use bool::BOOL;

// TODO: define HResult as Result<T, HRESULT> and then Result for compat and maybe deprecate
//pub type Result<T> = Result<T, Error>;
//pub type HResult<T> = Result<T, HRESULT>;
