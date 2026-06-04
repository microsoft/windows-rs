#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]

#[expect(missing_docs, non_snake_case, clippy::upper_case_acronyms)]
mod bindings;
mod datetime;
mod timespan;

use bindings::*;
pub use bindings::{DateTime, TimeSpan};
pub use timespan::TimeRangeError;
use timespan::*;
