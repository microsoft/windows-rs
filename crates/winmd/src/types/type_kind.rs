use crate::blob::Blob;
use crate::codes::*;
use crate::flags::*;
use crate::tables::*;
use crate::types::*;
use crate::{write_ident, TypeReader};

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone, PartialEq)]
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
    fn from_type_name(reader: &TypeReader, name: TypeName) -> Self {
        match name.def.category(reader) {
            TypeCategory::Interface => TypeKind::Interface(name),
            TypeCategory::Class => TypeKind::Class(name),
            TypeCategory::Enum => TypeKind::Enum(name),
            TypeCategory::Struct => TypeKind::Struct(name),
            TypeCategory::Delegate => TypeKind::Delegate(name),
        }
    }

    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        Self::from_type_name(reader, TypeName::from_type_def(reader, def))
    }

    pub fn from_type_ref(reader: &TypeReader, type_ref: TypeRef) -> Self {
        let (namespace, name) = type_ref.name(reader);
        if (namespace, name) == ("System", "Guid") {
            TypeKind::Guid
        } else {
            Self::from_type_def(reader, reader.resolve((namespace, name)))
        }
    }

    pub fn from_type_spec(reader: &TypeReader, spec: TypeSpec) -> Self {
        TypeKind::Interface(TypeName::from_type_spec(reader, spec))
    }

    pub fn from_type_def_or_ref(reader: &TypeReader, code: TypeDefOrRef) -> Self {
        match code {
            TypeDefOrRef::TypeRef(value) => Self::from_type_ref(reader, value),
            TypeDefOrRef::TypeDef(value) => Self::from_type_def(reader, value),
            TypeDefOrRef::TypeSpec(value) => Self::from_type_spec(reader, value),
        }
    }

    pub fn from_blob(blob: &mut Blob, generics: &[TypeKind]) -> Self {
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
            0x15 => {
                Self::from_type_name(blob.reader, TypeName::from_type_spec_blob(blob, generics))
            }
            _ => panic!("TypeKind::from_blob"),
        }
    }

    pub fn from_field(reader: &TypeReader, field: Field) -> Self {
        let mut blob = field.sig(reader);
        blob.read_unsigned();
        blob.read_modifiers();
        Self::from_blob(&mut blob, &Vec::new())
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

    pub fn to_stream(&self) -> TokenStream {
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
            Self::String => quote! { winrt::HString },
            Self::Object => quote! { winrt::Object },
            Self::Guid => quote! { winrt::Guid },
            Self::Class(name) => name.ident(),
            Self::Interface(name) => name.ident(),
            Self::Enum(name) => name.ident(),
            Self::Struct(name) => name.ident(),
            Self::Delegate(name) => name.ident(),
            Self::Generic(name) => {
                let name = write_ident(name);
                quote! { #name }
            }
        }
    }
}
