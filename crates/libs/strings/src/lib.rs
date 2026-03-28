#![doc = include_str!("../readme.md")]
#![debugger_visualizer(natvis_file = "../windows-strings.natvis")]
#![cfg_attr(all(not(feature = "std")), no_std)]

extern crate alloc;
use alloc::string::String;

#[cfg(windows)]
mod bstr;
#[cfg(windows)]
pub use bstr::*;

mod hstring;
pub use hstring::*;

mod hstring_builder;
pub use hstring_builder::*;

mod hstring_header;
use hstring_header::*;

#[cfg(windows)]
mod bindings;

mod decode;
use decode::*;

mod ref_count;
use ref_count::*;

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

unsafe extern "C" {
    fn strlen(s: PCSTR) -> usize;
}
