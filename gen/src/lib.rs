mod blob;
mod codes;
mod file;
mod flags;
mod reader;
mod row;
mod tables;
mod types;
mod writer;

use blob::*;
use codes::*;
use file::*;
use flags::*;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
pub use reader::*;
use row::*;
use std::collections::btree_map::*;
use std::convert::TryInto;
use tables::*;
pub use types::*;
use winmd_macros::*;
pub use writer::*;
use std::collections::*;

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
