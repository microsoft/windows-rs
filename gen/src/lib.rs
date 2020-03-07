mod codes;
mod error;
mod file;
mod flags;
mod helpers;
mod namespace;
mod reader;
mod row;
mod rust_writer;
mod signatures;
mod tables;
mod writer;

use codes::*;
use error::*;
use file::*;
use flags::*;
use helpers::*;
use namespace::*;
use reader::*;
use row::*;
pub use rust_writer::*;
use signatures::*;
use tables::*;
use writer::*;

pub(crate) trait Code {
    fn decode(code: u32, file: u16) -> Self;
    fn encode(&self) -> u32;
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
