/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(feature = "use_raw_dylib", feature(raw_dylib))]
#![no_std]
#![doc(html_no_source)]

extern crate self as windows_sys;
mod Windows;
pub mod core;
pub use Windows::*;
