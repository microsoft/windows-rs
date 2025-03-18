use super::*;

#[derive(Debug)]
pub enum Type<'a> {
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
    Type,
    Name(TypeName<'a>),
    Array(Box<Self>),    // ELEMENT_TYPE_SZARRAY
    ArrayRef(Box<Self>), // ELEMENT_TYPE_BYREF, ELEMENT_TYPE_SZARRAY
    ConstRef(Box<Self>), // ELEMENT_TYPE_CMOD_REQD (IsConst)
    Generic(usize),      // ELEMENT_TYPE_VAR

    PtrMut(Box<Self>, usize),     // ELEMENT_TYPE_PTR
    PtrConst(Box<Self>, usize),   // ELEMENT_TYPE_CMOD_REQD (IsConst), ELEMENT_TYPE_PTR
    ArrayFixed(Box<Self>, usize), // ELEMENT_TYPE_ARRAY
}

impl<'a> Type<'a> {
    pub fn new(namespace: &'a str, name: &'a str) -> Self {
        Self::Name(TypeName::new(namespace, name))
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
            rest => panic!("{rest:?}"),
        }
    }
}
