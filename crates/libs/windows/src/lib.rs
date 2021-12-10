/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![doc(html_no_source)]

extern crate self as windows;
mod Windows;
pub mod core;
pub use Windows::*;
