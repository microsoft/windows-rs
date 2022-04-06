use super::*;

#[derive(Clone)]
pub enum MemberRefParent<'a> {
    TypeDef(TypeDef<'a>),
    TypeRef(TypeRef<'a>),
    MethodDef(MethodDef<'a>),
    TypeSpec(TypeSpec<'a>),
}

impl<'a> MemberRefParent<'a> {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(value) => ((value.0.key.row + 1) << 3),
            Self::TypeRef(value) => ((value.0.key.row + 1) << 3) | 1,
            Self::MethodDef(value) => ((value.0.key.row + 1) << 3) | 3,
            Self::TypeSpec(value) => ((value.0.key.row + 1) << 3) | 4,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::TypeDef(value) => value.name(),
            Self::TypeRef(value) => value.name(),
            _ => unimplemented!(),
        }
    }
}

impl<'a> Decode<'a> for MemberRefParent<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match kind {
            0 => Self::TypeDef(TypeDef(Row::new(scope, ScopeKey::new(row, TABLE_TYPEDEF, file)), Vec::new())),
            1 => Self::TypeRef(TypeRef(Row::new(scope, ScopeKey::new(row, TABLE_TYPEREF, file)))),
            3 => Self::MethodDef(MethodDef(Row::new(scope, ScopeKey::new(row, TABLE_METHODDEF, file)))),
            4 => Self::TypeSpec(TypeSpec(Row::new(scope, ScopeKey::new(row, TABLE_TYPESPEC, file)))),
            _ => unimplemented!(),
        }
    }
}
