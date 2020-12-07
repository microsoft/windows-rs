use crate::*;
use squote::{quote, TokenStream};
use winmd::Decode;

// TODO: TypeKind should be a struct that also stores pointer count
// and array rank.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TypeKind {
    Void,
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
    ISize,
    USize,
    String,
    Object,
    Guid,
    Class(TypeName),
    Interface(TypeName),
    Enum(TypeName),
    Struct(TypeName),
    Delegate(TypeName),
    Generic(&'static str),
}

impl TypeKind {
    pub fn signature(&self) -> String {
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
            Self::Class(name) => name.class_signature(),
            Self::Interface(name) => name.interface_signature(),
            Self::Enum(name) => name.enum_signature(),
            Self::Struct(name) => name.struct_signature(),
            Self::Delegate(name) => name.delegate_signature(),
            _ => panic!("TypeKind::signature"),
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
            Self::Generic(name) => name.to_string(),
            _ => panic!("TypeKind::runtime_name"),
        }
    }

    fn from_type_name(name: TypeName) -> Self {
        match name.def.category() {
            winmd::TypeCategory::Interface => TypeKind::Interface(name),
            winmd::TypeCategory::Class => TypeKind::Class(name),
            winmd::TypeCategory::Enum => TypeKind::Enum(name),
            winmd::TypeCategory::Struct => TypeKind::Struct(name),
            winmd::TypeCategory::Delegate => TypeKind::Delegate(name),
        }
    }

    pub fn from_type_def(def: &winmd::TypeDef, calling_namespace: &'static str) -> Self {
        Self::from_type_name(TypeName::from_type_def(def, calling_namespace))
    }

    pub fn from_type_ref(type_ref: &winmd::TypeRef, calling_namespace: &'static str) -> Self {
        let (namespace, name) = type_ref.name();
        if (namespace, name) == ("System", "Guid") {
            TypeKind::Guid
        } else {
            Self::from_type_def(
                &type_ref.reader.resolve_type_def((namespace, name)),
                calling_namespace,
            )
        }
    }

    pub fn from_type_spec(
        spec: &winmd::TypeSpec,
        generics: &[TypeKind],
        calling_namespace: &'static str,
    ) -> Self {
        TypeKind::Interface(TypeName::from_type_spec(spec, generics, calling_namespace))
    }

    fn from_type_def_or_ref(
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

    pub fn from_blob(
        blob: &mut winmd::Blob,
        generics: &[TypeKind],
        calling_namespace: &'static str,
    ) -> (Self, u32) {
        blob.read_expected(0x1D);

        let mut pointers = 0;

        while blob.read_expected(0x0f) {
            pointers += 1;
        }

        blob.read_modifiers();

        let kind = match blob.read_unsigned() {
            0x01 => TypeKind::Void,
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
            0x18 => TypeKind::ISize,
            0x19 => TypeKind::USize,
            0x0E => TypeKind::String,
            0x1C => TypeKind::Object,
            0x11 | 0x12 => {
                let def =
                    winmd::TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index);

                if def.name().0.is_empty() {
                    // TODO: handle nested types
                    TypeKind::Bool
                } else {
                    Self::from_type_def_or_ref(&def, generics, calling_namespace)
                }
            }
            0x13 => generics[blob.read_unsigned() as usize].clone(),
            0x14 => {
                // type
                // rank (dimensions)
                // bounds count
                // bound
                TypeKind::Bool
            }
            0x15 => Self::from_type_name(TypeName::from_type_spec_blob(
                blob,
                generics,
                calling_namespace,
            )),
            unused => panic!("TypeKind::from_blob 0x{:X}", unused),
        };

        (kind, pointers)
    }

    pub fn from_field(field: &winmd::Field, calling_namespace: &'static str) -> (Self, u32) {
        let mut blob = field.sig();
        blob.read_unsigned();
        blob.read_modifiers();
        Self::from_blob(&mut blob, &Vec::new(), calling_namespace)
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        match self {
            TypeKind::Class(name) => name.dependencies(),
            TypeKind::Interface(name) => name.dependencies(),
            TypeKind::Enum(name) => name.dependencies(),
            TypeKind::Struct(name) => name.dependencies(),
            TypeKind::Delegate(name) => name.dependencies(),
            _ => Vec::new(),
        }
    }

    pub fn gen(&self) -> TokenStream {
        match self {
            Self::Void => quote! { ::std::ffi::c_void },
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
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::String => quote! { ::winrt::HString },
            Self::Object => quote! { ::winrt::Object },
            Self::Guid => quote! { ::winrt::Guid },
            Self::Class(name) => name.gen(),
            Self::Interface(name) => name.gen(),
            Self::Enum(name) => name.gen(),
            Self::Struct(name) => name.gen(),
            Self::Delegate(name) => name.gen(),
            Self::Generic(name) => {
                let name = format_ident(name);
                quote! { #name }
            }
        }
    }

    pub fn gen_full(&self) -> TokenStream {
        match self {
            Self::Void => quote! { ::std::ffi::c_void },
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
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::String => quote! { ::winrt::HString },
            Self::Object => quote! { ::winrt::Object },
            Self::Guid => quote! { ::winrt::Guid },
            Self::Class(name) => name.gen_full(),
            Self::Interface(name) => name.gen_full(),
            Self::Enum(name) => name.gen_full(),
            Self::Struct(name) => name.gen_full(),
            Self::Delegate(name) => name.gen_full(),
            Self::Generic(name) => {
                let name = format_ident(name);
                quote! { #name }
            }
        }
    }

    pub fn gen_field(&self) -> TokenStream {
        let mut tokens = self.gen();

        if let Self::Interface(name) = self {
            if name.name == "IReference`1" && name.namespace == "Windows.Foundation" {
                tokens = quote! {
                    ::std::option::Option<#tokens>
                }
            }
        }

        tokens
    }

    pub fn gen_abi(&self) -> TokenStream {
        match self {
            Self::Void => quote! { ::std::ffi::c_void },
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
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::Guid => quote! { ::winrt::Guid },
            Self::String
            | Self::Object
            | Self::Class(_)
            | Self::Interface(_)
            | Self::Delegate(_) => {
                quote! { ::winrt::RawPtr }
            }
            Self::Generic(name) => {
                let name = format_ident(name);
                quote! { <#name as ::winrt::Abi>::Abi }
            }
            Self::Enum(name) | Self::Struct(name) => {
                let name = name.gen();
                quote! { <#name as ::winrt::Abi>::Abi }
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
