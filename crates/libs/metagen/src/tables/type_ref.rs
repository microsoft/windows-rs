use super::*;

#[derive(Default)]
pub struct TypeRef {
}

impl TypeRef {
    pub const ID: u64 = 1 << 0x01;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
