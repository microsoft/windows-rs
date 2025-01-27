#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![allow(non_upper_case_globals)]

mod cursor;
mod cursor_kind;
mod index;
mod owned;
mod string;
mod tu;
mod visitor;

pub use cursor::*;
pub use cursor_kind::*;
pub use index::*;
pub use owned::*;
pub use string::*;
pub use tu::*;
pub use visitor::*;

use windows_link::*;

pub fn version() -> Owned<CXString> {
    link!("libclang.dll" "system" fn clang_getClangVersion() -> CXString);
    unsafe { Owned::new(clang_getClangVersion()) }
}

#[test]
fn test() {
    assert!(version().starts_with("clang version"));
}
