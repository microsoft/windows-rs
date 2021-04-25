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

        let static_lib = def
            .attributes()
            .filter_map(|attribute| match attribute.name() {
                "StaticLibraryAttribute" => Some(attribute.args()[0].1.gen_value()),
                _ => None,
            })
            .next();
        let link = match static_lib {
            Some(libName) => quote! { #[link(name = #libName, kind = #static_lib)] },
            None => quote! { #[link(name = )]},
        };

        if signature.has_query_interface() {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(|p| p.gen_win32_abi_arg());
            let params = signature.gen_win32_params(leading_params, gen);

            quote! {
                pub unsafe fn #name<#constraints T: ::windows::Interface>(#params) -> ::windows::Result<T> {
                    #[cfg(windows)]
                    {
                        #link
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        let mut result__ = ::std::option::Option::None;
                        #name(#(#args,)* &<T as ::windows::Interface>::IID, ::windows::Abi::set_abi(&mut result__)).and_some(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        } else if signature.has_retval() {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = leading_params.iter().map(|p| p.gen_win32_abi_arg());
            let params = signature.gen_win32_params(leading_params, gen);

            let return_type_tokens = signature
                .params
                .last()
                .unwrap()
                .signature
                .kind
                .gen_name(gen);

            quote! {
                pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<#return_type_tokens> {
                    #[cfg(windows)]
                    {
                        #link
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        let mut result__: <#return_type_tokens as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        #name(#(#args,)* &mut result__).from_abi::<#return_type_tokens>(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        } else if let Some(return_type) = &signature.return_type {
            if return_type.kind == ElementType::HRESULT {
                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<()> {
                        #[cfg(windows)]
                        {
                            #link
                            extern "system" {
                                fn #name(#(#abi_params),*) -> ::windows::HRESULT;
                            }
                            #name(#(#args),*).ok()
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            } else {
                let return_type = return_type.gen_win32(gen);

                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> #return_type {
                        #[cfg(windows)]
                        {
                            #link
                            extern "system" {
                                fn #name(#(#abi_params),*) #abi_return_type;
                            }
                            #name(#(#args),*)
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
        } else {
            quote! {
                pub unsafe fn #name<#constraints>(#params) {
                    #[cfg(windows)]
                    {
                        #link
                        extern "system" {
                            fn #name(#(#abi_params),*);
                        }
                        #name(#(#args),*)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
    }
}
