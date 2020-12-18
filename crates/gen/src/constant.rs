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
        quote! {}
    }
}
