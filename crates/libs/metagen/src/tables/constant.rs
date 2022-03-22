use super::*;

#[derive(Default)]
pub struct Constant {
}

impl Constant {
    pub const ID: u64 = 1 << 0x0B;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
