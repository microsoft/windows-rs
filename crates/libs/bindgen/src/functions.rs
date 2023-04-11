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
    let return_type = gen.return_sig(&signature);
    let abi = gen.reader.method_def_extern_abi(def);
    let impl_map = gen
        .reader
        .method_def_impl_map(def)
        .expect("ImplMap not found");
    let scope = gen.reader.impl_map_scope(impl_map);
    let link = gen.reader.module_ref_name(scope).to_lowercase();

    // TODO: skip inline functions for now.
    if link == "forceinline" {
        return TokenStream::new();
    }

    let params = signature.params.iter().map(|p| {
        let name = gen.param_name(p.def);
        let tokens = gen.type_default_name(&p.ty);
        quote! { #name: #tokens }
    });

    let mut tokens = features;

    if gen.std {
        let link = link.trim_end_matches(".dll");
        tokens.combine(&quote! {
            #[link(name = #link)]
            extern #abi {
                pub fn #name(#(#params),*) #return_type;
            }
        });
    } else {
        tokens.combine(&gen_link(
            &link,
            abi,
            doc.as_str(),
            name.as_str(),
            params,
            return_type.as_str(),
        ));
    }

    tokens
}

fn gen_link<P: IntoIterator<Item = TokenStream>>(
    link: &str,
    abi: &str,
    doc: &str,
    name: &str,
    params: P,
    return_type: &str,
) -> TokenStream {
    let mut tokens = String::new();
    for param in params {
        tokens.push_str(&format!("{}, ", param.as_str()));
    }
    let tokens = tokens.trim_end_matches(", ");
    format!(
        "::windows_targets::link!(\"{link}\" \"{abi}\"{doc} fn {name}({tokens}) {return_type});"
    )
    .into()
}

fn gen_win_function(gen: &Gen, def: MethodDef) -> TokenStream {
    let name = to_ident(gen.reader.method_def_name(def));
    let signature = gen.reader.method_def_signature(def, &[]);
    let generics = gen.constraint_generics(&signature.params);
    let where_clause = gen.where_clause(&signature.params);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen.param_name(p.def);
        match p.kind {
            SignatureParamKind::ValueType => {
                let abi = gen.type_default_name(&p.ty);
                quote! { #name: #abi }
            }
            _ => {
                let abi = gen.type_abi_name(&p.ty);
                quote! { #name: #abi }
            }
        }
    });

    let extern_abi = gen.reader.method_def_extern_abi(def);
    let abi_return_type = gen.return_sig(&signature);

    let link = if let Some(link) = gen.reader.method_def_static_lib(def) {
        quote! {
            #[link(name = #link, kind = "static")]
            extern #extern_abi {
                fn #name(#(#abi_params),*) #abi_return_type;
            }
        }
    } else {
        let impl_map = gen
            .reader
            .method_def_impl_map(def)
            .expect("ImplMap not found");
        let scope = gen.reader.impl_map_scope(impl_map);
        let link = gen.reader.module_ref_name(scope).to_lowercase();

        // TODO: skip inline functions for now.
        if link == "forceinline" {
            return TokenStream::new();
        }

        if gen.namespace.starts_with("Windows.") {
            gen_link(
                &link,
                extern_abi,
                "",
                name.as_str(),
                abi_params,
                abi_return_type.as_str(),
            )
        } else {
            let link = link.trim_end_matches(".dll");
            quote! {
                #[link(name = #link)]
                extern #extern_abi {
                    fn #name(#(#abi_params),*) #abi_return_type;
                }
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
            let generics = expand_generics(generics, quote!(T));
            let where_clause =
                expand_where_clause(where_clause, quote!(T: ::windows::core::ComInterface));

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params) -> ::windows::core::Result<T> #where_clause {
                    #link
                    let mut result__ = ::std::ptr::null_mut();
                    #name(#args).from_abi(result__)
                }
            }
        }
        SignatureKind::QueryOptional(_) => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause =
                expand_where_clause(where_clause, quote!(T: ::windows::core::ComInterface));

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> #where_clause {
                    #link
                    #name(#args).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let return_type = gen.type_name(&return_type);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params) -> ::windows::core::Result<#return_type> #where_clause {
                    #link
                    let mut result__ = ::windows::core::zeroed::<#return_type>();
                    #name(#args).from_abi(result__)
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
                pub unsafe fn #name<#generics>(#params) -> ::windows::core::Result<()> #where_clause {
                    #link
                    #name(#args).ok()
                }
            }
        }
        SignatureKind::ReturnValue => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let is_nullable = gen.reader.type_is_nullable(&return_type);
            let return_type = gen.type_name(&return_type);

            if is_nullable {
                quote! {
                    #doc
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> ::windows::core::Result<#return_type> #where_clause {
                        #link
                        let mut result__ = ::windows::core::zeroed::<#return_type>();
                        #name(#args);
                        ::windows::core::from_abi(result__.assume_init())
                    }
                }
            } else {
                quote! {
                    #doc
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> #return_type #where_clause {
                        #link
                        let mut result__ = ::windows::core::zeroed::<#return_type>();
                        #name(#args);
                        ::std::mem::transmute(result__)
                    }
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
                    pub unsafe fn #name<#generics>(#params) -> ::windows::core::Result<#return_type> #where_clause {
                        #link
                        let result__ = #name(#args);
                        ::windows::imp::then(!result__.is_invalid(), ||result__).ok_or_else(::windows::core::Error::from_win32)
                    }
                }
            } else {
                let args = gen.win32_args(&signature.params, kind);
                let params = gen.win32_params(&signature.params, kind);

                quote! {
                    #doc
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) #abi_return_type #where_clause {
                        #link
                        #name(#args)
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
                pub unsafe fn #name<#generics>(#params) #does_not_return #where_clause {
                    #link
                    #name(#args)
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
        if gen
            .reader
            .impl_map_flags(map)
            .contains(PInvokeAttributes::LAST_ERROR)
        {
            if let Some(Type::TypeDef((return_type, _))) = &signature.return_type {
                if gen.reader.type_def_is_handle(*return_type) {
                    if gen
                        .reader
                        .type_def_underlying_type(*return_type)
                        .is_pointer()
                    {
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
