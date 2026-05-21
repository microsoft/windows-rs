#![expect(missing_docs, non_snake_case)]
#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]
#![forbid(unsafe_code)]

mod bindings;
pub use bindings::*;

mod datetime;
mod timespan;

pub use timespan::TimeRangeError;
