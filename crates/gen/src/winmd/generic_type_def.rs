use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericTypeDef {
    pub def: TypeDef,
    pub generics: Vec<ElementType>,
}

impl GenericTypeDef {
    pub fn gen_name(&self, gen: &Gen) -> TokenStream {
        quote! {}
    }
}
