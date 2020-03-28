use crate::*;

#[derive(Debug, Clone)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<TypeKind>,
    pub def: TypeDef,
}

impl TypeName {
    pub fn dependencies(&self) -> Vec<TypeDef> {
        std::iter::once(self.def)
            .chain(self.generics.iter().flat_map(|i| i.dependencies()))
            .collect()
    }

    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        let (namespace, name) = def.name(reader);
        let namespace = namespace.to_string();
        let name = name.to_string();
        let mut generics = Vec::new();

        for generic in def.generics(reader) {
            let name = generic.name(reader).to_string();
            generics.push(TypeKind::Generic(name));
        }

        TypeName {
            namespace,
            name,
            generics,
            def,
        }
    }

    pub fn from_type_spec_blob(blob: &mut Blob, generics: &Vec<TypeKind>) -> Self {
        blob.read_unsigned();
        let def = TypeDefOrRef::decode(blob.read_unsigned(), blob.file).resolve(blob.reader);
        let mut args = Vec::with_capacity(blob.read_unsigned() as usize);

        for _ in 0..args.capacity() {
            args.push(TypeKind::from_blob(blob, generics));
        }
        let (namespace, name) = def.name(blob.reader);
        let namespace = namespace.to_string();
        let name = name.to_string();
        let generics = args;

        TypeName {
            namespace,
            name,
            generics,
            def,
        }
    }

    pub fn from_type_spec(reader: &Reader, spec: TypeSpec) -> Self {
        let mut blob = spec.sig(reader);
        blob.read_unsigned();
        TypeName::from_type_spec_blob(&mut blob, &Vec::new())
    }
}
