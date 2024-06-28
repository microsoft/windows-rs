/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![allow(non_snake_case)]
#![cfg_attr(
    windows_debugger_visualizer,
    debugger_visualizer(natvis_file = "../.natvis")
)]
#![cfg_attr(all(not(feature = "std")), no_std)]

extern crate alloc;
use alloc::string::String;

pub use windows_result::Result;
use windows_result::*;

mod bstr;
pub use bstr::*;

mod hstring;
pub use hstring::*;

mod bindings;

mod decode;
use decode::*;

mod ref_count;
use ref_count::*;

mod heap;
use heap::*;

mod literals;
pub use literals::*;

mod pcstr;
pub use pcstr::*;

mod pcwstr;
pub use pcwstr::*;

mod pstr;
pub use pstr::*;

mod pwstr;
pub use pwstr::*;

mod traits;
pub use traits::*;

extern "C" {
    fn strlen(s: PCSTR) -> usize;
}
