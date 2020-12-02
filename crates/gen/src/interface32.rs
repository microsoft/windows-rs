use crate::*;
use squote::{quote, TokenStream};

// Win32 (COM) interfaces are quite different to WinRT interfaces so probably best to treat seperately.
// TODO: Just remember to handle the where a COM interface derives from IInspectable (sadly happens).
#[derive(Debug)]
pub struct Interface32 {
    pub name: TypeName,
}

impl Interface32 {
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
