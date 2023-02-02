mod imp;
use super::*;

// Generates an in-memory .winmd file.
pub fn write(name: &str, winrt: bool, items: &[Item], assemblies: &[&str]) -> Vec<u8> {
    imp::write(name, winrt, items, assemblies)
}

pub enum Item {
    Struct(Struct),
    Enum(Enum),
    Interface(Interface),
}

pub struct Struct {
    pub namespace: String,
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct Enum {
    pub namespace: String,
    pub name: String,
    pub constants: Vec<Constant>,
}

pub struct Interface {
    pub namespace: String,
    pub name: String,
    pub methods: Vec<Method>,
}

pub struct Field {
    pub name: String,
    pub ty: Type,
}

pub struct Constant {
    pub name: String,
    pub value: Value,
}

pub struct Method {
    pub name: String,
    pub return_type: Type,
    pub params: Vec<Param>,
}

pub struct Param {
    pub name: String,
    pub ty: Type,
    pub flags: ParamFlags,
}

flags!(ParamFlags, u32);
impl ParamFlags {
    pub const INPUT: Self = Self(0x1);
    pub const OUTPUT: Self = Self(0x2);
    pub const OPTIONAL: Self = Self(0x10);
}

pub enum Type {
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
    Named((String, String)),
}

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
}
