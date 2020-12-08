use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Delegate32 {
    pub name: TypeName,
}

impl Delegate32 {
    pub fn from_type_name(name: TypeName) -> Self {
        Self { name }
    }

    pub fn gen(&self) -> TokenStream {
        let definition = self.name.gen_definition();

        quote! {
            #[allow(non_camel_case_types)]
            pub type #definition = extern "system" fn();
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        Vec::new()
    }
}
