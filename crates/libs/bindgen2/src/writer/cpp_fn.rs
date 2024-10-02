use super::*;

impl Writer {
    pub fn write_cpp_fn(&self, def: &CppFn) -> TokenStream {
        if self.sys {
            self.write_sys_fn(def)
        } else {
            self.write_fn(def)
        }
    }

    fn write_sys_fn(&self, _def: &CppFn) -> TokenStream {
        quote! {}
    }

    fn write_fn(&self, _def: &CppFn) -> TokenStream {
        quote! {}
    }
}
