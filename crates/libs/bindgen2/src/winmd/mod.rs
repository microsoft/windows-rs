use super::*;

mod attributes;
mod bindings;
mod blob;
mod codes;
mod dependencies;
mod file;
mod item;
mod ord;
mod reader;
mod row;
mod signature;
mod tables;
mod r#type;
mod type_name;

pub use attributes::*;
use bindings::*;
use blob::*;
pub use codes::*;
pub use dependencies::*;
pub use file::*;
pub use item::*;
pub use r#type::*;
pub use reader::*;
pub use row::*;
pub use signature::*;
pub use tables::*;
pub use type_name::*;

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
    EnumDef(&'static Item, Box<Self>),
}

pub struct GUID(
    pub u32,
    pub u16,
    pub u16,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
);

impl std::fmt::Display for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10
        )
    }
}