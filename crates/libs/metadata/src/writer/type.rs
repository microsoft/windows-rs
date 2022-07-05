use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
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
    TypeName,
    GenericParam(String),
    TypeDef((TypeName, Vec<Self>)),
    MutPtr((Box<Self>, usize)),
    ConstPtr((Box<Self>, usize)),
    Win32Array((Box<Self>, usize)),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    WinrtConstRef(Box<Self>),
}

impl Type {
    /// Returns an `ELEMENT_TYPE` (see ECMA-335) type constant for the `Type` object, typically
    /// used to indicate the type of a constant or primitive type signature.
    pub fn to_code(&self) -> Option<usize> {
        match self {
            Self::Void => Some(0x01),
            Self::Bool => Some(0x02),
            Self::Char => Some(0x03),
            Self::I8 => Some(0x04),
            Self::U8 => Some(0x05),
            Self::I16 => Some(0x06),
            Self::U16 => Some(0x07),
            Self::I32 => Some(0x08),
            Self::U32 => Some(0x09),
            Self::I64 => Some(0x0a),
            Self::U64 => Some(0x0b),
            Self::F32 => Some(0x0c),
            Self::F64 => Some(0x0d),
            Self::ISize => Some(0x18),
            Self::USize => Some(0x19),
            Self::String => Some(0x0e),
            Self::IInspectable => Some(0x1c),
            _ => None,
        }
    }

    /// Encodes the `Type` as FieldSig (see ECMA-335).
    pub fn to_field_sig(&self) -> Vec<u8> {
        if let Some(code) = self.to_code() {
            return vec![0x6, code as _];
        }
        unimplemented!();
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}
