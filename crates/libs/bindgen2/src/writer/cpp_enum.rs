use super::*;

impl Writer {
    pub fn write_cpp_enum(&self, item: &CppEnum) -> TokenStream {
        let name = to_ident(item.def.name());
        quote! { pub enum #name { } }
    }
}
