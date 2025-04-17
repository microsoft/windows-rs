use super::*;

#[derive(Debug, PartialEq)]
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
    Utf8(String),
    Utf16(String),
    AttributeEnum(String, i32),
}

impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::Bool(..) => Type::Bool,
            Self::U8(..) => Type::U8,
            Self::I8(..) => Type::I8,
            Self::U16(..) => Type::U16,
            Self::I16(..) => Type::I16,
            Self::U32(..) => Type::U32,
            Self::I32(..) => Type::I32,
            Self::U64(..) => Type::U64,
            Self::I64(..) => Type::I64,
            Self::F32(..) => Type::F32,
            Self::F64(..) => Type::F64,
            Self::Utf8(..) => Type::String,
            Self::Utf16(..) => Type::String,
            Self::AttributeEnum(..) => Type::AttributeEnum,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(value) => write!(f, "{value}"),
            Self::U8(value) => write!(f, "{value}"),
            Self::I8(value) => write!(f, "{value}"),
            Self::U16(value) => write!(f, "{value}"),
            Self::I16(value) => write!(f, "{value}"),
            Self::U32(value) => write!(f, "{value}"),
            Self::I32(value) => write!(f, "{value}"),
            Self::U64(value) => write!(f, "{value}"),
            Self::I64(value) => write!(f, "{value}"),
            Self::F32(value) => write!(f, "{value}"),
            Self::F64(value) => write!(f, "{value}"),
            Self::Utf8(value) => write!(f, "{value}"),
            Self::Utf16(value) => write!(f, "{value}"),
            rest => panic!("{rest:?}"),
        }
    }
}
