use super::*;

pub struct ClassLayout {
}

impl ClassLayout {
    pub const ID: u64 = 1 << 0x0F;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
