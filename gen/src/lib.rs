mod codes;
mod error;
mod file;
mod flags;
mod generic_guard;
mod helpers;
mod namespace;
mod reader;
mod row;
mod rust_writer;
mod signatures;
mod tables;
mod writer;

pub use codes::*;
pub use error::*;
pub use file::*;
pub use flags::*;
pub use generic_guard::*;
pub use helpers::*;
pub use namespace::*;
pub use reader::*;
pub use row::*;
pub use rust_writer::*;
pub use signatures::*;
pub use tables::*;
pub use writer::*;

pub(crate) trait Code {
    fn decode(code: u32, file: u16) -> Self;
    fn encode(&self) -> u32;
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";



