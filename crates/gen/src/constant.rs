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
        let value = value.gen();

        quote! {
            pub const #name: #value;
        }
    }
}
