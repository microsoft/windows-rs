use super::*;

#[derive(Default)]
pub struct ImplMap {
}

impl ImplMap {
    pub const ID: u64 = 1 << 0x1C;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
