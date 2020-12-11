use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Class32 {
    pub name: TypeName,
}

impl Class32 {
    pub fn from_type_name(name: TypeName) -> Self {
        Self { name }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();

        quote! {
            #[link(name = "onecoreuap")]
            extern "system" {
                pub fn #name();
            }
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        Vec::new()
    }
}
