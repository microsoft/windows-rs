use super::*;

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
    pub fn gen(&self) -> TokenStream {
        match self {
            ConstantValue::U8(value) => quote! { u8 = #value },
            ConstantValue::I8(value) => quote! { i8 = #value },
            ConstantValue::U16(value) => quote! { u16 = #value },
            ConstantValue::I16(value) => quote! { i16 = #value },
            ConstantValue::U32(value) => quote! { u32 = #value },
            ConstantValue::I32(value) => quote! { i32 = #value },
            ConstantValue::U64(value) => quote! { u64 = #value },
            ConstantValue::I64(value) => quote! { i64 = #value },
            ConstantValue::F32(value) => quote! { f32 = #value },
            ConstantValue::F64(value) => quote! { f64 = #value },
            ConstantValue::String(value) => quote! { &'static str = #value },
        }
    }

    pub fn gen_value(&self) -> TokenStream {
        match self {
            ConstantValue::U8(value) => quote! { #value },
            ConstantValue::I8(value) => quote! { #value },
            ConstantValue::U16(value) => quote! { #value },
            ConstantValue::I16(value) => quote! { #value },
            ConstantValue::U32(value) => quote! { #value },
            ConstantValue::I32(value) => quote! { #value },
            ConstantValue::U64(value) => quote! { #value },
            ConstantValue::I64(value) => quote! { #value },
            ConstantValue::F32(value) => quote! { #value },
            ConstantValue::F64(value) => quote! { #value },
            ConstantValue::String(value) => quote! { #value },
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::U32(value) => Self::U32(value + 1),
            Self::I32(value) => Self::I32(value + 1),
            _ => unexpected!(),
        }
    }
}
