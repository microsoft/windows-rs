use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Constant {
    pub name: TypeName,
    pub field: winmd::Field,
}

impl Constant {
    pub fn new(name: TypeName, field: &winmd::Field) -> Self {
        Self {
            name,
            field: *field,
        }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.field.name();

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/90
        if name == "NaN" || name == "POSITIVE_INFINITY" || name == "NEGATIVE_INFINITY" {
            return quote! {};
        }

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/88
        if self.field.constant().is_none() {
            return quote! {};
        }

        let constant = self
            .field
            .constant()
            .expect(&format!("Missing constant value: {}", name));

        let mut value = constant.value();

        let value = match constant.value_type() {
            winmd::ElementType::I8 => ConstantValue::I8(value.read_i8()),
            winmd::ElementType::U8 => ConstantValue::U8(value.read_u8()),
            winmd::ElementType::I16 => ConstantValue::I16(value.read_i16()),
            winmd::ElementType::U16 => ConstantValue::U16(value.read_u16()),
            winmd::ElementType::I32 => ConstantValue::I32(value.read_i32()),
            winmd::ElementType::U32 => ConstantValue::U32(value.read_u32()),
            winmd::ElementType::I64 => ConstantValue::I64(value.read_i64()),
            winmd::ElementType::U64 => ConstantValue::U64(value.read_u64()),
            winmd::ElementType::F32 => ConstantValue::F32(value.read_f32()),
            winmd::ElementType::F64 => ConstantValue::F64(value.read_f64()),
            winmd::ElementType::String => ConstantValue::String(value.read_utf16()),
            value_type => panic!("Unsupported constant: {} ({:?})", name, value_type),
        };

        let name = format_ident(name);
        let value = value.gen();

        quote! {
            pub const #name: #value;
        }
    }
}

#[derive(Debug)]
pub enum ConstantValue {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
}

impl ConstantValue {
    fn gen(&self) -> TokenStream {
        match self {
            Self::U8(value) => quote! { u8 = #value },
            Self::I8(value) => quote! { i8 = #value },
            Self::U16(value) => quote! { u16 = #value },
            Self::I16(value) => quote! { i16 = #value },
            Self::U32(value) => quote! { u32 = #value },
            Self::I32(value) => quote! { i32 = #value },
            Self::U64(value) => quote! { u64 = #value },
            Self::I64(value) => quote! { i64 = #value },
            Self::F32(value) => quote! { f32 = #value },
            Self::F64(value) => quote! { f64 = #value },
            Self::String(value) => quote! { &'static str = #value },
        }
    }
}
