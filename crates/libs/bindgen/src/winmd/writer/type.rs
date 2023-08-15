#![allow(dead_code, clippy::upper_case_acronyms)]

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<Type>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
    GUID,
    IUnknown,
    IInspectable,
    HRESULT,
    PSTR,
    PWSTR,
    PCSTR,
    PCWSTR,
    BSTR,
    TypeName,
    TypeRef(TypeName),
    GenericParam(u16),
    MutPtr(Box<Self>, usize),
    ConstPtr(Box<Self>, usize),
    Win32Array(Box<Self>, usize),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    ConstRef(Box<Self>),
}

impl Type {
    pub fn into_mut_ptr(self) -> Self {
        match self {
            Self::MutPtr(ty, count) => Self::MutPtr(ty, count + 1),
            Self::ConstPtr(ty, count) => Self::MutPtr(ty, count + 1),
            _ => Self::MutPtr(Box::new(self), 1),
        }
    }

    pub fn into_const_ptr(self) -> Self {
        match self {
            Self::MutPtr(ty, count) => Self::ConstPtr(ty, count + 1),
            Self::ConstPtr(ty, count) => Self::ConstPtr(ty, count + 1),
            _ => Self::ConstPtr(Box::new(self), 1),
        }
    }

    pub fn into_array(self, len: usize) -> Self {
        Self::Win32Array(Box::new(self), len)
    }
}

pub struct Signature {
    pub params: Vec<SignatureParam>,
    pub return_type: Type,
    pub call_flags: u8,
}

// TODO: just Param?
pub struct SignatureParam {
    pub name: String,
    pub ty: Type,
}
