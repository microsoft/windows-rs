use super::*;

impl Writer {
    pub fn write_cpp_delegate(&self, item: &CppDelegate) -> TokenStream {
        let name = to_ident(item.def.name());
        let method = item.method();
        let signature = method.signature(&[]);

        let params = signature.params.iter().map(|(ty, param)| {
            let name = to_ident(&param.name().to_lowercase());
            let ty = self.write_default_name(ty);
            quote! { #name: #ty }
        });

        let return_sig = self.write_return_sig(method, &signature);

        let mut dependencies = Dependencies::new();

        if self.package {
            item.dependencies(&mut dependencies, self.minimal);
        }

        let cfg = self.write_cfg(item.def, item.def.namespace(), dependencies, false);

        // TODO: are all callback "system" ABI?

        quote! {
            #cfg
            pub type #name = Option<unsafe extern "system" fn(#(#params),*) #return_sig>;
        }
    }
}
