use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Callback(pub tables::TypeDef);

impl Callback {
    pub fn dependencies(&self) -> Vec<ElementType> {
        self.method().dependencies(&[])
    }

    pub fn definition(&self) -> Vec<ElementType> {
        vec![ElementType::Callback(self.clone())]
    }

    fn method(&self) -> tables::MethodDef {
        self.0
            .methods()
            .find(|m| m.name() == "Invoke")
            .expect("Callback")
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let signature = self.method().signature(&[]);

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

        let query_interface_fn = if signature.has_query_interface() {
            let constraints = signature.gen_constraints(&signature.params);
            let leading_params = &signature.params[..signature.params.len() - 2];
            let params = signature.gen_win32_params(leading_params, gen);
            let args = leading_params.iter().map(|p| p.gen_win32_abi_arg());
            quote! {
                pub unsafe fn #name<#constraints T: ::windows::Interface>(func: &#name, #params) -> ::windows::Result<T> {
                    let mut result__ = ::std::option::Option::None;
                    (func)(#(#args,)* &<T as ::windows::Interface>::IID, ::windows::Abi::set_abi(&mut result__)).and_some(result__)
                }
            }
        } else {
            quote!()
        };

        quote! {
            pub type #name = unsafe extern "system" fn(#(#params),*) #return_type;
            #query_interface_fn
        }
    }
}
