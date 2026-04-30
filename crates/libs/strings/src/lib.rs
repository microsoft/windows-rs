#![doc = include_str!("../readme.md")]
#![cfg_attr(windows, debugger_visualizer(natvis_file = "../windows-strings.natvis"))]
#![cfg_attr(all(not(feature = "std")), no_std)]

// `windows-strings` is fully cross-platform: every type compiles on every
// target. The `HSTRING` / `BSTR` allocators are routed through the `bindings`
// module, which on Windows calls into the kernel32 process heap and oleaut32
// BSTR allocator (so strings interoperate with native code) and on other
// targets is serviced by the Rust global allocator using a layout that
// matches the public Win32 contract.

extern crate alloc;
use alloc::string::String;

mod bstr;
pub use bstr::*;

mod hstring;
pub use hstring::*;

mod hstring_builder;
pub use hstring_builder::*;

mod hstring_header;
use hstring_header::*;

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
