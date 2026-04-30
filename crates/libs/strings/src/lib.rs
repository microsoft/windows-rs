#![doc = include_str!("../readme.md")]
#![cfg_attr(windows, debugger_visualizer(natvis_file = "../windows-strings.natvis"))]
#![cfg_attr(all(not(feature = "std")), no_std)]

// The `PCSTR`/`PCWSTR`/`PSTR`/`PWSTR` pointer types, the `s!`/`w!` literal
// macros and the UTF-8/UTF-16 `decode` helpers are portable: they only need
// `core`/`alloc` plus libc's `strlen`/`wcslen`. They build on every target.
//
// `HSTRING`, `BSTR`, the `h!` literal macro, and the helpers backing them
// (`hstring_builder`, `hstring_header`, `ref_count`, `bindings`) are
// inherently Win32 — they call `WindowsCreateString*`, `SysAllocString*`,
// and use the Windows process heap — and therefore stay gated to Windows.

extern crate alloc;
use alloc::string::String;

#[cfg(windows)]
mod bstr;
#[cfg(windows)]
pub use bstr::*;

#[cfg(windows)]
mod hstring;
#[cfg(windows)]
pub use hstring::*;

#[cfg(windows)]
mod hstring_builder;
#[cfg(windows)]
pub use hstring_builder::*;

#[cfg(windows)]
mod hstring_header;
#[cfg(windows)]
use hstring_header::*;

#[cfg(windows)]
mod bindings;

mod decode;
use decode::*;

#[cfg(windows)]
mod ref_count;
#[cfg(windows)]
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
