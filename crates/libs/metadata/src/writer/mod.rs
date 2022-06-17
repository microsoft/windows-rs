mod blobs;
mod codes;
mod gen;
mod helpers;
pub mod pe;
mod strings;
mod tables;
mod r#type;
mod type_name;

use super::*;
use blobs::*;
use codes::*;
pub use gen::*;
pub use helpers::*;
pub use r#type::*;
use strings::*;
pub use tables::*;
pub use type_name::*;

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
    TypeDef(TypeName),
}

impl Default for Value {
    fn default() -> Self {
        Self::Bool(false)
    }
}
