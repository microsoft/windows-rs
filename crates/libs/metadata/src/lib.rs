#![doc = include_str!("../readme.md")]
#![expect(non_snake_case, non_upper_case_globals, non_camel_case_types)]

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

pub use reader::{AsRow, HasAttributes};

pub fn trim_tick(name: &str) -> &str {
    if let Some(pos) = name.find('`') {
        &name[..pos]
    } else {
        name
    }
}
