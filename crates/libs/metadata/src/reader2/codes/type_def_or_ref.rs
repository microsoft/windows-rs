use super::*;

#[derive(Clone)]
pub enum TypeDefOrRef<'a> {
    None,
    TypeDef(TypeDef<'a>),
    TypeRef(TypeRef<'a>),
    TypeSpec(TypeSpec<'a>),
}

impl<'a> TypeDefOrRef<'a> {
    pub fn encode(&self) -> usize {
        (match self {
            Self::None => 0,
            Self::TypeDef(value) => ((value.0.key.row + 1) << 2),
            Self::TypeRef(value) => ((value.0.key.row + 1) << 2) | 1,
            Self::TypeSpec(value) => ((value.0.key.row + 1) << 2) | 2,
        }) as _
    }
}

impl<'a> Decode<'a> for TypeDefOrRef<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self {
        if code == 0 {
            return Self::None;
        }
        let (kind, row) = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match kind {
            0 => Self::TypeDef(TypeDef(Row::new(scope, ScopeKey::new(row, TABLE_TYPEDEF, file)), Vec::new())),
            1 => Self::TypeRef(TypeRef(Row::new(scope, ScopeKey::new(row, TABLE_TYPEREF, file)))),
            2 => Self::TypeSpec(TypeSpec(Row::new(scope, ScopeKey::new(row, TABLE_TYPESPEC, file)))),
            _ => unimplemented!(),
        }
    }
}
