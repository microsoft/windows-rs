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

fn gen_function_if(entry: &[Type], gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for def in entry {
        if let Type::MethodDef(def) = def {
            tokens.combine(&gen_sys_function(def, gen));
        }
    }

    tokens
}

fn gen_sys_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let signature = def.signature(&[]);
    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);
    let mut return_type = gen_return_sig(&signature, gen);

    if return_type.is_empty() {
        return_type = does_not_return(def);
    }

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.def);
        let tokens = gen_default_type(&p.ty, gen);
        quote! { #name: #tokens }
    });

    quote! {
        #doc
        #features
        pub fn #name(#(#params),*) #return_type;
    }
}

fn gen_win_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let signature = def.signature(&[]);
    let constraints = gen_param_constraints(&signature.params, gen);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.def);
        let tokens = gen_abi_element_name(&p.ty, gen);
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

    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = gen_win32_args(leading_params);
            let params = gen_win32_params(leading_params, gen);

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
                        #name(#args &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = gen_win32_args(leading_params);
            let params = gen_win32_params(leading_params, gen);

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
                        #name(#args &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = gen_win32_args(leading_params);
            let params = gen_win32_params(leading_params, gen);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let return_type_tokens = gen_element_name(&return_type, gen);
            let abi_return_type_tokens = gen_abi_element_name(&return_type, gen);

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
                        let mut result__: #abi_return_type_tokens = ::core::mem::zeroed();
                        #name(#args ::core::mem::transmute(&mut result__)).from_abi::<#return_type_tokens>(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        SignatureKind::ResultVoid => {
            let args = gen_win32_args(&signature.params);
            let params = gen_win32_params(&signature.params, gen);

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
            match HandleLastErorr::new(def, &signature) {
                HandleLastErorr::None => {
                    let args = gen_win32_args(&signature.params);
                    let params = gen_win32_params(&signature.params, gen);

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
                HandleLastErorr::Pointer => {
                    let args = gen_win32_args(&signature.params);
                    let params = gen_win32_params(&signature.params, gen);
                    let return_type = gen_element_name(&signature.return_type.unwrap(), gen);

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
                                (!result__.is_null()).then(||result__).ok_or_else(||::windows::core::Error::from_win32())
                            }
                            #[cfg(not(windows))]
                            unimplemented!("Unsupported target OS");
                        }
                    }
                }
                HandleLastErorr::Integer => {
                    let args = gen_win32_args(&signature.params);
                    let params = gen_win32_params(&signature.params, gen);
                    let return_type = gen_element_name(&signature.return_type.unwrap(), gen);

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
                                (!result__.is_invalid()).then(||result__).ok_or_else(||::windows::core::Error::from_win32())
                            }
                            #[cfg(not(windows))]
                            unimplemented!("Unsupported target OS");
                        }
                    }
                }
            }
        }
        SignatureKind::ReturnVoid => {
            let args = gen_win32_args(&signature.params);
            let params = gen_win32_params(&signature.params, gen);
            let does_not_return = does_not_return(def);

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

fn does_not_return(def: &MethodDef) -> TokenStream {
    if def.does_not_return() {
        quote! { -> ! }
    } else {
        quote! {}
    }
}

enum HandleLastErorr {
    None,
    Pointer,
    Integer,
}

impl HandleLastErorr {
    fn new(def: &MethodDef, signature: &Signature) -> Self {
        if let Some(map) = def.impl_map() {
            if map.flags().last_error() {
                if let Some(Type::TypeDef(return_type)) = &signature.return_type {
                    if return_type.is_handle() {
                        if return_type.underlying_type().is_pointer() {
                            return Self::Pointer;
                        }
                        if !return_type.invalid_values().is_empty() {
                            return Self::Integer;
                        }
                    }
                }
            }
        }

        Self::None
    }
}