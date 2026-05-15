#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]
#![allow(
    missing_docs,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::missing_transmute_annotations
)]

#[cfg(windows)]
mod bindings;
#[cfg(windows)]
pub use bindings::*;

#[cfg(all(windows, feature = "std"))]
mod reference;

#[cfg(all(windows, feature = "std"))]
const E_NOTIMPL: windows_core::HRESULT = windows_core::HRESULT(0x80004001_u32 as _);
