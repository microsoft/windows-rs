use super::*;

#[derive(Copy, Clone)]
pub enum TypeDefOrRef {
    None,
    TypeDef(TypeDef),
    TypeRef(TypeRef),
    TypeSpec(TypeSpec),
}

impl Decode for TypeDefOrRef {
    fn decode(file: usize, code: usize) -> Self {
        if code == 0 {
            return Self::None;
        }
        let (kind, row) = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match kind {
            0 => Self::TypeDef(TypeDef(ScopeKey::new(row, TABLE_TYPEDEF, file))),
            1 => Self::TypeRef(TypeRef(ScopeKey::new(row, TABLE_TYPEREF, file))),
            2 => Self::TypeSpec(TypeSpec(ScopeKey::new(row, TABLE_TYPESPEC, file))),
            _ => unimplemented!(),
        }
    }
}

impl TypeDefOrRef {
    pub fn type_name<'a>(&self, scope: &'a Scope) -> TypeName<'a> {
        match self {
            Self::TypeDef(value) => value.type_name(scope),
            Self::TypeRef(value) => value.type_name(scope),
            _ => unimplemented!(),
        }
    }
}

