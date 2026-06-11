#![warn(unused_qualifications)]
#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]

#[cfg(windows)]
#[expect(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod bindings;

#[cfg(all(windows, feature = "std"))]
mod reference;

#[cfg(all(windows, feature = "std"))]
pub use reference::IReference;
