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
        let name = format_ident(self.field.name());

        quote! {
            pub const  #name: i32 = 0;
        }
    }
}
