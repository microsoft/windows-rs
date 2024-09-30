use super::*;

impl Writer {
    pub fn write_struct(&self, def: &Struct) -> TokenStream {
        let name = to_ident(def.def.name());
        quote! { struct #name { } }
    }
}
