#![allow(dead_code)]

use std::collections::*;
pub mod reader;
pub mod reader2;
pub mod writer;

use std::io::*;
use std::mem::*;
use std::ptr::*;

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

extern "C" {
    fn strlen(cs: *const u8) -> usize;
}

fn composite_index_size(tables: &[usize]) -> usize {
    fn small(row_count: usize, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        loop {
            value >>= 1;
            if value == 0 {
                break;
            }
            bits += 1;
        }
        bits
    }

    let bits_needed = bits_needed(tables.len());

    if tables.iter().all(|table| small(*table, bits_needed)) {
        2
    } else {
        4
    }
}
