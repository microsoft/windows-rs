use std::cmp::Ordering;
use std::collections::*;

mod attributes;
mod bindings;
mod blob;
mod codes;
mod column;
mod file;
mod filter;
mod reader;
mod row;
mod table;
mod tables;
mod r#type;
mod type_name;

pub use attributes::*;
pub use bindings::*;
pub use blob::*;
pub use codes::*;
use column::*;
pub use file::*;
use filter::*;
pub use r#type::*;
pub use reader::*;
pub use row::*;
use table::*;
pub use tables::*;
pub use type_name::*;

#[repr(C)]
#[derive(Default)]
pub struct METADATA_HEADER {
    pub signature: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub reserved: u32,
    pub length: u32,
    pub version: [u8; 20],
    pub flags: u16,
    pub streams: u16,
}

pub const METADATA_SIGNATURE: u32 = 0x424A_5342;

/// A coded index (see codes.rs) is a table index that may refer to different tables. The size of the column in memory
/// must therefore be large enough to hold an index for a row in the largest possible table. This function determines
/// this size for the given winmd file.
pub fn coded_index_size(tables: &[usize]) -> usize {
    fn small(row_count: usize, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        while {
            value >>= 1;
            value != 0
        } {
            bits += 1;
        }
        bits
    }

    let bits_needed = bits_needed(tables.len());

    if tables.iter().all(|table| small(*table, bits_needed)) {
        2
    } else {
        4
    }
}

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    TypeName(TypeName),
    EnumDef(TypeDef, Box<Self>),
}

pub struct MethodDefSig {
    pub call_flags: MethodCallAttributes,
    pub return_type: Type,
    pub params: Vec<Type>,
}

impl MethodDefSig {
    pub fn size(&self) -> usize {
        self.params.iter().fold(0, |sum, param| sum + std::cmp::max(4, param.size()))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum TypeKind {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
}
