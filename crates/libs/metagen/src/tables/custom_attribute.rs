use super::*;

#[derive(Default)]
pub struct CustomAttribute {
}

impl CustomAttribute {
    pub const ID: u64 = 1 << 0x0C;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
