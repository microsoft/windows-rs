use super::*;

#[derive(Clone)]
pub enum HasCustomAttribute<'a> {
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

impl<'a> HasCustomAttribute<'a> {
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

impl<'a> Decode<'a> for HasCustomAttribute<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 5) - 1), (code >> 5) - 1);
        match kind {
            0 => Self::MethodDef(MethodDef(Row::new(scope, ScopeKey::new(row, TABLE_METHODDEF, file)))),
            1 => Self::Field(Field(Row::new(scope, ScopeKey::new(row, TABLE_FIELD, file)))),
            2 => Self::TypeRef(TypeRef(Row::new(scope, ScopeKey::new(row, TABLE_TYPEREF, file)))),
            3 => Self::TypeDef(TypeDef(Row::new(scope, ScopeKey::new(row, TABLE_TYPEDEF, file)), Vec::new())),
            4 => Self::Param(Param(Row::new(scope, ScopeKey::new(row, TABLE_PARAM, file)))),
            5 => Self::InterfaceImpl(InterfaceImpl(Row::new(scope, ScopeKey::new(row, TABLE_INTERFACEIMPL, file)))),
            6 => Self::MemberRef(MemberRef(Row::new(scope, ScopeKey::new(row, TABLE_MEMBERREF, file)))),
            13 => Self::TypeSpec(TypeSpec(Row::new(scope, ScopeKey::new(row, TABLE_TYPESPEC, file)))),
            19 => Self::GenericParam(GenericParam(Row::new(scope, ScopeKey::new(row, TABLE_GENERICPARAM, file)))),
            _ => unimplemented!(),
        }
    }
}
