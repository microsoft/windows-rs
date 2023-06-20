use super::*;

pub fn gen(gen: &Gen, def: MethodDef) -> TokenStream {
    if gen.sys {
        gen_sys_function(gen, def)
    } else {
        gen_win_function(gen, def)
    }
}

fn gen_sys_function(gen: &Gen, def: MethodDef) -> TokenStream {
    let signature = gen.reader.method_def_signature(def, &[]);
    let cfg = gen.reader.signature_cfg(&signature);
    let mut tokens = gen.cfg_features(&cfg);
    tokens.combine(&gen_link(gen, &signature, &cfg));
    tokens
}

fn gen_win_function(gen: &Gen, def: MethodDef) -> TokenStream {
    let name = to_ident(gen.reader.method_def_name(def));
    let signature = gen.reader.method_def_signature(def, &[]);
    let generics = gen.constraint_generics(&signature.params);
    let where_clause = gen.where_clause(&signature.params);
    let abi_return_type = gen.return_sig(&signature);
    let cfg = gen.reader.signature_cfg(&signature);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);
    let link = gen_link(gen, &signature, &cfg);

    let kind = gen.reader.signature_kind(&signature);
    match kind {
        SignatureKind::Query(_) => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause =
                expand_where_clause(where_clause, quote!(T: ::windows_core::ComInterface));

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params) -> ::windows_core::Result<T> #where_clause {
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
                expand_where_clause(where_clause, quote!(T: ::windows_core::ComInterface));

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()> #where_clause {
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
                pub unsafe fn #name<#generics>(#params) -> ::windows_core::Result<#return_type> #where_clause {
                    #link
                    let mut result__ = ::std::mem::zeroed();
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
                pub unsafe fn #name<#generics>(#params) -> ::windows_core::Result<()> #where_clause {
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
                    pub unsafe fn #name<#generics>(#params) -> ::windows_core::Result<#return_type> #where_clause {
                        #link
                        let mut result__ = ::std::mem::zeroed();
                        #name(#args);
                        ::windows_core::from_abi(result__.assume_init())
                    }
                }
            } else {
                quote! {
                    #doc
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> #return_type #where_clause {
                        #link
                        let mut result__ = ::std::mem::zeroed();
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
                let return_type = gen.type_name(&signature.return_type);

                quote! {
                    #doc
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> ::windows_core::Result<#return_type> #where_clause {
                        #link
                        let result__ = #name(#args);
                        ::windows_core::imp::then(!result__.is_invalid(), ||result__).ok_or_else(::windows_core::Error::from_win32)
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

fn gen_link(gen: &Gen, signature: &Signature, cfg: &Cfg) -> TokenStream {
    let name = gen.reader.method_def_name(signature.def);
    let ident = to_ident(name);
    let library = gen.reader.method_def_module_name(signature.def);
    let abi = gen.reader.method_def_extern_abi(signature.def);

    let symbol = if let Some(impl_map) = gen.reader.method_def_impl_map(signature.def) {
        gen.reader.impl_map_import_name(impl_map)
    } else {
        name
    };

    let link_name = if symbol != name {
        quote! { #[link_name = #symbol] }
    } else {
        quote! {}
    };

    let params = signature.params.iter().map(|p| {
        let name = gen.param_name(p.def);
        let tokens = if p.kind == SignatureParamKind::ValueType {
            gen.type_default_name(&p.ty)
        } else {
            gen.type_abi_name(&p.ty)
        };
        quote! { #name: #tokens }
    });

    let return_type = gen.return_sig(signature);

    let vararg = if gen.sys && signature.call_flags.contains(MethodCallAttributes::VARARG) {
        "...".into()
    } else {
        quote! {}
    };

    if gen.std || !gen.namespace.starts_with("Windows.") {
        let library = library.trim_end_matches(".dll");

        quote! {
            #[link(name = #library)]
            extern #abi {
                #link_name
                pub fn #ident(#(#params,)* #vararg) #return_type;
            }
        }
    } else if let Some(library) = gen.reader.method_def_static_lib(signature.def) {
        quote! {
            #[link(name = #library, kind = "static")]
            extern #abi {
                #link_name
                pub fn #ident(#(#params,)* #vararg) #return_type;
            }
        }
    } else {
        let symbol = if symbol != name {
            format!(" \"{symbol}\"")
        } else {
            String::new()
        };

        let doc = if gen.sys {
            gen.cfg_doc(cfg).0
        } else {
            String::new()
        };

        let mut tokens = String::new();
        for param in params {
            tokens.push_str(&format!("{}, ", param.as_str()));
        }
        tokens.push_str(&vararg.0);
        let tokens = tokens.trim_end_matches(", ");
        format!(
            r#"::windows_targets::link!("{library}" "{abi}"{symbol}{doc} fn {name}({tokens}){return_type});"#
        )
        .into()
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
            .contains(PInvokeAttributes::SupportsLastError)
        {
            if let Type::TypeDef(return_type, _) = &signature.return_type {
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
