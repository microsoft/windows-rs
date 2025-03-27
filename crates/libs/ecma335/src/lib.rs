#![doc = include_str!("../readme.md")]
#![allow(non_snake_case, non_upper_case_globals)]

use std::cmp::Ordering;
use std::collections::*;

mod attributes;
pub mod reader;
mod value;
pub mod writer;

pub use attributes::*;
pub use value::*;
mod bindings;
use bindings::*;

mod ty;
pub use ty::*;

mod type_name;
pub use type_name::*;

mod signature;
pub use signature::*;

pub use reader::HasAttributes;

mod merge;
pub use merge::*;

// mod fast_merge;
// pub use fast_merge::*;
