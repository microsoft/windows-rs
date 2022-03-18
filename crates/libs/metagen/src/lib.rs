// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format

mod pe;
use std::collections::*;
use std::mem::*;
use std::slice::*;

pub fn test() {
    let mut strings = Strings::default();
    strings.0.insert("hello".to_string());
    strings.0.insert("world".to_string());
    let blobs = Blobs::default();
    let tables = Tables::default();

    pe::write("/git/test.winmd", &strings, &blobs, &tables);
}

fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

trait Write {
    fn write<T: Sized>(&mut self, value: &T);
}

impl Write for Vec<u8> {
    fn write<T: Sized>(&mut self, value: &T) {
        unsafe {
            self.extend_from_slice(from_raw_parts(value as *const _ as _, size_of::<T>()));
        }
    }
}

#[derive(Default)]
struct Strings(BTreeSet<String>);

impl Strings {
    fn stream(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(0); // start with empty string

        for value in &self.0 { 
            buffer.extend_from_slice(value.as_bytes());
            buffer.push(0); // terminator
        }

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}

#[derive(Default)]
struct Blobs(BTreeSet<Vec<u8>>);

impl Blobs {
    fn stream(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(0); // start with zero byte

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}

#[derive(Default)]
struct Tables {
    // TODO: just use fixed 4 byte index sizes
}

impl Tables {
    fn stream(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let header = TableStreamHeader::new();
        buffer.write(&header);

        // row sizes
        buffer.write(&1u32); // Module

        // Module table
        buffer.write(&0u16); // Generation (reserved)
        buffer.write(&0u32); // Name
        buffer.write(&1u32); // Mvid
        buffer.write(&0u32); // EncId (reserved)
        buffer.write(&0u32); // EncBaseId (reserved)

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}

#[repr(C)]
#[derive(Default)]
struct TableStreamHeader {
    reserved1: u32,
    major_version: u8,
    minor_version: u8,
    heap_sizes: u8,
    reserved2: u8,
    valid: u64,
    sorted: u64,
}

impl TableStreamHeader {
    fn new() -> Self {
        Self { major_version: 2, reserved2: 1, heap_sizes: 0b111, valid: 0b1, ..Default::default() }
    }
}
