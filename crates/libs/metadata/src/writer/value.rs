use super::*;

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

impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::Bool(_) => Type::Bool,
            Self::U8(_) => Type::U8,
            Self::I8(_) => Type::I8,
            Self::U16(_) => Type::U16,
            Self::I16(_) => Type::I16,
            Self::U32(_) => Type::U32,
            Self::I32(_) => Type::I32,
            Self::U64(_) => Type::U64,
            Self::I64(_) => Type::I64,
            Self::F32(_) => Type::F32,
            Self::F64(_) => Type::F64,
            Self::String(_) => Type::String,
            Self::TypeDef(value) => Type::TypeDef((value.clone(), Vec::new())),
        }
    }

    pub fn to_blob(&self) -> Vec<u8> {
        match self {
            Self::U8(value) => value.to_le_bytes().into(),
            Self::I8(value) => value.to_le_bytes().into(),
            Self::U16(value) => value.to_le_bytes().into(),
            Self::I16(value) => value.to_le_bytes().into(),
            Self::U32(value) => value.to_le_bytes().into(),
            Self::I32(value) => value.to_le_bytes().into(),
            Self::U64(value) => value.to_le_bytes().into(),
            Self::I64(value) => value.to_le_bytes().into(),
            Self::F32(value) => value.to_le_bytes().into(),
            Self::F64(value) => value.to_le_bytes().into(),
            _ => unimplemented!(),
        }
    }
}
