use crate::*;
use squote::{quote, TokenStream};

// TODO: have Struct handle both WinRT and Win32 structs - it's almost all the same code.
#[derive(Debug)]
pub struct Struct32 {
    pub name: TypeName,
}

impl Struct32 {
    pub fn from_type_name(name: TypeName) -> Self {
        Self { name }
    }

    pub fn gen(&self) -> TokenStream {
        quote! {}
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        Vec::new()
    }
}
