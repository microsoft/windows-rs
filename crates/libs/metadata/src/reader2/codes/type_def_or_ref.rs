use super::*;

#[derive(Clone)]
pub enum TypeDefOrRef<'a> {
    None,
    TypeDef(TypeDef<'a>),
    TypeRef(TypeRef<'a>),
    TypeSpec(TypeSpec<'a>),
}

impl<'a>  TypeDefOrRef<'a> {
    pub fn resolve(&'static self, enclosing: Option<&TypeDef>, generics: &[Type]) -> Type {
        if let Self::TypeSpec(def) = self {
            return def.blob().reader().read_type_spec(enclosing, generics);
        }

        let mut type_name = self.type_name();

        for (known_name, kind) in WELL_KNOWN_TYPES {
            if type_name == known_name {
                return kind;
            }
        }

        const REMAP_TYPES: [(TypeName, TypeName); 1] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2)];

        for (from, to) in REMAP_TYPES {
            if type_name == from {
                type_name = to;
            }
        }

        code.resolve(enclosing).into()
    }

    fn type_name(&self) -> TypeName {
        match self {
            Self::TypeRef(value) => value.type_name(),
            Self::TypeDef(value) => value.type_name(),
            _ => unimplemented!(),
        }
    }
    fn scope(&self) -> &Scope {
        match self {
            Self::TypeRef(value) => alue.scope,
            Self::TypeDef(value) => alue.scope,
            Self::TypeSpec(value) => value.scope,
        }
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
