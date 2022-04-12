use super::*;

#[derive(Clone)]
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

    // pub fn type_from_code(&self, enclosing: Option<&TypeDef>, generics: &'a [Type]) -> Type {
    //     if let TypeDefOrRef::TypeSpec(def) = self {
    //         let mut blob = def.signature();
    //         return blob.type_from_blob_impl(enclosing, generics);
    //     }

    //     let full_name = self.type_name();

    //     for (known_name, kind) in WELL_KNOWN_TYPES {
    //         if full_name == known_name {
    //             return kind;
    //         }
    //     }

    //     for (from, to) in REMAP_TYPES {
    //         if full_name == from {
    //             full_name = to;
    //             break;
    //         }
    //     }

    //     let scope = self.scope();

    //     if let Some(outer) = enclosing {
    //         Type::TypeDef(scope.get_nested(outer, full_name.name))
    //     } else {
    //         Type::TypeDef(scope.get(&full_name).next().expect("Type not found"))
    //     }
    // }
}

const REMAP_TYPES: [(TypeName, TypeName); 1] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2)];

const WELL_KNOWN_TYPES: [(TypeName, Type); 10] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::IInspectable, Type::IInspectable), (TypeName::LARGE_INTEGER, Type::I64), (TypeName::ULARGE_INTEGER, Type::U64), (TypeName::PSTR, Type::PSTR), (TypeName::PWSTR, Type::PWSTR)];
