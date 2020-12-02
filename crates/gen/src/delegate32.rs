use crate::*;
use squote::{quote, TokenStream};

// Win32 and WinRT delegates are vastly different so perhaps leave these distinct
#[derive(Debug)]
pub struct Delegate32 {
    pub name: TypeName,
}

impl Delegate32 {
    pub fn from_type_name(_reader: &winmd::TypeReader, name: TypeName) -> Self {
        Self { name }
    }

    pub fn gen(&self) -> TokenStream {
        quote! {}
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        Vec::new()
    }
}
