use super::*;

impl CppDelegate {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());
        let method = self.method();
        let signature = method.signature(&[]);

        let params = signature.params.iter().map(|(ty, param)| {
            let name = to_ident(&param.name().to_lowercase());
            let ty = ty.write_default(writer);
            quote! { #name: #ty }
        });

        let return_sig = writer.write_return_sig(method, &signature);

        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies, &writer.config);
        }

        let cfg = writer.write_cfg(self.def, self.def.namespace(), dependencies, false);

        // TODO: are all callback "system" ABI?

        quote! {
            #cfg
            pub type #name = Option<unsafe extern "system" fn(#(#params),*) #return_sig>;
        }
    }
}
