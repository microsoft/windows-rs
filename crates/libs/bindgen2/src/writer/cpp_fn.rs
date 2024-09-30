use super::*;

impl Writer {
    pub fn write_cpp_fn(&self, def: &CppFn) -> TokenStream {
        let name = to_ident(def.method.name());
        quote! { pub fn #name(); }
    }
}
