use super::*;

pub struct TypeSpec {
}

impl TypeSpec {
    pub const ID: u64 = 1 << 0x1B;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
