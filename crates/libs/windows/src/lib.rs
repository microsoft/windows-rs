/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![doc(html_no_source)]
#![allow(non_snake_case, clashing_extern_declarations)]
#![cfg_attr(windows_debugger_visualizer, feature(debugger_visualizer), debugger_visualizer(natvis_file = "../windows.natvis"))]
#![cfg_attr(windows_raw_dylib, feature(raw_dylib))]

extern crate self as windows;
mod Windows;
pub mod core;
pub use Windows::*;
