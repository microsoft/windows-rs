use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Function {
    pub name: TypeName,
    pub method: winmd::MethodDef,
}

impl Function {
    pub fn new(name: TypeName, method: &winmd::MethodDef) -> Self {
        Self {
            name,
            method: *method,
        }
    }

    pub fn gen(&self) -> TokenStream {
        let name = format_ident(self.method.name());

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
