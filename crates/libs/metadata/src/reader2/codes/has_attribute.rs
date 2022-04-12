use super::*;

#[derive(Clone)]
pub enum HasAttribute {
    MethodDef(MethodDef),
    Field(Field),
    TypeRef(TypeRef),
    TypeDef(TypeDef),
    Param(Param),
    InterfaceImpl(InterfaceImpl),
    MemberRef(MemberRef),
    TypeSpec(TypeSpec),
    GenericParam(GenericParam),
}

impl HasAttribute {
    pub fn encode(&self) -> usize {
        (match self {
            Self::MethodDef(value) => ((value.0.row + 1) << 5),
            Self::Field(value) => ((value.0.row + 1) << 5) | 1,
            Self::TypeRef(value) => ((value.0.row + 1) << 5) | 2,
            Self::TypeDef(value) => ((value.0.row + 1) << 5) | 3,
            Self::Param(value) => ((value.0.row + 1) << 5) | 4,
            Self::InterfaceImpl(value) => ((value.0.row + 1) << 5) | 5,
            Self::MemberRef(value) => ((value.0.row + 1) << 5) | 6,
            Self::TypeSpec(value) => ((value.0.row + 1) << 5) | 13,
            Self::GenericParam(value) => ((value.0.row + 1) << 5) | 19,
        }) as _
    }
}
