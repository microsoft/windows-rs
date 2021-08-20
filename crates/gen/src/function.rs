use super::*;

pub fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = to_ident(def.name());
    let signature = def.signature(&[]);

    let constraints = gen_method_constraints(&signature.params);
    let params = gen_win32_params(&signature.params, gen);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let abi_return_type = if let Some(t) = &signature.return_type {
        // TODO: This should be gen_win32_abi?
        let tokens = gen_sig(t, gen);
        quote! { -> #tokens }
    } else {
        TokenStream::new()
    };

    let args = signature.params.iter().map(|p| gen_win32_abi_arg(p));
    let mut link = def.impl_map().expect("Function").scope().name();

    let raw_dylib = cfg!(feature = "raw_dylib");

    // TODO: remove this whole block once raw-dylib has stabilized as the workarounds
    // will no longer be necessary.
    if !raw_dylib && (link.contains("-ms-win-") || link == "D3DCOMPILER_47" || link == "SspiCli") {
        link = "onecoreuap";
    }

    let link_attr = match def.static_lib() {
        Some(link) => quote! { #[link(name = #link, kind = "static")] },
        None => {
            // TODO: switch to always using raw-dylib once it has stabilized
            if raw_dylib {
                quote! { #[link(name = #link, kind="raw-dylib")] }
            } else {
                quote! { #[link(name = #link)] }
            }
        }
    };

    if signature.has_query_interface() {
        let leading_params = &signature.params[..signature.params.len() - 2];
        let args = leading_params.iter().map(|p| gen_win32_abi_arg(p));
        let params = gen_win32_params(leading_params, gen);

        quote! {
            pub unsafe fn #name<#constraints T: ::windows::Interface>(#params) -> ::windows::Result<T> {
                #[cfg(windows)]
                {
                    #link_attr
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
        // TODO: this code is duplicated in com_interfaces.rs
        let leading_params = &signature.params[..signature.params.len() - 1];
        let args = leading_params.iter().map(|p| gen_win32_abi_arg(p));
        let params = gen_win32_params(leading_params, gen);

        let mut return_param = signature.params[signature.params.len() - 1].clone();

        let return_type_tokens = if return_param.signature.pointers > 1 {
            return_param.signature.pointers -= 1;
            gen_win32_param(&return_param, gen)
        } else {
            gen_name(&return_param.signature.kind, gen)
        };

        quote! {
            pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<#return_type_tokens> {
                #[cfg(windows)]
                {
                    #link_attr
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
        match &return_type.kind {
            ElementType::HRESULT => {
                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<()> {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) -> ::windows::HRESULT;
                            }
                            #name(#(#args),*).ok()
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
            ElementType::TypeDef(def) if def.type_name() == TypeName::NTSTATUS => {
                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<()> {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) #abi_return_type;
                            }
                            #name(#(#args),*).ok()
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
            _ => {
                let return_type = gen_sig(return_type, gen);

                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> #return_type {
                        #[cfg(windows)]
                        {
                            #link_attr
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
        }
    } else {
        quote! {
            pub unsafe fn #name<#constraints>(#params) {
                #[cfg(windows)]
                {
                    #link_attr
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
