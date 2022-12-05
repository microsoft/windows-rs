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
    BSTR,
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
    /// Creates a `Type` object from an `ELEMENT_TYPE` (see ECMA-335) type constant, typically
    /// used to indicate the type of a constant or primitive type signature.
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

    /// Converts the `Type` to an equivalent `const` variant if appropriate, typically used when the
    /// mutability is informed by something outside of the type signature.
    pub fn to_const(self) -> Self {
        match self {
            Self::MutPtr(p) => Self::ConstPtr(p).to_const(),
            Self::ConstPtr((kind, pointers)) => Self::ConstPtr((Box::new(kind.clone().to_const()), pointers)),
            Self::PSTR => Self::PCSTR,
            Self::PWSTR => Self::PCWSTR,
            _ => self,
        }
    }

    /// Removes one level of indirection, typically used when transforming a logical return or array parameter
    /// from its underlying type signature.
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
            _ => panic!("`deref` can only be called on pointer types"),
        }
    }

    /// Returns `true` if the `Type` represents a WinRT array.
    pub fn is_winrt_array(&self) -> bool {
        matches!(self, Type::WinrtArray(_))
    }

    /// Returns `true` if the `Type` represents a mutable WinRT array reference.
    pub fn is_winrt_array_ref(&self) -> bool {
        matches!(self, Type::WinrtArrayRef(_))
    }

    /// Returns `true` if the `Type` represents an immutable WinRT array reference.
    pub fn is_winrt_const_ref(&self) -> bool {
        matches!(self, Type::WinrtConstRef(_))
    }

    /// Returns `true` if the `Type` is a generic parameter.
    pub fn is_generic(&self) -> bool {
        matches!(self, Type::GenericParam(_))
    }

    /// Returns `true` if the `Type` is a pointer.
    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::ConstPtr(_) | Type::MutPtr(_))
    }

    /// Returns `true` if the `Type` is unsigned.
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::USize)
    }

    /// Returns `true` if the `Type` is incomplete.
    pub fn is_void(&self) -> bool {
        match self {
            Type::ConstPtr((kind, _)) | Type::MutPtr((kind, _)) => kind.is_void(),
            Type::Void => true,
            _ => false,
        }
    }

    /// Returns `true` if the `Type` has a byte-sized address.
    pub fn is_byte_size(&self) -> bool {
        match self {
            Type::ConstPtr((kind, _)) | Type::MutPtr((kind, _)) => kind.is_byte_size(),
            Type::I8 | Type::U8 | Type::PSTR | Type::PCSTR => true,
            _ => false,
        }
    }
}
