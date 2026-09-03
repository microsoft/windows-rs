#![doc = include_str!("../readme.md")]
#![expect(non_snake_case, non_upper_case_globals)]
#![allow(missing_docs)]

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

mod clr;
use clr::*;

mod ty;
pub use ty::*;

mod type_name;
pub use type_name::*;

mod signature;
pub use signature::*;

pub use reader::{AsRow, HasAttributes};

/// Metadata merge and namespace remapping support.
pub mod merge;

/// Creates a builder for combining winmd files.
pub fn merge() -> merge::Merger {
    merge::Merger::new()
}

/// Creates a [`merge::Remapper`] that rewrites a flat winmd into a header-based namespace
/// partition for `--package` generation.
pub fn remap() -> merge::Remapper {
    merge::Remapper::new()
}

/// Removes the generic arity suffix beginning with a backtick.
pub fn trim_tick(name: &str) -> &str {
    if let Some(pos) = name.find('`') {
        &name[..pos]
    } else {
        name
    }
}
