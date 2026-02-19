#![doc = include_str!("../readme.md")]
#![allow(dead_code)]

mod error;
mod formatter;
mod reader;
mod syntax;
mod writer;

use std::collections::BTreeMap;
use syn::spanned::Spanned;

pub use error::Error;
pub use reader::Reader;
pub use writer::Writer;
