use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Function(pub MethodDef);

impl Function {
    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.0.name());
        quote! { #name }
    }

    pub fn gen(&self, _: Gen) -> TokenStream {
        quote! {}
    }
}
