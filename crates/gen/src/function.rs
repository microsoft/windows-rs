use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Function {
    pub def: winmd::MethodDef,
}

impl Function {
    pub fn from_method_def(def: &winmd::MethodDef) -> Self {
        Self { def: *def }
    }

    pub fn gen(&self) -> TokenStream {
        quote! {

        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        Vec::new()
    }
}