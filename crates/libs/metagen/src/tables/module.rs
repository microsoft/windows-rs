use super::*;

pub struct Module;

impl Module {
    pub const ID: u64 =  1 << 0;

    pub fn write(buffer: &mut Vec<u8>) {
        buffer.write(&0u16); // Generation (reserved)
        buffer.write(&0u32); // Name (none)
        buffer.write(&1u32); // Mvid (zero guid)
        buffer.write(&0u32); // EncId (reserved)
        buffer.write(&0u32); // EncBaseId (reserved)
    }
}


