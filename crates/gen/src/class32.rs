use crate::*;
use squote::{quote, TokenStream};

// Win32 classes describe global constants and functions and is vastly different to WinRT classes.
#[derive(Debug)]
pub struct Class32 {
    pub name: TypeName,
}

impl Class32 {
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
