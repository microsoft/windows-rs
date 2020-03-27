use crate::*;

#[derive(Debug, Clone)]
pub enum TypeKind {
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
    String,
    Object,
    Guid,
    Class(TypeName),
    Interface(TypeName),
    Enum(TypeName),
    Struct(TypeName),
    Delegate(TypeName),
    Generic(String),
}

impl TypeKind {
    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);

        match def.category(reader) {
            TypeCategory::Interface => TypeKind::Interface(name),
            TypeCategory::Class => TypeKind::Class(name),
            TypeCategory::Enum => TypeKind::Enum(name),
            TypeCategory::Struct => TypeKind::Struct(name),
            TypeCategory::Delegate => TypeKind::Delegate(name),
        }
    }

    pub fn from_type_ref(reader: &Reader, type_ref: TypeRef) -> Self {
        let (namespace, name) = type_ref.name(reader);
        if (namespace, name) == ("System", "Guid") {
            TypeKind::Guid
        } else {
            Self::from_type_def(reader, reader.resolve((namespace, name)))
        }
    }

    pub fn from_type_spec(reader: &Reader, spec: TypeSpec) -> Self {
        TypeKind::Interface(TypeName::from_type_spec(reader, spec))
    }

    pub fn from_type_def_or_ref(reader: &Reader, code: TypeDefOrRef) -> Self {
        match code {
            TypeDefOrRef::TypeRef(value) => Self::from_type_ref(reader, value),
            TypeDefOrRef::TypeDef(value) => Self::from_type_def(reader, value),
            TypeDefOrRef::TypeSpec(value) => Self::from_type_spec(reader, value),
        }
    }

    pub fn from_blob(blob: &mut Blob, generics: &Vec<TypeKind>) -> Self {
        blob.read_expected(0x1D);
        blob.read_modifiers();

        match blob.read_unsigned() {
            0x02 => TypeKind::Bool,
            0x03 => TypeKind::Char,
            0x04 => TypeKind::I8,
            0x05 => TypeKind::U8,
            0x06 => TypeKind::I16,
            0x07 => TypeKind::U16,
            0x08 => TypeKind::I32,
            0x09 => TypeKind::U32,
            0x0A => TypeKind::I64,
            0x0B => TypeKind::U64,
            0x0C => TypeKind::F32,
            0x0D => TypeKind::F64,
            0x0E => TypeKind::String,
            0x1C => TypeKind::Object,
            0x11 | 0x12 => Self::from_type_def_or_ref(
                blob.reader,
                TypeDefOrRef::decode(blob.read_unsigned(), blob.file),
            ),
            0x13 => generics[blob.read_unsigned() as usize].clone(),

            0x15 => TypeKind::Interface(TypeName::from_type_spec_blob(blob, generics)),
            _ => panic!("TypeKind::from_blob"),
        }
    }

    pub fn from_field(reader: &Reader, field: Field) -> Self {
        let mut blob = field.sig(reader);
        blob.read_unsigned();
        blob.read_modifiers();
        Self::from_blob(&mut blob, &Vec::new())
    }

    // pub fn add_dependencies(&self, reader: &Reader, map: &mut BTreeMap::<TypeDef, Type>) {
    //     // match self {
    //     //     TypeKind::Class(name) => f(name),
    //     //     TypeKind::Interface(name) => f(name),
    //     //     TypeKind::Enum(name) => f(name),
    //     //     TypeKind::Struct(name) => f(name),
    //     //     TypeKind::Delegate(name) => f(name),
    //     //     _ => {}
    //     // }
    // }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        Vec::new()
    }
}
