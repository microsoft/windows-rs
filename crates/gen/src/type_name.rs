use crate::*;
use squote::{format_ident, quote, Ident, Literal, TokenStream};
use std::iter::FromIterator;
use winmd::Decode;

/// A type's name including module namespace and generics
#[derive(Debug, Clone)]
pub struct TypeName {
    /// The type's module namespace as a period separated string
    ///
    /// e.g. "Outer.Inner"
    pub namespace: String,
    /// The type's unqualified name without generics as a string
    ///
    /// e.g. "MyType"
    pub name: String,
    /// A collection of the types generics
    pub generics: Vec<TypeKind>,
    /// The type definition for this type
    pub def: winmd::TypeDef,

    // The namespace of the type being tokenized.
    calling_namespace: String,
}

impl TypeName {
    fn new(
        namespace: String,
        name: String,
        generics: Vec<TypeKind>,
        def: winmd::TypeDef,
        calling_namespace: &str,
    ) -> Self {
        Self {
            namespace,
            name,
            generics,
            def,
            calling_namespace: calling_namespace.to_owned(),
        }
    }

    pub fn gen_constraint(&self) -> TokenStream {
        TokenStream::from_iter(self.generics.iter().map(|generic| {
            let generic = generic.gen();
            quote! { #generic: ::winrt::RuntimeType + 'static, }
        }))
    }

    pub fn from_type_def_or_ref(
        reader: &winmd::TypeReader,
        code: winmd::TypeDefOrRef,
        generics: &[TypeKind],
        calling_namespace: &str,
    ) -> Self {
        match code {
            winmd::TypeDefOrRef::TypeRef(value) => {
                Self::from_type_ref(reader, value, calling_namespace)
            }
            winmd::TypeDefOrRef::TypeDef(value) => {
                Self::from_type_def(reader, value, calling_namespace)
            }
            winmd::TypeDefOrRef::TypeSpec(value) => {
                Self::from_type_spec(reader, value, generics, calling_namespace)
            }
        }
    }

    fn from_type_ref(
        reader: &winmd::TypeReader,
        type_ref: winmd::TypeRef,
        calling_namespace: &str,
    ) -> Self {
        let (namespace, name) = type_ref.name(reader);
        Self::from_type_def(
            reader,
            reader.resolve_type_def((namespace, name)),
            calling_namespace,
        )
    }

    pub fn from_type_def(
        reader: &winmd::TypeReader,
        def: winmd::TypeDef,
        calling_namespace: &str,
    ) -> Self {
        let (namespace, name) = def.name(reader);
        let owned_namespace = namespace.to_string();
        let owned_name = name.to_string();
        let mut generics = Vec::new();

        for generic in def.generics(reader) {
            let name = generic.name(reader).to_string();
            generics.push(TypeKind::Generic(name));
        }

        let calling_namespace = if calling_namespace.is_empty() {
            namespace
        } else {
            calling_namespace
        };

        Self::new(
            owned_namespace,
            owned_name,
            generics,
            def,
            calling_namespace,
        )
    }

    pub fn from_type_spec_blob(
        blob: &mut winmd::Blob,
        generics: &[TypeKind],
        calling_namespace: &str,
    ) -> Self {
        blob.read_unsigned();
        let def =
            winmd::TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index).resolve(blob.reader);
        let mut args = Vec::with_capacity(blob.read_unsigned() as usize);

        for _ in 0..args.capacity() {
            args.push(TypeKind::from_blob(blob, generics, calling_namespace));
        }
        let (namespace, name) = def.name(blob.reader);
        let namespace = namespace.to_string();
        let name = name.to_string();
        let generics = args;

        Self::new(namespace, name, generics, def, calling_namespace)
    }

    pub fn from_type_spec(
        reader: &winmd::TypeReader,
        spec: winmd::TypeSpec,
        generics: &[TypeKind],
        calling_namespace: &str,
    ) -> Self {
        let mut blob = spec.sig(reader);
        blob.read_unsigned();
        Self::from_type_spec_blob(&mut blob, generics, calling_namespace)
    }

    pub fn gen_signature(&self, signature: &str) -> TokenStream {
        let signature = Literal::byte_string(signature.as_bytes());

        if self.generics.is_empty() {
            return quote! { ::winrt::ConstBuffer::from_slice(#signature) };
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
                let string = string.push_other(<#g as ::winrt::RuntimeType>::SIGNATURE);
                #semi
            }
        });

        quote! {
            let string = ::winrt::ConstBuffer::new();
            let string = string.push_slice(b"pinterface(");
            let string = string.push_slice(#signature);
            let string = string.push_slice(b";");
            #(#generics)*
            string.push_slice(b")")
        }
    }

    pub fn gen_guid(&self, guid: &TypeGuid) -> TokenStream {
        if self.generics.is_empty() {
            let guid = guid.gen();

            return quote! {
                ::winrt::Guid::from_values(#guid)
            };
        }

        let typ = self.gen();

        quote! {
            ::winrt::Guid::from_signature(<#typ as ::winrt::RuntimeType>::SIGNATURE)
        }
    }

    pub fn interface_signature(&self, reader: &winmd::TypeReader) -> String {
        let guid = TypeGuid::from_type_def(reader, self.def);

        if self.generics.is_empty() {
            format!("{{{:#?}}}", guid)
        } else {
            let mut result = format!("pinterface({{{:#?}}}", guid);

            for generic in &self.generics {
                result.push(';');
                result.push_str(&generic.signature(reader));
            }

            result.push(')');
            result
        }
    }

    pub fn class_signature(&self, reader: &winmd::TypeReader) -> String {
        let default = self
            .def
            .interfaces(reader)
            .find(|i| i.is_default(reader))
            .unwrap();

        let default = TypeName::from_type_def_or_ref(
            reader,
            default.interface(reader),
            &self.generics,
            &self.calling_namespace,
        );

        format!(
            "rc({}.{};{})",
            self.namespace,
            self.name,
            default.interface_signature(reader)
        )
    }

    pub fn enum_signature(&self, reader: &winmd::TypeReader) -> String {
        format!(
            "enum({}.{};{})",
            self.namespace,
            self.name,
            self.enum_type(reader)
        )
    }

    fn enum_type(&self, reader: &winmd::TypeReader) -> &str {
        for field in self.def.fields(reader) {
            for constant in field.constants(reader) {
                match constant.value_type(reader) {
                    winmd::ElementType::I32 => return "i4",
                    winmd::ElementType::U32 => return "u4",
                    _ => panic!("Invalid enum type"),
                };
            }
        }

        panic!("Invalid enum");
    }

    pub fn struct_signature(&self, reader: &winmd::TypeReader) -> String {
        let mut result = format!("struct({}.{}", self.namespace, self.name);

        for field in self.def.fields(reader) {
            result.push(';');
            result.push_str(
                &TypeKind::from_field(reader, field, &self.calling_namespace).signature(reader),
            );
        }

        result.push(')');
        result
    }

    pub fn delegate_signature(&self, reader: &winmd::TypeReader) -> String {
        if self.generics.is_empty() {
            format!("delegate({})", self.interface_signature(reader))
        } else {
            self.interface_signature(reader)
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
        gen_format(&self.name, Some(&namespace), &self.generics, format_ident)
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

    pub fn gen_binding_abi(&self) -> TokenStream {
        let namespace = gen_binding_namespace(&self.namespace);
        gen_format(
            &self.name,
            Some(&namespace),
            &self.generics,
            format_abi_ident,
        )
    }

    pub fn gen_binding(&self) -> TokenStream {
        let namespace = gen_binding_namespace(&self.namespace);
        gen_format(&self.name, Some(&namespace), &self.generics, format_ident)
    }

    /// Create definition tokens
    ///
    /// For example: `Vector::<OtherType>`
    ///
    /// Note: ideally gen_definition and to_abi_definiton_tokens would not be required
    /// and we would simply use gen and gen_abi everywhere but Rust is really
    /// weird in requiring `IVector<T>` in some places and `IVector::<T>` in others.
    pub fn gen_definition(&self) -> TokenStream {
        gen_format(&self.name, None, &self.generics, format_ident)
    }

    /// Create abi definition tokens
    ///
    /// For example: `abi_Vector::<OtherType>`
    pub fn gen_abi_definition(&self) -> TokenStream {
        gen_format(&self.name, None, &self.generics, format_abi_ident)
    }

    pub fn phantoms(&self) -> TokenStream {
        if self.generics.is_empty() {
            return TokenStream::new();
        }

        let phantoms = self.generics.iter().enumerate().map(|(count, generic)| {
            let name = format_ident!("t{}__", count);
            let generic = generic.gen();
            quote! { #name: ::std::marker::PhantomData::<#generic>, }
        });

        TokenStream::from_iter(phantoms)
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
        (&self.namespace, &self.name, &self.generics, &self.def).cmp(&(
            &other.namespace,
            &other.name,
            &other.generics,
            &other.def,
        ))
    }
}

fn format_abi_ident(name: &str) -> Ident {
    squote::format_ident!("abi_{}", name)
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn runtime_name() {
        let mut type_name = TypeName::new(
            String::from("Outer.Inner"),
            String::from("MyType"),
            vec![],
            winmd::TypeDef(winmd::Row {
                index: 0,
                table_index: winmd::TableIndex::InterfaceImpl,
                file_index: 0,
            }),
            "Outer.Inner",
        );

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

    #[test]
    fn signatures() {
        let reader = &winmd::TypeReader::from_os();

        // Primitive signatures
        assert!(TypeKind::Bool.signature(reader) == "b1");
        assert!(TypeKind::Char.signature(reader) == "c2");
        assert!(TypeKind::I8.signature(reader) == "i1");
        assert!(TypeKind::U8.signature(reader) == "u1");
        assert!(TypeKind::I16.signature(reader) == "i2");
        assert!(TypeKind::U16.signature(reader) == "u2");
        assert!(TypeKind::I32.signature(reader) == "i4");
        assert!(TypeKind::U32.signature(reader) == "u4");
        assert!(TypeKind::I64.signature(reader) == "i8");
        assert!(TypeKind::U64.signature(reader) == "u8");
        assert!(TypeKind::F32.signature(reader) == "f4");
        assert!(TypeKind::F64.signature(reader) == "f8");
        assert!(TypeKind::String.signature(reader) == "string");
        assert!(TypeKind::Object.signature(reader) == "cinterface(IInspectable)");
        assert!(TypeKind::Guid.signature(reader) == "g16");

        // Non-generic interface signature
        let def = reader.resolve_type_def(("Windows.Foundation", "IAsyncAction"));
        let name = Type::from_type_def(reader, def).name().clone();
        assert!(
            TypeKind::Interface(name).signature(reader) == "{5a648006-843a-4da9-865b-9d26e5dfad7b}"
        );

        // Generic interface signature
        let def = reader.resolve_type_def(("Windows.Foundation.Collections", "IVector`1"));
        let mut name = Type::from_type_def(reader, def).name().clone();
        name.generics.clear();
        name.generics.push(TypeKind::I32);
        assert!(
            TypeKind::Interface(name).signature(reader)
                == "pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};i4)"
        );

        // Signed enum signature
        let def = reader.resolve_type_def(("Windows.Foundation", "AsyncStatus"));
        let name = Type::from_type_def(reader, def).name().clone();
        assert!(
            TypeKind::Enum(name).signature(reader) == "enum(Windows.Foundation.AsyncStatus;i4)"
        );

        // Unsigned enum signature
        let def = reader.resolve_type_def((
            "Windows.ApplicationModel.Appointments",
            "AppointmentDaysOfWeek",
        ));
        let name = Type::from_type_def(reader, def).name().clone();
        assert!(
            TypeKind::Enum(name).signature(reader)
                == "enum(Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek;u4)"
        );

        // Non-generic delegate signature
        let def = reader.resolve_type_def(("Windows.Foundation", "AsyncActionCompletedHandler"));
        let name = Type::from_type_def(reader, def).name().clone();
        assert!(
            TypeKind::Delegate(name).signature(reader)
                == "delegate({a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7})"
        );

        // Generic delegate signature
        let stringable = reader.resolve_type_def(("Windows.Foundation", "IStringable"));
        let stringable = Type::from_type_def(reader, stringable).name().clone();

        let def = reader.resolve_type_def(("Windows.Foundation", "EventHandler`1"));
        let mut name = Type::from_type_def(reader, def).name().clone();
        name.generics.clear();
        name.generics.push(TypeKind::Interface(stringable));
        assert!(
             TypeKind::Delegate(name).signature(reader) == "pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f};{96369f54-8eb6-48f0-abce-c1b211e627c3})"
        );

        // Class signature
        let def = reader.resolve_type_def(("Windows.Foundation", "Uri"));
        let name = Type::from_type_def(reader, def).name().clone();
        assert!(
            TypeKind::Class(name).signature(reader)
                == "rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})"
        );

        // Class with generic default interface signature
        let def = reader.resolve_type_def(("Windows.Foundation", "WwwFormUrlDecoder"));
        let name = Type::from_type_def(reader, def).name().clone();
        assert!(
             TypeKind::Class(name).signature(reader)
                == "rc(Windows.Foundation.WwwFormUrlDecoder;{d45a0451-f225-4542-9296-0e1df5d254df})"
        );

        // Simple struct
        let def = reader.resolve_type_def(("Windows.Foundation", "Rect"));
        let name = Type::from_type_def(reader, def).name().clone();
        assert!(
            TypeKind::Struct(name).signature(reader)
                == "struct(Windows.Foundation.Rect;f4;f4;f4;f4)"
        );
    }
}
