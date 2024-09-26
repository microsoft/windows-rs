use super::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
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

    PSTR,
    PCSTR,
    PWSTR,
    PCWSTR,

    TypeDef(TypeDef, Vec<Self>),
    Generic(&'static str),
    PtrMut(Box<Self>, usize),
    PtrConst(Box<Self>, usize),
    ArrayFixed(Box<Self>, usize),
    Array(Box<Self>),
    ArrayRef(Box<Self>),
    ConstRef(Box<Self>),
    PrimitiveOrEnum(Box<Self>, TypeDef),
}

impl Type {
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

    pub fn to_const_type(&self) -> Self {
        match self {
            Self::PtrMut(ty, pointers) => Self::PtrMut(Box::new(ty.to_const_type()), *pointers),
            Self::PtrConst(ty, pointers) => Self::PtrConst(Box::new(ty.to_const_type()), *pointers),
            Self::PSTR => Self::PCSTR,
            Self::PWSTR => Self::PCWSTR,
            _ => self.clone(),
        }
    }

    fn to_underlying_type(&self) -> Self {
        match self {
            Self::PtrMut(ty, _) => *ty.clone(),
            Self::PtrConst(ty, _) => *ty.clone(),
            Self::ArrayFixed(ty, _) => *ty.clone(),
            Self::Array(ty) => *ty.clone(),
            Self::ArrayRef(ty) => *ty.clone(),
            Self::ConstRef(ty) => *ty.clone(),
            Self::PrimitiveOrEnum(_, def) => Self::TypeDef(*def, vec![]),
            _ => self.clone(),
        }
    }

    pub fn to_const_ptr(&self) -> Self {
        match self {
            Self::PtrMut(ty, pointers) => Self::PtrConst(ty.clone(), *pointers),
            _ => self.clone(),
        }
    }

    pub fn deref(self) -> Self {
        match self {
            Self::PtrConst(ty, 1) | Self::PtrMut(ty, 1) => {
                if *ty == Self::Void {
                    Self::U8
                } else {
                    *ty
                }
            }
            Self::PtrConst(ty, pointers) => Self::PtrConst(ty.clone(), pointers - 1),
            Self::PtrMut(ty, pointers) => Self::PtrMut(ty.clone(), pointers - 1),
            Self::PSTR | Self::PCSTR => Self::U8,
            Self::PWSTR | Self::PCWSTR => Self::U16,
            rest => unimplemented!("{rest:?}"),
        }
    }

    // pub fn dependencies(&self, set: &mut HashSet<Self>) {
    //     let ty = self.to_underlying_type();

    //     if !set.insert(ty.clone()) {
    //         return;
    //     }

    //     // TODO: maybe self.item() or self.type_name()

    //     let
    //         Self::TypeDef(def, generics) = ty else {
    //             return;
    //         };

    //         // TODO: get the `Item` from the reader again to be able to retrieve the nested types etc.
    //     }
    // }
}
