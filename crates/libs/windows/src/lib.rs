/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(feature = "implement", feature(const_fn_fn_ptr_basics))]
#![cfg_attr(feature = "implement", feature(const_fn_trait_bound))]
#![doc(html_no_source)]

extern crate self as windows;
mod Windows;
pub mod core;
pub use Windows::*;
