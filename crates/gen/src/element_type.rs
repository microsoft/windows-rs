use crate::blob::Blob;
use crate::codes::{Decode, TypeDefOrRef};

#[derive(Debug)]
pub enum ElementType {
    Void,
    Boolean,
    Char,
    I1,
    U1,
    I2,
    U2,
    I4,
    U4,
    I8,
    U8,
    R4,
    R8,
    String,
    ValueType(TypeDefOrRef),
    Class(TypeDefOrRef),
}

impl ElementType {
    pub fn from_blob(blob: &mut Blob) -> ElementType {
        let code = blob.read_unsigned();
        match code {
            0x01 => ElementType::Void,
            0x02 => ElementType::Boolean,
            0x03 => ElementType::Char,
            0x04 => ElementType::I1,
            0x05 => ElementType::U1,
            0x06 => ElementType::I2,
            0x07 => ElementType::U2,
            0x08 => ElementType::I4,
            0x09 => ElementType::U4,
            0x0a => ElementType::I8,
            0x0b => ElementType::U8,
            0x0c => ElementType::R4,
            0x0d => ElementType::R8,
            0x0e => ElementType::String,
            0x11 => {
                ElementType::ValueType(TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index))
            }
            0x12 => ElementType::Class(TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index)),

            unknown_type => panic!(format!("Unexpected ElementType: {:x}", unknown_type)),
        }
    }
}
