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
    GenericParam(GenericParam),
    TypeDef((TypeDef, Vec<Self>)),
    MutPtr((Box<Self>, usize)),
    ConstPtr((Box<Self>, usize)),
    Win32Array((Box<Self>, usize)),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    WinrtConstRef(Box<Self>),
}

impl Type {
    pub fn from_code(code: usize) -> Option<Self> {
        match code {
            0x01 => Some(Self::Void),
            0x02 => Some(Self::Bool),
            0x03 => Some(Self::Char),
            0x04 => Some(Self::I8),
            0x05 => Some(Self::U8),
            0x06 => Some(Self::I16),
            0x07 => Some(Self::U16),
            0x08 => Some(Self::I32),
            0x09 => Some(Self::U32),
            0x0a => Some(Self::I64),
            0x0b => Some(Self::U64),
            0x0c => Some(Self::F32),
            0x0d => Some(Self::F64),
            0x18 => Some(Self::ISize),
            0x19 => Some(Self::USize),
            0x0e => Some(Self::String),
            0x1c => Some(Self::IInspectable),
            _ => None,
        }
    }
    pub fn to_const(self) -> Self {
        match self {
            Self::MutPtr((kind, pointers)) => Self::ConstPtr((kind, pointers)),
            Self::PSTR => Self::PCSTR,
            Self::PWSTR => Self::PCWSTR,
            _ => self,
        }
    }
    pub fn deref(&self) -> Self {
        match self {
            Self::ConstPtr((kind, 1)) | Self::MutPtr((kind, 1)) => {
                if **kind == Self::Void {
                    Self::U8
                } else {
                    *kind.clone()
                }
            }
            Self::ConstPtr((kind, pointers)) => Self::ConstPtr((kind.clone(), pointers - 1)),
            Self::MutPtr((kind, pointers)) => Self::MutPtr((kind.clone(), pointers - 1)),
            Self::PSTR | Self::PCSTR => Self::U8,
            Self::PWSTR | Self::PCWSTR => Self::U16,
            _ => unimplemented!(),
        }
    }
    pub fn is_winrt_array(&self) -> bool {
        matches!(self, Type::WinrtArray(_))
    }
    pub fn is_winrt_array_ref(&self) -> bool {
        matches!(self, Type::WinrtArrayRef(_))
    }
    pub fn is_winrt_const_ref(&self) -> bool {
        matches!(self, Type::WinrtConstRef(_))
    }
    pub fn is_generic(&self) -> bool {
        matches!(self, Type::GenericParam(_))
    }
    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::ConstPtr(_) | Type::MutPtr(_))
    }
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::USize)
    }
    pub fn is_void(&self) -> bool {
        match self {
            // TODO: do we care about void behind pointers?
            Type::ConstPtr((kind, _)) | Type::MutPtr((kind, _)) => kind.is_void(),
            Type::Void => true,
            _ => false,
        }
    }
}
