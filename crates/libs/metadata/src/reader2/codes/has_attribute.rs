use super::*;

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
            Self::MethodDef(row) => ((row.0.row + 1) << 5),
            Self::Field(row) => ((row.0.row + 1) << 5) | 1,
            Self::TypeRef(row) => ((row.0.row + 1) << 5) | 2,
            Self::TypeDef(row) => ((row.0.row + 1) << 5) | 3,
            Self::Param(row) => ((row.0.row + 1) << 5) | 4,
            Self::InterfaceImpl(row) => ((row.0.row + 1) << 5) | 5,
            Self::MemberRef(row) => ((row.0.row + 1) << 5) | 6,
            Self::TypeSpec(row) => ((row.0.row + 1) << 5) | 13,
            Self::GenericParam(row) => ((row.0.row + 1) << 5) | 19,
        }) as _
    }
}
