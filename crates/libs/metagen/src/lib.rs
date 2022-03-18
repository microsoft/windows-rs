// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format

mod pe;
use std::collections::*;
use std::mem::*;
use std::slice::*;

pub fn test() {
    let mut strings = BTreeSet::<String>::new();
    strings.insert("mscorlib".to_string());
    let blobs = BTreeSet::<Vec<u8>>::new();
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
struct Tables {}

impl Tables {
    // TODO: must refer to the strings/blobs containers to get their relative offsets...
    fn stream(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let header = TableStreamHeader::new();
        buffer.write(&header);

        // row sizes
        buffer.write(&1u32); // Module
        buffer.write(&1u32); // AssemblyRef

        // Module table
        buffer.write(&0u16); // Generation (reserved)
        buffer.write(&0u32); // Name (none)
        buffer.write(&1u32); // Mvid (zero guid)
        buffer.write(&0u32); // EncId (reserved)
        buffer.write(&0u32); // EncBaseId (reserved)

        // AssemblyRef table
        // mscorelib entry only required by ILSpy
        buffer.write(&0u16); // MajorVersion
        buffer.write(&0u16); // MinorVersion
        buffer.write(&0u16); // BuildNumber
        buffer.write(&0u16); // RevisionNumber
        buffer.write(&0u32); // Flags (none)
        buffer.write(&0u32); // PublicKeyOrToken (none)
        buffer.write(&1u32); // Name (mscorelib) <-- TODO: this needs to point to an offset in the strings stream
        buffer.write(&0u32); // Culture (none)
        buffer.write(&0u32); // HashValue (none)

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}

// TODO: push into pe.rs and abstract this lib to deal only with the logical metadata
// in a layout independent way
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
        Self {
            major_version: 2,
            reserved2: 1,
            heap_sizes: 0b111, // 4 byte indexes
            valid: TableId::Module as u64 | TableId::AssemblyRef as u64,
            ..Default::default()
        }
    }
}

// TODO: just use constants
enum TableId {
    Module = 1 << 0,
    AssemblyRef = 1 << 0x23,
}
