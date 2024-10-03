use super::*;

mod attributes;
mod bindings;
mod blob;
mod codes;
mod dependencies;
mod file;
mod item;
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
