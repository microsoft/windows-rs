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

        let value = constant.value();

        let name = format_ident(name);
        let value = gen(&value);

        quote! {
            pub const #name: #value;
        }
    }
}

fn gen(value: &winmd::ConstantValue) -> TokenStream {
    match value {
        winmd::ConstantValue::U8(value) => quote! { u8 = #value },
        winmd::ConstantValue::I8(value) => quote! { i8 = #value },
        winmd::ConstantValue::U16(value) => quote! { u16 = #value },
        winmd::ConstantValue::I16(value) => quote! { i16 = #value },
        winmd::ConstantValue::U32(value) => quote! { u32 = #value },
        winmd::ConstantValue::I32(value) => quote! { i32 = #value },
        winmd::ConstantValue::U64(value) => quote! { u64 = #value },
        winmd::ConstantValue::I64(value) => quote! { i64 = #value },
        winmd::ConstantValue::F32(value) => quote! { f32 = #value },
        winmd::ConstantValue::F64(value) => quote! { f64 = #value },
        winmd::ConstantValue::String(value) => quote! { &'static str = #value },
    }
}
