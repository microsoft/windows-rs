use super::*;

pub enum ConstantValue {
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
    TypeDef(TypeDef),
}

impl ConstantValue {
    pub fn unwrap_u32(&self) -> u32 {
        match self {
            Self::U32(value) => *value,
            _ => unimplemented!(),
        }
    }

    pub fn unwrap_u16(&self) -> u16 {
        match self {
            Self::U16(value) => *value,
            _ => unimplemented!(),
        }
    }

    pub fn unwrap_u8(&self) -> u8 {
        match self {
            Self::U8(value) => *value,
            _ => unimplemented!(),
        }
    }

    pub fn unwrap_string(&self) -> &str {
        match self {
            Self::String(value) => value,
            _ => unimplemented!(),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::U32(value) => Self::U32(value + 1),
            Self::I32(value) => Self::I32(value + 1),
            _ => unimplemented!(),
        }
    }
}
