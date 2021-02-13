use crate::*;
use squote::{quote, Ident, Literal, TokenStream};
use std::iter::FromIterator;
use winmd::Decode;

/// A type's name including module namespace and generics
#[derive(Debug, Clone)]
pub struct TypeName {
    /// The type's module namespace as a period separated string
    ///
    /// e.g. "Outer.Inner"
    pub namespace: &'static str,
    /// The type's unqualified name without generics as a string
    ///
    /// e.g. "MyType"
    pub name: &'static str,
    /// A collection of the types generics
    pub generics: Vec<TypeKind>,
    /// The type definition for this type
    pub def: winmd::TypeDef,

    // The namespace of the type being tokenized.
    calling_namespace: &'static str,
}

impl TypeName {
    pub fn new(
        def: &winmd::TypeDef,
        generics: Vec<TypeKind>,
        calling_namespace: &'static str,
    ) -> Self {
        Self {
            namespace: def.namespace(),
            name: def.name(),
            generics,
            def: *def,
            calling_namespace,
        }
    }

    pub fn gen_constraint(&self) -> TokenStream {
        TokenStream::from_iter(self.generics.iter().map(|generic| {
            let generic = generic.gen();
            quote! { #generic: ::windows::RuntimeType + 'static, }
        }))
    }

    pub fn from_type_def_or_ref(
        code: &winmd::TypeDefOrRef,
        generics: &[TypeKind],
        calling_namespace: &'static str,
    ) -> Self {
        match code {
            winmd::TypeDefOrRef::TypeRef(value) => Self::from_type_ref(value, calling_namespace),
            winmd::TypeDefOrRef::TypeDef(value) => Self::from_type_def(value, calling_namespace),
            winmd::TypeDefOrRef::TypeSpec(value) => {
                Self::from_type_spec(value, generics, calling_namespace)
            }
        }
    }

    fn from_type_ref(type_ref: &winmd::TypeRef, calling_namespace: &'static str) -> Self {
        Self::from_type_def(&type_ref.resolve(), calling_namespace)
    }

    pub fn from_type_def(def: &winmd::TypeDef, calling_namespace: &'static str) -> Self {
        let mut generics = Vec::new();

        for generic in def.generics() {
            generics.push(TypeKind::Generic(generic.name()));
        }

        Self::new(def, generics, calling_namespace)
    }

    pub fn from_type_spec_blob(
        blob: &mut winmd::Blob,
        generics: &[TypeKind],
        calling_namespace: &'static str,
    ) -> Self {
        blob.read_unsigned();

        let def = winmd::TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index)
            .resolve();

        let mut args = Vec::with_capacity(blob.read_unsigned() as usize);

        for _ in 0..args.capacity() {
            let t = Type::from_blob(blob, None, generics, calling_namespace, false).unwrap();
            args.push(t.kind);
        }

        Self::new(&def, args, calling_namespace)
    }

    pub fn from_type_spec(
        spec: &winmd::TypeSpec,
        generics: &[TypeKind],
        calling_namespace: &'static str,
    ) -> Self {
        let mut blob = spec.blob();
        blob.read_unsigned();
        Self::from_type_spec_blob(&mut blob, generics, calling_namespace)
    }

    pub fn gen_signature(&self, signature: &str) -> TokenStream {
        let signature = Literal::byte_string(signature.as_bytes());

        if self.generics.is_empty() {
            return quote! { ::windows::ConstBuffer::from_slice(#signature) };
        }

        let generics = self.generics.iter().enumerate().map(|(index, g)| {
            let g = g.gen();
            let semi = if index != self.generics.len() - 1 {
                Some(quote! {
                    let string = string.push_slice(b";");
                })
            } else {
                None
            };

            quote! {
                let string = string.push_other(<#g as ::windows::RuntimeType>::SIGNATURE);
                #semi
            }
        });

        quote! {
            let string = ::windows::ConstBuffer::new();
            let string = string.push_slice(b"pinterface(");
            let string = string.push_slice(#signature);
            let string = string.push_slice(b";");
            #(#generics)*
            string.push_slice(b")")
        }
    }

    pub fn gen_guid(&self, guid: &Guid) -> TokenStream {
        if self.generics.is_empty() {
            let guid = guid.gen();

            return quote! {
                ::windows::Guid::from_values(#guid)
            };
        }

        let typ = self.gen();

        quote! {
            ::windows::Guid::from_signature(<#typ as ::windows::RuntimeType>::SIGNATURE)
        }
    }

    pub fn interface_signature(&self) -> String {
        let guid = self.def.guid();

        if self.generics.is_empty() {
            format!("{{{:#?}}}", guid)
        } else {
            let mut result = format!("pinterface({{{:#?}}}", guid);

            for generic in &self.generics {
                result.push(';');
                result.push_str(&generic.signature());
            }

            result.push(')');
            result
        }
    }

    pub fn class_signature(&self) -> String {
        let default = self.def.interfaces().find(|i| i.is_default()).unwrap();

        let default = Self::from_type_def_or_ref(
            &default.interface(),
            &self.generics,
            &self.calling_namespace,
        );

        format!(
            "rc({}.{};{})",
            self.namespace,
            self.name,
            default.interface_signature()
        )
    }

    pub fn enum_signature(&self) -> String {
        format!(
            "enum({}.{};{})",
            self.namespace,
            self.name,
            self.enum_type()
        )
    }

    fn enum_type(&self) -> &str {
        for field in self.def.fields() {
            if let Some(constant) = field.constant() {
                match constant.value_type() {
                    winmd::ElementType::I32 => return "i4",
                    winmd::ElementType::U32 => return "u4",
                    _ => unexpected!(),
                };
            }
        }

        unexpected!();
    }

    pub fn struct_signature(&self) -> String {
        let mut result = format!("struct({}.{}", self.namespace, self.name);

        for field in self.def.fields() {
            result.push(';');
            let t = Type::from_field(&field, &self.calling_namespace);
            result.push_str(&t.kind.signature());
        }

        result.push(')');
        result
    }

    pub fn delegate_signature(&self) -> String {
        if self.generics.is_empty() {
            format!("delegate({})", self.interface_signature())
        } else {
            self.interface_signature()
        }
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

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        std::iter::once(self.def)
            .chain(self.generics.iter().flat_map(|i| i.dependencies()))
            .collect()
    }

    /// Create tokens
    ///
    /// For example: `Vector<OtherType>`
    pub fn gen(&self) -> TokenStream {
        let namespace = gen_namespace(&self.namespace, &self.calling_namespace);
        gen_format(&self.name, Some(&namespace), &self.generics, to_ident)
    }

    pub fn gen_full(&self) -> TokenStream {
        let namespace = gen_full_namespace(&self.namespace);
        gen_format(&self.name, Some(&namespace), &self.generics, to_ident)
    }

    pub fn gen_full_abi(&self) -> TokenStream {
        let namespace = gen_full_namespace(&self.namespace);
        gen_format(
            &self.name,
            Some(&namespace),
            &self.generics,
            format_abi_ident,
        )
    }

    /// Create abi tokens
    ///
    /// For example: `abi_Vector<OtherType>`
    pub fn gen_abi(&self) -> TokenStream {
        let namespace = gen_namespace(&self.namespace, &self.calling_namespace);
        gen_format(
            &self.name,
            Some(&namespace),
            &self.generics,
            format_abi_ident,
        )
    }

    /// Create definition tokens
    ///
    /// For example: `Vector::<OtherType>`
    ///
    /// Note: ideally gen_definition and to_abi_definiton_tokens would not be required
    /// and we would simply use gen and gen_abi everywhere but Rust is really
    /// weird in requiring `IVector<T>` in some places and `IVector::<T>` in others.
    pub fn gen_definition(&self) -> TokenStream {
        gen_format(&self.name, None, &self.generics, to_ident)
    }

    /// Create abi definition tokens
    ///
    /// For example: `abi_Vector::<OtherType>`
    pub fn gen_abi_definition(&self) -> TokenStream {
        gen_format(&self.name, None, &self.generics, format_abi_ident)
    }

    pub fn phantoms(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        for generic in &self.generics {
            let generic = generic.gen();
            tokens.combine(&quote! { ::std::marker::PhantomData::<#generic>, });
        }

        tokens
    }
}

impl PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace
            && self.name == other.name
            && self.generics == other.generics
            && self.def == other.def
    }
}

impl Eq for TypeName {}

impl PartialOrd for TypeName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypeName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.namespace, &self.name, &self.generics).cmp(&(
            &other.namespace,
            &other.name,
            &other.generics,
        ))
    }
}

fn format_abi_ident(name: &str) -> Ident {
    squote::format_ident!("{}_abi", name)
}

fn gen_format<F>(
    name: &str,
    namespace: Option<&TokenStream>,
    generics: &[TypeKind],
    format: F,
) -> TokenStream
where
    F: FnOnce(&str) -> Ident,
{
    if generics.is_empty() {
        let name = format(name);
        quote! { #namespace#name }
    } else {
        let colon_separated = namespace.map(|_| quote! { :: });
        let name = format(&name[..name.len() - 2]);
        let generics = generics.iter().map(|g| g.gen());
        quote! { #namespace#name#colon_separated<#(#generics),*> }
    }
}
