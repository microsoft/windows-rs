use super::*;

#[derive(Default)]
struct GenericParam{

}

impl GenericParam {
    pub const ID: u64 = 1 << 0x2A;

    pub fn write(&self, _buffer: &mut Vec<u8>, _strings: &mut Strings) {
    }
}
