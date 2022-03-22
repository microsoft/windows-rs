use super::*;

#[derive(Default)]
struct MethodDef {
}

impl MethodDef {
    pub const ID: u64 = 1 << 0x06;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
        todo!()
    }
}
