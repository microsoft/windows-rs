use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct ComClass {
    pub name: TypeName,
}

impl ComClass {
    pub fn from_type_name(name: TypeName) -> Self {
        Self { name }
    }

    pub fn gen(&self) -> TokenStream {
        // TODO: generate constant for CLSID

        quote! {}
    }
}
