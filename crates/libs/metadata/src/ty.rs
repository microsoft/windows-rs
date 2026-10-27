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
    ClassName(TypeName),          // ELEMENT_TYPE_CLASS
    ValueName(TypeName),          // ELEMENT_TYPE_VALUETYPE
    Array(Box<Self>),             // ELEMENT_TYPE_SZARRAY
    Generic(String, u16),         // ELEMENT_TYPE_VAR
    RefMut(Box<Self>),            // ELEMENT_TYPE_BYREF
    RefConst(Box<Self>),          // ELEMENT_TYPE_CMOD_REQD (IsConst)
    PtrMut(Box<Self>, usize),     // ELEMENT_TYPE_PTR
    PtrConst(Box<Self>, usize),   // ELEMENT_TYPE_CMOD_REQD (IsConst), ELEMENT_TYPE_PTR
    ArrayFixed(Box<Self>, usize), // ELEMENT_TYPE_ARRAY
    FnPtr(Box<Signature>),        // ELEMENT_TYPE_FNPTR
}

impl Type {
    pub fn class_named(namespace: &str, name: &str) -> Self {
        Self::ClassName(TypeName::named(namespace, name))
    }

    pub fn value_named(namespace: &str, name: &str) -> Self {
        Self::ValueName(TypeName::named(namespace, name))
    }

    /// Returns the named representation used by `System.Type` attribute values.
    pub fn type_name(&self) -> Option<TypeName> {
        let name = match self {
            Self::Void => "Void",
            Self::Bool => "Boolean",
            Self::Char => "Char",
            Self::I8 => "SByte",
            Self::U8 => "Byte",
            Self::I16 => "Int16",
            Self::U16 => "UInt16",
            Self::I32 => "Int32",
            Self::U32 => "UInt32",
            Self::I64 => "Int64",
            Self::U64 => "UInt64",
            Self::F32 => "Single",
            Self::F64 => "Double",
            Self::ISize => "IntPtr",
            Self::USize => "UIntPtr",
            Self::String => "String",
            Self::Object => "Object",
            Self::ClassName(name) | Self::ValueName(name) if name.generics.is_empty() => {
                return Some(name.clone());
            }
            _ => return None,
        };
        Some(TypeName::named("System", name))
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
            Self::USize => ELEMENT_TYPE_U,
            Self::ISize => ELEMENT_TYPE_I,
            Self::F32 => ELEMENT_TYPE_R4,
            Self::F64 => ELEMENT_TYPE_R8,
            Self::String => ELEMENT_TYPE_STRING,
            rest => panic!("{rest:?}"),
        }
    }
}
