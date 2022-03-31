use std::collections::*;
pub mod reader;
pub mod reader2;
pub mod writer;
use std::mem::*;
use std::ptr::*;
use std::slice::*;
use std::io::*;

#[repr(C)]
#[derive(Default)]
struct METADATA_HEADER {
    signature: u32,
    major_version: u16,
    minor_version: u16,
    reserved: u32,
    length: u32,
    version: [u8; 20],
    flags: u16,
    streams: u16,
}

const METADATA_SIGNATURE: u32 = 0x424A_5342;
