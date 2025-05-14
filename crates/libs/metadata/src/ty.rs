use super::*;

#[derive(Debug, PartialEq, Clone)]
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
    Object,
    AttributeEnum, // 0x55 is an unnamed ELEMENT_TYPE used by attributes to specify an enum
    Name(TypeName),
    Array(Box<Self>),    // ELEMENT_TYPE_SZARRAY
    ArrayRef(Box<Self>), // ELEMENT_TYPE_BYREF, ELEMENT_TYPE_SZARRAY
    ConstRef(Box<Self>), // ELEMENT_TYPE_CMOD_REQD (IsConst)
    Generic(u16),        // ELEMENT_TYPE_VAR

    PtrMut(Box<Self>, usize),     // ELEMENT_TYPE_PTR
    PtrConst(Box<Self>, usize),   // ELEMENT_TYPE_CMOD_REQD (IsConst), ELEMENT_TYPE_PTR
    ArrayFixed(Box<Self>, usize), // ELEMENT_TYPE_ARRAY
}

impl Type {
    pub fn named(namespace: &str, name: &str) -> Self {
        Self::Name(TypeName::named(namespace, name))
    }

    pub fn code(&self) -> u8 {
        match self {
            Self::Bool => ELEMENT_TYPE_BOOLEAN,
            Self::U8 => ELEMENT_TYPE_U1,
            Self::I8 => ELEMENT_TYPE_I1,
            Self::U16 => ELEMENT_TYPE_U2,
            Self::I16 => ELEMENT_TYPE_I2,
            Self::U32 => ELEMENT_TYPE_U4,
            Self::I32 => ELEMENT_TYPE_I4,
            Self::U64 => ELEMENT_TYPE_U8,
            Self::I64 => ELEMENT_TYPE_I8,
            Self::F32 => ELEMENT_TYPE_R4,
            Self::F64 => ELEMENT_TYPE_R8,
            Self::String => ELEMENT_TYPE_STRING,
            Self::AttributeEnum => 0x55,
            rest => panic!("{rest:?}"),
        }
    }
}
