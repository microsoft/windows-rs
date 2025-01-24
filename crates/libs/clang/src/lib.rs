#![doc = include_str!("../readme.md")]
#![cfg(windows)]

mod imports;
mod owned;
mod string;

use imports::*;
use owned::*;
use std::ffi::*;
use std::fmt::*;
use std::ops::*;
use string::*;
use windows_link::*;

pub fn version() -> Owned<CXString> {
    unsafe { Owned::new(clang_getClangVersion()) }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(*version(), "clang version 19.1.7");
    }
}
