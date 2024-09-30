use super::*;

impl Writer {
    pub fn write_cpp_enum(&self, def: &CppEnum) -> TokenStream {
        let name = to_ident(def.def.name());
        quote! { pub enum #name { } }
    }
}
