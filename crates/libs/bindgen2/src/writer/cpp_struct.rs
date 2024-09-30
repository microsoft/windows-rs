use super::*;

impl Writer {
    pub fn write_cpp_struct(&self, def: &CppStruct) -> TokenStream {
        let name = to_ident(def.def.name());
        quote! { pub struct #name { } }
    }
}
