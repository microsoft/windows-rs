#![warn(unused_qualifications)]
#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]

#[allow(dead_code)]
#[expect(non_snake_case, clippy::upper_case_acronyms)]
mod bindings;
mod datetime;
mod timespan;

#[cfg(windows)]
use bindings::*;
pub use bindings::{DateTime, TimeSpan};
pub use timespan::TimeRangeError;
use timespan::*;
