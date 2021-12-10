use super::*;

pub fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = to_ident(def.name());
    let signature = def.signature(&[]);
    let constraints = gen_method_constraints(&signature.params, gen);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let abi_return_type = gen_win32_return_sig(&signature, gen);

    let link_attr = match def.static_lib() {
        Some(link) => quote! { #[link(name = #link, kind = "static")] },
        None => {
            if gen.relative.starts_with("Windows.") {
                quote! { #[link(name = "windows")] }
            } else {
                let link = def.impl_map().expect("Function").scope().name().to_lowercase();

                quote! { #[link(name = #link)] }
            }
        }
    };

    let cfg = gen.gen_function_cfg(def.attributes(), &signature);

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
            let return_type_tokens = gen_win32_result_type(&signature, gen);

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
                        let mut result__: <#return_type_tokens as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
                        #name(#(#args,)* &mut result__).from_abi::<#return_type_tokens>(result__)
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
        SignatureKind::ReturnStruct | SignatureKind::PreserveSig | SignatureKind::ReturnVoid => {
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
    }
}
