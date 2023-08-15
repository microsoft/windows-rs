use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Type {
    // Primitives in ECMA-335
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

    // System types
    GUID,         // Both Win32 and WinRT agree that this is represented by System.Guid
    String,       // TODO: Win32 should use System.String when referring to an HSTRING
    IInspectable, // TODO: Win32 should use System.Object when referring to an IInspectable

    // Meta-type indicating type name in attribute blob.
    TypeName,

    // Regular ECMA-335 types that map to metadata
    TypeRef(TypeDefOrRef), // Note: this ought to be a TypeName but that would require Type to have a lifetime reference.
    GenericParam(GenericParam),
    TypeDef(TypeDef, Vec<Self>),

    // Qualified types
    MutPtr(Box<Self>, usize),
    ConstPtr(Box<Self>, usize),
    Win32Array(Box<Self>, usize),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    ConstRef(Box<Self>),

    // TODO: temporary hack to accommodate Win32 metadata
    PrimitiveOrEnum(Box<Self>, Box<Self>),

    // TODO: these should not be "special" and just point to regular metadata types in Win32.Foundation
    HRESULT,  // TODO: Win32 should use Windows.Foundation.HResult when referring to HRESULT
    IUnknown, // TODO: should be defined in Windows.Win32.Foundation.IUnknown
    PSTR,
    PWSTR,
    PCSTR,
    PCWSTR,
    BSTR,
}

impl Type {
    /// Creates a `Type` object from an `ELEMENT_TYPE` (see ECMA-335) type constant, typically
    /// used to indicate the type of a constant or primitive type signature.
    pub fn from_code(code: usize) -> Option<Self> {
        match code as u8 {
            ELEMENT_TYPE_VOID => Some(Self::Void),
            ELEMENT_TYPE_BOOLEAN => Some(Self::Bool),
            ELEMENT_TYPE_CHAR => Some(Self::Char),
            ELEMENT_TYPE_I1 => Some(Self::I8),
            ELEMENT_TYPE_U1 => Some(Self::U8),
            ELEMENT_TYPE_I2 => Some(Self::I16),
            ELEMENT_TYPE_U2 => Some(Self::U16),
            ELEMENT_TYPE_I4 => Some(Self::I32),
            ELEMENT_TYPE_U4 => Some(Self::U32),
            ELEMENT_TYPE_I8 => Some(Self::I64),
            ELEMENT_TYPE_U8 => Some(Self::U64),
            ELEMENT_TYPE_R4 => Some(Self::F32),
            ELEMENT_TYPE_R8 => Some(Self::F64),
            ELEMENT_TYPE_I => Some(Self::ISize),
            ELEMENT_TYPE_U => Some(Self::USize),
            ELEMENT_TYPE_STRING => Some(Self::String),
            ELEMENT_TYPE_OBJECT => Some(Self::IInspectable),
            _ => None,
        }
    }

    /// Converts the `Type` to an equivalent `const` variant if appropriate.
    pub fn to_const_type(self) -> Self {
        match self {
            Self::MutPtr(kind, pointers) => Self::MutPtr(Box::new(kind.to_const_type()), pointers),
            Self::ConstPtr(kind, pointers) => Self::ConstPtr(Box::new(kind.to_const_type()), pointers),
            Self::PSTR => Self::PCSTR,
            Self::PWSTR => Self::PCWSTR,
            _ => self,
        }
    }

    pub fn to_underlying_type(&self) -> Self {
        match self {
            Type::MutPtr(ty, _) => *ty.clone(),
            Type::ConstPtr(ty, _) => *ty.clone(),
            Type::Win32Array(ty, _) => *ty.clone(),
            Type::WinrtArray(ty) => *ty.clone(),
            Type::WinrtArrayRef(ty) => *ty.clone(),
            Type::ConstRef(ty) => *ty.clone(),
            _ => self.clone(),
        }
    }

    /// Converts a mutable pointer type, if appropriate, to a const pointer type.
    pub fn to_const_ptr(self) -> Self {
        match self {
            Self::MutPtr(kind, pointers) => Self::ConstPtr(kind, pointers),
            _ => self,
        }
    }

    /// Removes one level of indirection, typically used when transforming a logical return or array parameter
    /// from its underlying type signature.
    pub fn deref(&self) -> Self {
        match self {
            Self::ConstPtr(kind, 1) | Self::MutPtr(kind, 1) => {
                if **kind == Self::Void {
                    Self::U8
                } else {
                    *kind.clone()
                }
            }
            Self::ConstPtr(kind, pointers) => Self::ConstPtr(kind.clone(), pointers - 1),
            Self::MutPtr(kind, pointers) => Self::MutPtr(kind.clone(), pointers - 1),
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
    pub fn is_const_ref(&self) -> bool {
        matches!(self, Type::ConstRef(_))
    }

    /// Returns `true` if the `Type` is a generic parameter.
    pub fn is_generic(&self) -> bool {
        matches!(self, Type::GenericParam(_))
    }

    /// Returns `true` if the `Type` is a pointer.
    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::ConstPtr(_, _) | Type::MutPtr(_, _))
    }

    /// Returns `true` if the `Type` is unsigned.
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::USize)
    }

    /// Returns `true` if the `Type` is incomplete.
    pub fn is_void(&self) -> bool {
        match self {
            Type::ConstPtr(kind, _) | Type::MutPtr(kind, _) => kind.is_void(),
            Type::Void => true,
            _ => false,
        }
    }

    /// Returns `true` if the `Type` has a byte-sized address.
    pub fn is_byte_size(&self) -> bool {
        match self {
            Type::ConstPtr(kind, _) | Type::MutPtr(kind, _) => kind.is_byte_size(),
            Type::I8 | Type::U8 | Type::PSTR | Type::PCSTR => true,
            _ => false,
        }
    }
}
