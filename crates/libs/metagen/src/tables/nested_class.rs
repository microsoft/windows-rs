use super::*;

struct NestedClass;

impl NestedClass {
    pub const ID: u64 = 1 << 0x29;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
