use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Callback(pub tables::TypeDef);

impl Callback {
    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let signature = self.0.invoke_method().signature(&[]);

        // Note that callbacks are C-style function pointers so the code gen will only use ABI types
        // to ensure the the ABI is faithfully preserved. Other types generally provide an abstraction
        // over the ABI but in this case that is not practical.

        let params = signature.params.iter().map(|p| {
            let name = p.param.gen_name();
            let tokens = p.gen_win32_abi_param(gen);
            quote! { #name: #tokens }
        });

        let return_type = if let Some(t) = &signature.return_type {
            let tokens = t.gen_win32_abi(gen);
            quote! { -> #tokens }
        } else {
            quote! {}
        };

        quote! {
            pub type #name = unsafe extern "system" fn(#(#params),*) #return_type;
        }
    }
}
