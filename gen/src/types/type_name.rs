use crate::blob::Blob;
use crate::*;

use std::iter::FromIterator;

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

        Self {
            namespace,
            name,
            generics,
            def,
        }
    }

    pub fn from_type_spec_blob(blob: &mut Blob, generics: &[TypeKind]) -> Self {
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

        Self {
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

    pub fn ident(&self) -> TokenStream {
        if self.generics.is_empty() {
            let name = write_ident(&self.name);
            quote! { #name }
        } else {
            let name = write_ident(&self.name[..self.name.len() - 2]);
            let generics = self.generics.iter().map(|g| g.ident());
            quote! { #name<#(#generics),*> }
        }
    }

    pub fn phantoms(&self) -> TokenStream {
        if self.generics.is_empty() {
            TokenStream::new()
        } else {
            let mut tokens = Vec::new();

            for (count, generic) in self.generics.iter().enumerate() {
                let name = format_ident!("__{}", count);
                let generic = generic.ident();
                tokens.push(quote! { #name: std::marker::PhantomData::<#generic>, })
            }

            TokenStream::from_iter(tokens)
        }
    }

    pub fn constraints(&self) -> TokenStream {
        let generics = self.generics.iter().map(|generic| {
            let generic = generic.ident();
            quote! { #generic: winrt::RuntimeType + 'static, }
        });

        TokenStream::from_iter(generics)
    }
}
