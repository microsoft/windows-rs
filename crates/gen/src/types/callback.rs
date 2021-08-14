use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Callback(pub TypeDef);

impl Callback {
    // TODO: make free gen_callback function
    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = gen_type_name(&self.0, gen);
        let signature = self.0.invoke_method().signature(&[]);

        // Note that callbacks are C-style function pointers so the code gen will only use ABI types
        // to ensure the the ABI is faithfully preserved. Other types generally provide an abstraction
        // over the ABI but in this case that is not practical.

        let params = signature.params.iter().map(|p| {
            let name = p.param.gen_name();
            let tokens = gen_win32_abi_param(p, gen);
            quote! { #name: #tokens }
        });

        let return_type = if let Some(t) = &signature.return_type {
            let tokens = gen_win32_abi_sig(t, gen);
            quote! { -> #tokens }
        } else {
            quote! {}
        };

        quote! {
            pub type #name = unsafe extern "system" fn(#(#params),*) #return_type;
        }
    }
}
