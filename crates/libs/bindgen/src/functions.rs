use super::*;

pub fn gen_sys_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
    if gen.sys {
        let mut tokens = quote! {};

        for entry in tree.types.values() {
            tokens.combine(&gen_function_if(entry, gen));
        }

        if !tokens.is_empty() {
            quote! {
                #[link(name = "windows")]
                extern "system" {
                    #tokens
                }
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    }
}

pub fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        let function = gen_sys_function(def, gen);

        quote! {
            #[link(name = "windows")]
            extern "system" {
                #function
            }
        }
    } else {
        gen_win_function(def, gen)
    }
}

fn gen_function_if(entry: &TypeEntry, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for def in &entry.def {
        if let ElementType::MethodDef(def) = def {
            tokens.combine(&gen_sys_function(def, gen));
        }
    }

    tokens
}

fn gen_sys_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let signature = def.signature(&[]);
    let return_type = gen_return_sig(&signature, gen);
    let cfg = gen.function_cfg(def).gen_with_doc(gen);

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_param_sig(p, gen);
        quote! { #name: #tokens }
    });

    quote! {
        #cfg
        pub fn #name(#(#params),*) #return_type;
    }
}

fn gen_win_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let signature = def.signature(&[]);
    let constraints = gen_param_constraints(&signature.params, gen);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_abi_param_sig(p, gen);
        quote! { #name: #tokens }
    });

    let abi_return_type = gen_return_sig(&signature, gen);

    let link_attr = match def.static_lib() {
        Some(link) => quote! { #[link(name = #link, kind = "static")] },
        None => {
            if gen.namespace.starts_with("Windows.") {
                quote! { #[link(name = "windows")] }
            } else {
                let link = def.impl_map().expect("Function").scope().name().to_lowercase();

                quote! { #[link(name = #link)] }
            }
        }
    };

    let cfg = gen.function_cfg(def).gen_with_doc(gen);

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(#params) -> ::windows::core::Result<T> {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        let mut result__ = ::core::option::Option::None;
                        #name(#(#args,)* &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(#params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        #name(#(#args,)* &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);
            let mut return_sig = signature.params[signature.params.len() - 1].signature.clone();
            return_sig.pointers -= 1;
            let return_type_tokens = gen_result_sig(&return_sig, gen);
            let abi_return_type_tokens = gen_abi_sig(&return_sig, gen);

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type_tokens> {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        let mut result__: #abi_return_type_tokens = ::core::mem::zeroed();
                        #name(#(#args,)* ::core::mem::transmute(&mut result__)).from_abi::<#return_type_tokens>(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::ResultVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<()> {
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
        SignatureKind::ReturnStruct | SignatureKind::PreserveSig => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #name<#constraints>(#params) #abi_return_type {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        ::core::mem::transmute(#name(#(#args),*))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::ReturnVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #cfg
                #[inline]
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
}
