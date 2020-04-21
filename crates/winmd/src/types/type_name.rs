use crate::blob::Blob;
use crate::case::*;
use crate::codes::*;
use crate::tables::*;
use crate::types::*;
use crate::*;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<TypeKind>,
    pub def: TypeDef,
}

impl TypeName {
    pub fn from_type_def_or_ref(
        reader: &TypeReader,
        code: TypeDefOrRef,
        generics: &Vec<TypeKind>,
    ) -> Self {
        match code {
            TypeDefOrRef::TypeRef(value) => Self::from_type_ref(reader, value),
            TypeDefOrRef::TypeDef(value) => Self::from_type_def(reader, value),
            TypeDefOrRef::TypeSpec(value) => Self::from_type_spec(reader, value, generics),
        }
    }

    pub fn from_type_ref(reader: &TypeReader, type_ref: TypeRef) -> TypeName {
        let (namespace, name) = type_ref.name(reader);
        Self::from_type_def(reader, reader.resolve_type_def((namespace, name)))
    }

    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
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

    pub fn from_type_spec_blob(blob: &mut Blob, generics: &Vec<TypeKind>) -> Self {
        blob.read_unsigned();
        let def = TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index).resolve(blob.reader);
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

    pub fn from_type_spec(reader: &TypeReader, spec: TypeSpec, generics: &Vec<TypeKind>) -> Self {
        let mut blob = spec.sig(reader);
        blob.read_unsigned();
        TypeName::from_type_spec_blob(&mut blob, generics)
    }

    pub fn runtime_name(&self) -> String {
        let mut result = format!("{}.{}", self.namespace, self.name);
        let mut generics = self.generics.iter();

        let first = match generics.next() {
            Some(first) => first,
            None => return result,
        };

        result += "<";
        result += &first.runtime_name();

        for kind in generics {
            result += ", ";
            result += &kind.runtime_name();
        }

        result += ">";

        result
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        std::iter::once(self.def)
            .chain(self.generics.iter().flat_map(|i| i.dependencies()))
            .collect()
    }

    pub fn to_tokens(&self, calling_namespace: &str) -> TokenStream {
        let namespace = self.to_namespace_stream(calling_namespace);

        if self.generics.is_empty() {
            let name = format_ident(&self.name);
            quote! { #namespace#name }
        } else {
            let name = format_ident(&self.name[..self.name.len() - 2]);
            let generics = self.generics.iter().map(|g| g.to_tokens(calling_namespace));
            quote! { #namespace#name::<#(#generics),*> }
        }
    }

    pub fn to_abi_tokens(&self, calling_namespace: &str) -> TokenStream {
        let namespace = self.to_namespace_stream(calling_namespace);

        if self.generics.is_empty() {
            let name = format_abi_ident(&self.name);
            quote! { #namespace#name }
        } else {
            let name = format_abi_ident(&self.name[..self.name.len() - 2]);
            let generics = self.generics.iter().map(|g| g.to_tokens(calling_namespace));
            quote! { #namespace#name::<#(#generics),*> }
        }
    }

    // Note: ideally to_definition_tokens and to_abi_definiton_tokens would not be required
    // and we would simply use to_tokens and to_abi_tokens everywhere but Rust is really
    // weird in requiring `IVector<T>` in some places and `IVector::<T>` in others.
    pub fn to_definition_tokens(&self, calling_namespace: &str) -> TokenStream {
        let namespace = self.to_namespace_stream(calling_namespace);

        if self.generics.is_empty() {
            let name = format_ident(&self.name);
            quote! { #namespace#name }
        } else {
            let name = format_ident(&self.name[..self.name.len() - 2]);
            let generics = self.generics.iter().map(|g| g.to_tokens(calling_namespace));
            quote! { #namespace#name<#(#generics),*> }
        }
    }

    pub fn to_abi_definition_tokens(&self, calling_namespace: &str) -> TokenStream {
        let namespace = self.to_namespace_stream(calling_namespace);

        if self.generics.is_empty() {
            let name = format_abi_ident(&self.name);
            quote! { #namespace#name }
        } else {
            let name = format_abi_ident(&self.name[..self.name.len() - 2]);
            let generics = self.generics.iter().map(|g| g.to_tokens(calling_namespace));
            quote! { #namespace#name<#(#generics),*> }
        }
    }

    pub fn phantoms(&self) -> TokenStream {
        if self.generics.is_empty() {
            return TokenStream::new();
        }

        let phantoms = self.generics.iter().enumerate().map(|(count, generic)| {
            let name = format_ident!("__{}", count);
            let generic = generic.to_tokens("");
            quote! { #name: std::marker::PhantomData::<#generic>, }
        });

        TokenStream::from_iter(phantoms)
    }

    pub fn constraints(&self) -> TokenStream {
        let generics = self.generics.iter().map(|generic| {
            let generic = generic.to_tokens("");
            quote! { #generic: ::winrt::RuntimeType + 'static, }
        });

        TokenStream::from_iter(generics)
    }

    fn to_namespace_stream(&self, calling_namespace: &str) -> TokenStream {
        let mut tokens = Vec::new();

        let mut source = calling_namespace.split('.').peekable();
        let mut destination = self.namespace.split('.').peekable();

        while source.peek() == destination.peek() {
            if source.next().is_none() {
                break;
            }
            destination.next();
        }

        let count = source.count();

        if count > 0 {
            tokens.resize(tokens.len() + count, quote! {super::});
        }

        tokens.extend(destination.map(|destination| {
            let destination = format_ident(&to_snake(destination, MethodKind::Normal));
            quote! { #destination:: }
        }));

        TokenStream::from_iter(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::TableIndex;
    use crate::row::Row;

    #[test]
    fn runtime_name() {
        let mut type_name = TypeName {
            name: String::from("MyType"),
            namespace: String::from("Outer.Inner"),
            generics: vec![],
            def: TypeDef(Row {
                index: 0,
                table_index: TableIndex::InterfaceImpl,
                file_index: 0,
            }),
        };

        assert_eq!(type_name.runtime_name(), String::from("Outer.Inner.MyType"));

        type_name.generics = vec![TypeKind::Bool];

        assert_eq!(
            type_name.runtime_name(),
            String::from("Outer.Inner.MyType<Boolean>")
        );

        type_name.generics = vec![TypeKind::Bool, TypeKind::U8];

        assert_eq!(
            type_name.runtime_name(),
            String::from("Outer.Inner.MyType<Boolean, UInt8>")
        );
    }
}
