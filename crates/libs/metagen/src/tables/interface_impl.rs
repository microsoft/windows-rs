use super::*;

#[derive(Default)]
pub struct InterfaceImpl {
}

impl InterfaceImpl {
    pub const ID: u64 = 1 << 0x09;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
