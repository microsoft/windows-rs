use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Function {
    pub name: TypeName,
    pub method: winmd::MethodDef,
}

impl Function {
    pub fn new(name:TypeName, method: &winmd::MethodDef) -> Self {
        Self { name, method: *method }
    }

    pub fn gen(&self) -> TokenStream {
        quote! {}
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        Vec::new()
    }
}
