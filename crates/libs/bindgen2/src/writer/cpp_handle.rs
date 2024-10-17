use super::*;

impl Writer {
    pub fn write_cpp_handle(&self, item: TypeDef) -> TokenStream {
        let name = to_ident(item.name());
        let ty = item.underlying_type().write(self);

        if self.config.sys {
            quote! {
                pub type #name = #ty;
            }
        } else {
            quote! {}
        }
    }
}
