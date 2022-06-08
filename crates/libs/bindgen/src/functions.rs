use super::*;

pub fn gen(gen: &Gen, def: MethodDef) -> TokenStream {
    if gen.sys {
        gen_sys_function(gen, def)
    } else {
        gen_win_function(gen, def)
    }
}

fn gen_sys_function(gen: &Gen, def: MethodDef) -> TokenStream {
    let name = to_ident(gen.reader.method_def_name(def));
    let signature = gen.reader.method_def_signature(def, &[]);
    let cfg = gen.reader.signature_cfg(&signature);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);
    let mut return_type = gen.return_sig(&signature);

    if return_type.is_empty() {
        return_type = does_not_return(gen, def);
    }

    let params = signature.params.iter().map(|p| {
        let name = gen.param_name(p.def);
        let tokens = gen.type_default_name(&p.ty);
        quote! { #name: #tokens }
    });

    quote! {
        #doc
        #features
        pub fn #name(#(#params),*) #return_type;
    }
}

fn gen_win_function(gen: &Gen, def: MethodDef) -> TokenStream {
    let name = to_ident(gen.reader.method_def_name(def));
    let signature = gen.reader.method_def_signature(def, &[]);
    let constraints = gen.param_constraints(&signature.params);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen.param_name(p.def);
        let tokens = gen.type_abi_name(&p.ty);
        quote! { #name: #tokens }
    });

    let abi_return_type = gen.return_sig(&signature);

    let link_attr = match gen.reader.method_def_static_lib(def) {
        Some(link) => quote! { #[link(name = #link, kind = "static")] },
        None => {
            if gen.namespace.starts_with("Windows.") {
                quote! { #[link(name = "windows")] }
            } else {
                let impl_map = gen.reader.method_def_impl_map(def).expect("ImplMap not found");
                let scope = gen.reader.impl_map_scope(impl_map);
                let link = gen.reader.module_ref_name(scope).to_lowercase();
                quote! { #[link(name = #link)] }
            }
        }
    };

    let cfg = gen.reader.signature_cfg(&signature);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    let kind = gen.reader.signature_kind(&signature);
    match kind {
        SignatureKind::Query(_) => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(#params) -> ::windows::core::Result<T> {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        let mut result__ = ::core::option::Option::None;
                        #name(#args).and_some(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::QueryOptional(_) => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(#params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        #name(#args).ok()
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = gen.win32_args(leading_params, kind);
            let params = gen.win32_params(leading_params, kind);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let return_type_tokens = gen.type_name(&return_type);
            let abi_return_type_tokens = gen.type_abi_name(&return_type);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type_tokens> {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        let mut result__ = ::core::mem::MaybeUninit::<#abi_return_type_tokens>::zeroed();
                        #name(#args ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<#return_type_tokens>(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::ResultVoid => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<()> {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #abi_return_type;
                        }
                        #name(#args).ok()
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::ReturnStruct | SignatureKind::PreserveSig => {
            if handle_last_error(gen, def, &signature) {
                let args = gen.win32_args(&signature.params, kind);
                let params = gen.win32_params(&signature.params, kind);
                let return_type = gen.type_name(&signature.return_type.unwrap());

                quote! {
                    #doc
                    #features
                    #[inline]
                    pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type> {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) -> #return_type;
                            }
                            let result__ = #name(#args);
                            (!result__.is_invalid()).then(||result__).ok_or_else(::windows::core::Error::from_win32)
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            } else {
                let args = gen.win32_args(&signature.params, kind);
                let params = gen.win32_params(&signature.params, kind);

                quote! {
                    #doc
                    #features
                    #[inline]
                    pub unsafe fn #name<#constraints>(#params) #abi_return_type {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) #abi_return_type;
                            }
                            ::core::mem::transmute(#name(#args))
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
        }
        SignatureKind::ReturnVoid => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let does_not_return = does_not_return(gen, def);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints>(#params) #does_not_return {
                    #[cfg(windows)]
                    {
                        #link_attr
                        extern "system" {
                            fn #name(#(#abi_params),*) #does_not_return;
                        }
                        #name(#args)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
    }
}

fn does_not_return(gen: &Gen, def: MethodDef) -> TokenStream {
    if gen.reader.method_def_does_not_return(def) {
        quote! { -> ! }
    } else {
        quote! {}
    }
}

fn handle_last_error(gen: &Gen, def: MethodDef, signature: &Signature) -> bool {
    if let Some(map) = gen.reader.method_def_impl_map(def) {
        if gen.reader.impl_map_flags(map).last_error() {
            if let Some(Type::TypeDef((return_type, _))) = &signature.return_type {
                if gen.reader.type_def_is_handle(*return_type) {
                    if gen.reader.type_def_underlying_type(*return_type).is_pointer() {
                        return true;
                    }
                    if !gen.reader.type_def_invalid_values(*return_type).is_empty() {
                        return true;
                    }
                }
            }
        }
    }
    false
}
