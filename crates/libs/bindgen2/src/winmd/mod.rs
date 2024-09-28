use super::*;

mod attributes;
mod bindings;
mod blob;
mod codes;
mod file;
mod item;
mod reader;
mod row;
mod tables;
mod r#type;
mod type_name;

pub use attributes::*;
use bindings::*;
use blob::*;
pub use codes::*;
pub use file::*;
pub use item::*;
pub use r#type::*;
pub use reader::*;
pub use row::*;
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
    EnumDef(TypeDef, Box<Self>),
}

#[derive(Debug)]
pub struct Signature {
    pub call_flags: MethodCallAttributes,
    pub return_type: (Type, Option<Param>),
    pub params: Vec<(Type, Param)>,
}

impl Signature {
    // pub fn size(&self) -> usize {
    //     self.params
    //         .iter()
    //         .fold(0, |sum, param| sum + std::cmp::max(4, param.0.size()))
    // }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum TypeKind {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
}

// TODO: for primitive dependencies (e.g. HSTRING, GUID, IUnknown) use the root namespace of ""
// so that standalone code generation can generate them as needed.
type Dependencies = HashMap<&'static str, HashSet<&'static str>>;