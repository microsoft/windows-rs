mod blob;
mod codes;
mod file;
mod flags;
mod reader;
mod row;
mod tables;
mod type_limits;
mod type_namespaces;
mod type_stage;
mod type_tree;
mod types;

use blob::*;
use codes::*;
use file::*;
use flags::*;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
pub use reader::*;
use row::*;
use std::collections::btree_map::*;
use std::collections::*;
use std::convert::TryInto;
use std::iter::FromIterator;
pub use tables::*;
pub use type_limits::*;
pub use type_namespaces::*;
pub use type_stage::*;
pub use type_tree::*;
pub use types::*;
use winmd_macros::*;

pub fn write_ident(name: &str) -> Ident {
    if name == "Self" {
        format_ident!("{}_", name)
    } else {
        format_ident!("r#{}", name)
    }
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
