use super::*;

#[derive(Clone)]
pub enum HasAttribute<'a> {
    MethodDef(MethodDef<'a>),
    Field(Field<'a>),
    TypeRef(TypeRef<'a>),
    TypeDef(TypeDef<'a>),
    Param(Param<'a>),
    InterfaceImpl(InterfaceImpl<'a>),
    MemberRef(MemberRef<'a>),
    TypeSpec(TypeSpec<'a>),
    GenericParam(GenericParam<'a>),
}

impl<'a> HasAttribute<'a> {
    pub fn encode(&self) -> usize {
        (match self {
            Self::MethodDef(value) => ((value.0.key.row + 1) << 5),
            Self::Field(value) => ((value.0.key.row + 1) << 5) | 1,
            Self::TypeRef(value) => ((value.0.key.row + 1) << 5) | 2,
            Self::TypeDef(value) => ((value.0.key.row + 1) << 5) | 3,
            Self::Param(value) => ((value.0.key.row + 1) << 5) | 4,
            Self::InterfaceImpl(value) => ((value.0.key.row + 1) << 5) | 5,
            Self::MemberRef(value) => ((value.0.key.row + 1) << 5) | 6,
            Self::TypeSpec(value) => ((value.0.key.row + 1) << 5) | 13,
            Self::GenericParam(value) => ((value.0.key.row + 1) << 5) | 19,
        }) as _
    }
}
