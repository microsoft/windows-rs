use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Function(pub tables::MethodDef);

impl Function {
    // TODO: move to MethodDef?
    pub fn gen(def: &tables::MethodDef, gen: &Gen) -> TokenStream {
        let name = def.gen_name(gen);
        let signature = def.signature(&[]);

        let constraints = signature.gen_constraints(&signature.params);
        let params = signature.gen_win32_params(&signature.params, gen);

        let return_type = if let Some(t) = &signature.return_type {
            let tokens = t.gen_win32(gen);
            quote! { -> #tokens }
        } else {
            quote! {}
        };

        let abi_params = signature.params.iter().map(|p| {
            let name = p.param.gen_name();
            let tokens = p.gen_win32_abi_param(gen);
            quote! { #name: #tokens }
        });

        let abi_return_type = if let Some(t) = &signature.return_type {
            // TODO: This should be gen_win32_abi?
            let tokens = t.gen_win32(gen);
            quote! { -> #tokens }
        } else {
            TokenStream::new()
        };

        let args = signature.params.iter().map(|p| p.gen_win32_abi_arg());

        let mut link = def.impl_map().expect("Function").scope().name();

        // TODO: workaround for https://github.com/microsoft/windows-rs/issues/463
        if link.contains("-ms-win-") || link == "D3DCOMPILER_47" || link == "SspiCli" {
            link = "onecoreuap";
        }

        let body = if signature.has_query_interface() {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(|p| p.gen_win32_abi_arg());

            quote! {
                #[link(name = #link)]
                extern "system" {
                    fn #name(#(#abi_params),*) #abi_return_type;
                }
                let mut result__ = ::std::option::Option::None;
                #name(#(#args,)* &<T as ::windows::Interface>::IID, ::windows::Abi::set_abi(&mut result__)).and_some(result__)
            }
        } else {
            quote! {
                #[link(name = #link)]
                extern "system" {
                    fn #name(#(#abi_params),*) #abi_return_type;
                }
                #name(#(#args),*)
            }
        };

        // Don't link against windows DLLs when generating code for non-Windows targets.
        let body = quote! {
            #[cfg(windows)]
            {
                #body
            }
            #[cfg(not(windows))]
            unimplemented!("Unsupported target OS");
        };

        if signature.has_query_interface() {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let params = signature.gen_win32_params(leading_params, gen);
            quote! {
                pub unsafe fn #name<#constraints T: ::windows::Interface>(#params) -> ::windows::Result<T> {
                    #body
                }
            }
        } else {
            quote! {
                pub unsafe fn #name<#constraints>(#params) #return_type {
                    #body
                }
            }
        }

        // TODO: do the same here as for COM methods with retval signatures
    }
}
