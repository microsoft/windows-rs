use super::*;

#[derive(Debug)]
pub enum ConstantValue {
    Bool(bool),
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
    TypeDef(TypeDef),
}

impl ConstantValue {
    pub fn unwrap_u32(&self) -> u32 {
        match self {
            Self::U32(value) => *value,
            _ => unexpected!(),
        }
    }

    pub fn unwrap_u16(&self) -> u16 {
        match self {
            Self::U16(value) => *value,
            _ => unexpected!(),
        }
    }

    pub fn unwrap_u8(&self) -> u8 {
        match self {
            Self::U8(value) => *value,
            _ => unexpected!(),
        }
    }

    pub fn unwrap_string(&self) -> &str {
        match self {
            Self::String(value) => value,
            _ => unexpected!(),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::U32(value) => Self::U32(value + 1),
            Self::I32(value) => Self::I32(value + 1),
            _ => unexpected!(),
        }
    }

    pub fn gen(&self) -> TokenStream {
        match self {
            ConstantValue::Bool(value) => quote! { bool = #value },
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
            _ => unexpected!(),
        }
    }

    pub fn gen_value(&self) -> TokenStream {
        match self {
            ConstantValue::Bool(value) => quote! { #value },
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
            _ => unexpected!(),
        }
    }
}
