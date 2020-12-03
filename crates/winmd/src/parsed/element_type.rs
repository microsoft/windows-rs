use super::*;
use crate::traits::Decode;

#[derive(Debug)]
pub enum ElementType {
    Void,
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    USize,
    String,
    Struct(TypeDefOrRef),
    Class(TypeDefOrRef),
}

impl ElementType {
    pub fn from_blob(blob: &mut Blob) -> ElementType {
        let code = blob.read_unsigned();
        match code {
            0x01 => ElementType::Void,
            0x02 => ElementType::Bool,
            0x03 => ElementType::Char,
            0x04 => ElementType::I8,
            0x05 => ElementType::U8,
            0x06 => ElementType::I16,
            0x07 => ElementType::U16,
            0x08 => ElementType::I32,
            0x09 => ElementType::U32,
            0x0a => ElementType::I64,
            0x0b => ElementType::U64,
            0x0c => ElementType::F32,
            0x0d => ElementType::F64,
            0x18 => ElementType::ISize,
            0x19 => ElementType::USize,
            0x0e => ElementType::String,
            0x11 => ElementType::Struct(TypeDefOrRef::decode(
                blob.reader,
                blob.read_unsigned(),
                blob.file_index,
            )),
            0x12 => ElementType::Class(TypeDefOrRef::decode(
                blob.reader,
                blob.read_unsigned(),
                blob.file_index,
            )),

            unknown_type => panic!(format!("Unexpected ElementType: {:x}", unknown_type)),
        }
    }

    pub fn from_code(code: u32) -> ElementType {
        match code {
            0x01 => ElementType::Void,
            0x02 => ElementType::Bool,
            0x03 => ElementType::Char,
            0x04 => ElementType::I8,
            0x05 => ElementType::U8,
            0x06 => ElementType::I16,
            0x07 => ElementType::U16,
            0x08 => ElementType::I32,
            0x09 => ElementType::U32,
            0x0a => ElementType::I64,
            0x0b => ElementType::U64,
            0x0c => ElementType::F32,
            0x0d => ElementType::F64,
            0x18 => ElementType::ISize,
            0x19 => ElementType::USize,
            0x0e => ElementType::String,
            unknown_code => panic!(format!("Unexpected ElementType: {:x}", unknown_code)),
        }
    }
}
