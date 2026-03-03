#![doc = include_str!("../readme.md")]
#![allow(dead_code)]

mod error;
mod formatter;
mod reader;
mod writer;

use std::collections::{BTreeMap, HashSet};
use syn::spanned::Spanned;

pub use error::Error;
pub use reader::Reader;
pub use writer::Writer;
