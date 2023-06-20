mod bindings;
pub use bindings::*;

#[repr(C)]
#[derive(Default)]
pub struct METADATA_HEADER {
    pub signature: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub reserved: u32,
    pub length: u32,
    pub version: [u8; 20],
    pub flags: u16,
    pub streams: u16,
}

pub const METADATA_SIGNATURE: u32 = 0x424A_5342;

/// A coded index (see codes.rs) is a table index that may refer to different tables. The size of the column in memory
/// must therefore be large enough to hold an index for a row in the largest possible table. This function determines
/// this size for the given winmd file.
pub fn coded_index_size(tables: &[usize]) -> usize {
    fn small(row_count: usize, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        while {
            value >>= 1;
            value != 0
        } {
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
