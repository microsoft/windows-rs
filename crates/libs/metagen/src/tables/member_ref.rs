use super::*;

pub struct MemberRef {
}

impl MemberRef {
    pub const ID: u64 = 1 << 0x0A;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
