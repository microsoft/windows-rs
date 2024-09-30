use super::*;

impl Writer {
    pub fn write_enum(&self, def: &Enum) -> TokenStream {
        let name = to_ident(def.def.name());
        quote! { pub enum #name { } }
    }
}
