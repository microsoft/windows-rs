use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericParam(pub Row);

impl GenericParam {
    pub fn name(&self) -> &'static str {
        self.0.str(3)
    }

    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.name());
        quote! { #name }
    }
}

impl std::fmt::Debug for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
