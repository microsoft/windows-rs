use super::*;

#[derive(Default)]
pub struct Param {
}

impl Param {
    pub const ID: u64 = 1 << 0x08;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
