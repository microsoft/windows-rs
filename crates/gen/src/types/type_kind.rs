use crate::blob::Blob;
use crate::codes::*;
use crate::flags::*;
use crate::tables::*;
use crate::types::*;
use crate::{format_ident, TypeReader};

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
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
    pub fn signature(&self, reader: &TypeReader) -> String {
        match self {
            Self::Bool => "b1".to_owned(),
            Self::Char => "c2".to_owned(),
            Self::I8 => "i1".to_owned(),
            Self::U8 => "u1".to_owned(),
            Self::I16 => "i2".to_owned(),
            Self::U16 => "u2".to_owned(),
            Self::I32 => "i4".to_owned(),
            Self::U32 => "u4".to_owned(),
            Self::I64 => "i8".to_owned(),
            Self::U64 => "u8".to_owned(),
            Self::F32 => "f4".to_owned(),
            Self::F64 => "f8".to_owned(),
            Self::String => "string".to_owned(),
            Self::Object => "cinterface(IInspectable)".to_owned(),
            Self::Guid => "g16".to_owned(),
            Self::Class(name) => name.class_signature(reader),
            Self::Interface(name) => name.interface_signature(reader),
            Self::Enum(name) => name.enum_signature(reader),
            Self::Struct(name) => name.struct_signature(reader),
            Self::Delegate(name) => name.delegate_signature(reader),
            Self::Generic(_) => panic!("signature"),
        }
    }

    pub fn runtime_name(&self) -> String {
        match self {
            Self::Bool => "Boolean".to_owned(),
            Self::Char => "Char16".to_owned(),
            Self::I8 => "Int8".to_owned(),
            Self::U8 => "UInt8".to_owned(),
            Self::I16 => "Int16".to_owned(),
            Self::U16 => "UInt16".to_owned(),
            Self::I32 => "Int32".to_owned(),
            Self::U32 => "UInt32".to_owned(),
            Self::I64 => "Int64".to_owned(),
            Self::U64 => "UInt64".to_owned(),
            Self::F32 => "Single".to_owned(),
            Self::F64 => "Double".to_owned(),
            Self::String => "String".to_owned(),
            Self::Object => "Object".to_owned(),
            Self::Guid => "Guid".to_owned(),
            Self::Class(name) => name.runtime_name(),
            Self::Interface(name) => name.runtime_name(),
            Self::Enum(name) => name.runtime_name(),
            Self::Struct(name) => name.runtime_name(),
            Self::Delegate(name) => name.runtime_name(),
            Self::Generic(name) => name.to_owned(),
        }
    }

    fn from_type_name(reader: &TypeReader, name: TypeName) -> Self {
        match name.def.category(reader) {
            TypeCategory::Interface => TypeKind::Interface(name),
            TypeCategory::Class => TypeKind::Class(name),
            TypeCategory::Enum => TypeKind::Enum(name),
            TypeCategory::Struct => TypeKind::Struct(name),
            TypeCategory::Delegate => TypeKind::Delegate(name),
        }
    }

    pub fn from_type_def(
        reader: &TypeReader,
        def: TypeDef,
        _generics: &Vec<TypeKind>,
        calling_namespace: &str,
    ) -> Self {
        Self::from_type_name(
            reader,
            TypeName::from_type_def(reader, def, calling_namespace),
        )
    }

    pub fn from_type_ref(
        reader: &TypeReader,
        type_ref: TypeRef,
        generics: &Vec<TypeKind>,
        calling_namespace: &str,
    ) -> Self {
        let (namespace, name) = type_ref.name(reader);
        if (namespace, name) == ("System", "Guid") {
            TypeKind::Guid
        } else {
            Self::from_type_def(
                reader,
                reader.resolve_type_def((namespace, name)),
                generics,
                calling_namespace,
            )
        }
    }

    pub fn from_type_spec(
        reader: &TypeReader,
        spec: TypeSpec,
        generics: &Vec<TypeKind>,
        calling_namespace: &str,
    ) -> Self {
        TypeKind::Interface(TypeName::from_type_spec(
            reader,
            spec,
            generics,
            calling_namespace,
        ))
    }

    fn from_type_def_or_ref(
        reader: &TypeReader,
        code: TypeDefOrRef,
        generics: &Vec<TypeKind>,
        calling_namespace: &str,
    ) -> Self {
        match code {
            TypeDefOrRef::TypeRef(value) => {
                Self::from_type_ref(reader, value, generics, calling_namespace)
            }
            TypeDefOrRef::TypeDef(value) => {
                Self::from_type_def(reader, value, generics, calling_namespace)
            }
            TypeDefOrRef::TypeSpec(value) => {
                Self::from_type_spec(reader, value, generics, calling_namespace)
            }
        }
    }

    pub fn from_blob(blob: &mut Blob, generics: &Vec<TypeKind>, calling_namespace: &str) -> Self {
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
                TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index),
                generics,
                calling_namespace,
            ),
            0x13 => generics[blob.read_unsigned() as usize].clone(),
            0x15 => Self::from_type_name(
                blob.reader,
                TypeName::from_type_spec_blob(blob, generics, calling_namespace),
            ),
            _ => panic!("TypeKind::from_blob"),
        }
    }

    pub fn from_field(reader: &TypeReader, field: Field, calling_namespace: &str) -> Self {
        let mut blob = field.sig(reader);
        blob.read_unsigned();
        blob.read_modifiers();
        Self::from_blob(&mut blob, &Vec::new(), calling_namespace)
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        match self {
            TypeKind::Class(name) => name.dependencies(),
            TypeKind::Interface(name) => name.dependencies(),
            TypeKind::Enum(name) => name.dependencies(),
            TypeKind::Struct(name) => name.dependencies(),
            TypeKind::Delegate(name) => name.dependencies(),
            _ => Vec::new(),
        }
    }

    pub fn to_tokens(&self) -> TokenStream {
        match self {
            Self::Bool => quote! { bool },
            Self::Char => quote! { u16 },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::String => quote! { ::winrt::HString },
            Self::Object => quote! { ::winrt::Object },
            Self::Guid => quote! { ::winrt::Guid },
            Self::Class(name) => name.tokens.clone(),
            Self::Interface(name) => name.tokens.clone(),
            Self::Enum(name) => name.tokens.clone(),
            Self::Struct(name) => name.tokens.clone(),
            Self::Delegate(name) => name.tokens.clone(),
            Self::Generic(name) => {
                let name = format_ident(name);
                quote! { #name }
            }
        }
    }

    pub fn to_abi_tokens(&self) -> TokenStream {
        match self {
            Self::Bool => quote! { bool, },
            Self::Char => quote! { u16, },
            Self::I8 => quote! { i8, },
            Self::U8 => quote! { u8, },
            Self::I16 => quote! { i16, },
            Self::U16 => quote! { u16, },
            Self::I32 => quote! { i32, },
            Self::U32 => quote! { u32, },
            Self::I64 => quote! { i64, },
            Self::U64 => quote! { u64, },
            Self::F32 => quote! { f32, },
            Self::F64 => quote! { f64, },
            Self::Guid => quote! { ::winrt::Guid, },
            Self::String => {
                quote! { <::winrt::HString as ::winrt::AbiTransferable>::Abi, }
            }
            Self::Object => {
                quote! { <::winrt::Object as ::winrt::AbiTransferable>::Abi, }
            }
            Self::Generic(name) => {
                let name = format_ident(name);
                quote! { <#name as ::winrt::AbiTransferable>::Abi, }
            }
            Self::Class(name)
            | Self::Interface(name)
            | Self::Delegate(name)
            | Self::Enum(name)
            | Self::Struct(name) => {
                let name = &name.tokens;
                quote! { <#name as ::winrt::AbiTransferable>::Abi, }
            }
        }
    }

    pub fn primitive(&self) -> bool {
        match self {
            Self::Bool
            | Self::Char
            | Self::I8
            | Self::U8
            | Self::I16
            | Self::U16
            | Self::I32
            | Self::U32
            | Self::I64
            | Self::U64
            | Self::F32
            | Self::F64 => true,
            _ => false,
        }
    }
}
