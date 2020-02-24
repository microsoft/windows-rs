mod file;
mod flags;
mod reader;
mod row;
mod tables;

pub(crate) use reader::Reader;
pub(crate) use row::{Row, RowData, RowIterator};
pub(crate) use tables::*;

pub(crate) trait Code {
    fn decode(code: u32, file: u16) -> Self;
    fn encode(&self) -> u32;
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
