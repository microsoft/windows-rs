use std::collections::*;
pub mod reader;
pub mod reader2;
pub mod writer;
use std::mem::*;
use std::slice::*;
use std::ptr::*;

#[repr(C)]
#[derive(Default)]
struct DosHeader {
    magic: u16,
    cblp: u16,
    cp: u16,
    crlc: u16,
    cparhdr: u16,
    minalloc: u16,
    maxalloc: u16,
    ss: u16,
    sp: u16,
    csum: u16,
    ip: u16,
    cs: u16,
    lfarlc: u16,
    ovno: u16,
    res: [u16; 4],
    oemid: u16,
    oeminfo: u16,
    res2: [u16; 10],
    lfanew: i32,
}

impl DosHeader {
    fn new() -> Self {
        Self {
            magic: 0x5A4D,                  // MZ
            lfarlc: 64,                     // file address of relocation table
            lfanew: size_of::<Self>() as _, // file address of next header
            ..Default::default()
        }
    }
}
