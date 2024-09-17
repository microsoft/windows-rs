use super::*;

// Note: Type::Name(TypeName) is preferred since we can use it in pattern matching whereas Type::TypeDef can't be used in that way.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
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
    String, // TODO: Win32 should use System.String when referring to an HSTRING
    Object, // TODO: Win32 should use System.Object when referring to an IInspectable

    Name(TypeName),
    Const(TypeName),

    GenericParam(GenericParam),
    TypeDef(TypeDef, Vec<Self>), // TODO: store generics inside TypeDef to simplify this

    MutPtr(Box<Self>, usize),
    ConstPtr(Box<Self>, usize),
    Win32Array(Box<Self>, usize),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    ConstRef(Box<Self>),

    // TODO: temporary hack to accommodate Win32 metadata
    PrimitiveOrEnum(Box<Self>, Box<Self>),
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
            ELEMENT_TYPE_OBJECT => Some(Self::Object),
            _ => None,
        }
    }

    /// Converts the `Type` to an equivalent `const` variant if appropriate.
    pub fn to_const_type(self) -> Self {
        match self {
            Self::MutPtr(kind, pointers) => Self::MutPtr(Box::new(kind.to_const_type()), pointers),
            Self::ConstPtr(kind, pointers) => {
                Self::ConstPtr(Box::new(kind.to_const_type()), pointers)
            }
            Self::Name(TypeName::PSTR) => Self::Const(TypeName::PSTR),
            Self::Name(TypeName::PWSTR) => Self::Const(TypeName::PWSTR),
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
            Self::Name(TypeName::PSTR) | Self::Const(TypeName::PSTR) => Self::U8,
            Self::Name(TypeName::PWSTR) | Self::Const(TypeName::PWSTR) => Self::U16,
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

    /// Returns `true` if the `Type` is a pointer.
    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::ConstPtr(_, _) | Type::MutPtr(_, _))
    }

    /// Returns `true` if the `Type` is unsigned.
    pub fn is_unsigned(&self) -> bool {
        matches!(
            self,
            Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::USize
        )
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
            Type::I8 | Type::U8 | Self::Name(TypeName::PSTR) | Self::Const(TypeName::PSTR) => true,
            _ => false,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::Name(TypeName::GUID) => 16,
            Type::TypeDef(def, _) => def.size(),
            Type::Win32Array(ty, len) => ty.size() * len,
            Type::PrimitiveOrEnum(ty, _) => ty.size(),
            _ => 4,
        }
    }

    pub fn align(&self) -> usize {
        match self {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::Name(TypeName::GUID) => 4,
            Type::TypeDef(def, _) => def.align(),
            Type::Win32Array(ty, len) => ty.align() * len,
            _ => 4,
        }
    }
}
