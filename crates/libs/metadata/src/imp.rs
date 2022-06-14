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

extern "C" {
    pub fn strlen(cs: *const u8) -> usize;
}

pub fn composite_index_size(tables: &[usize]) -> usize {
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
