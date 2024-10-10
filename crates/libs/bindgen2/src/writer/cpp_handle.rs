use super::*;

impl Writer {
    pub fn write_cpp_handle(&self, item: TypeDef) -> TokenStream {
        let name = to_ident(item.name());
        let ty = item.underlying_type();
        let ty = self.write_name(&ty);

        if self.sys {
            quote! {
                pub type #name = #ty;
            }
        } else {
            quote! {}
        }
    }
}