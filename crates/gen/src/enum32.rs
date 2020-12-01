use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Enum32 {
    pub name: TypeName,
}

impl Enum32 {
    pub fn from_type_name(_reader: &winmd::TypeReader, name: TypeName) -> Self {
        Self { name }
    }

    pub fn gen(&self) -> TokenStream {
        quote! {}
    }
}
