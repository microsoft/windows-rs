#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, forget_copy, clippy::all)]
#[cfg(feature = "Wdk")]
pub mod Wdk;
#[cfg(feature = "Win32")]
pub mod Win32;
