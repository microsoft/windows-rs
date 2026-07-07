#![doc = include_str!("../readme.md")]
#![cfg_attr(
    windows,
    debugger_visualizer(natvis_file = "../windows-strings.natvis")
)]
#![cfg_attr(all(not(feature = "std")), no_std)]

// `HSTRING` and its supporting types are cross-platform: the only Win32 dependency is the
// heap allocator, which `hstring_header` swaps at compile time. On Windows it uses the
// process heap (so HSTRINGs interoperate with native code); elsewhere it uses the Rust
// global allocator.
//
// `BSTR` and the `bindings` module are inherently Win32 — `BSTR` is part of the OLE
// Automation ABI and must use `SysAllocStringLen`/`SysFreeString` so callers across the
// FFI boundary can free strings allocated here — and therefore stay gated to Windows.

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
#[expect(clippy::upper_case_acronyms)]
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
    fn strlen(s: *const core::ffi::c_char) -> usize;
}
