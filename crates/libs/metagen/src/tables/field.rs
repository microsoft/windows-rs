use super::*;

pub struct Field {
}

impl Field{
    pub const ID: u64 = 1 << 0x04;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
