use super::*;

#[derive(Default)]
pub struct ModuleRef {
}

impl ModuleRef {
    pub const ID: u64 = 1 << 0x1A;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
