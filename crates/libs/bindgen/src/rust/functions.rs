use super::*;

pub fn writer(writer: &Writer, namespace: &str, def: MethodDef) -> TokenStream {
    // TODO: remove inline functions from metadata
    if writer.reader.method_def_module_name(def) == "forceinline" {
        return quote! {};
    }

    // TODO: remove ordinal functions from metadata
    if let Some(impl_map) = writer.reader.method_def_impl_map(def) {
        if writer.reader.impl_map_import_name(impl_map).starts_with('#') {
            return quote! {};
        }
    }

    if writer.sys {
        gen_sys_function(writer, namespace, def)
    } else {
        gen_win_function(writer, namespace, def)
    }
}

fn gen_sys_function(writer: &Writer, namespace: &str, def: MethodDef) -> TokenStream {
    let signature = method_def_signature(writer.reader, namespace, def, &[]);
    let cfg = signature_cfg(writer.reader, def);
    let mut tokens = writer.cfg_features(&cfg);
    tokens.combine(&gen_link(writer, namespace, &signature, &cfg));
    tokens
}

fn gen_win_function(writer: &Writer, namespace: &str, def: MethodDef) -> TokenStream {
    let name = to_ident(writer.reader.method_def_name(def));
    let signature = method_def_signature(writer.reader, namespace, def, &[]);
    let generics = writer.constraint_generics(&signature.params);
    let where_clause = writer.where_clause(&signature.params);
    let abi_return_type = writer.return_sig(&signature);
    let cfg = signature_cfg(writer.reader, def);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);
    let link = gen_link(writer, namespace, &signature, &cfg);

    let kind = signature_kind(writer.reader, &signature);
    match kind {
        SignatureKind::Query(_) => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: ::windows_core::ComInterface));

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
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: ::windows_core::ComInterface));

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
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let return_type = writer.type_name(&return_type);

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
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);

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
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let is_nullable = type_is_nullable(writer.reader, &return_type);
            let return_type = writer.type_name(&return_type);

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
            if handle_last_error(writer, def, &signature) {
                let args = writer.win32_args(&signature.params, kind);
                let params = writer.win32_params(&signature.params, kind);
                let return_type = writer.type_name(&signature.return_type);

                quote! {
                    #doc
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> ::windows_core::Result<#return_type> #where_clause {
                        #link
                        let result__ = #name(#args);
                        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
                    }
                }
            } else {
                let args = writer.win32_args(&signature.params, kind);
                let params = writer.win32_params(&signature.params, kind);

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
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let does_not_return = does_not_return(writer, def);

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

fn gen_link(writer: &Writer, namespace: &str, signature: &Signature, cfg: &Cfg) -> TokenStream {
    let name = writer.reader.method_def_name(signature.def);
    let ident = to_ident(name);
    let library = writer.reader.method_def_module_name(signature.def);
    let abi = method_def_extern_abi(writer.reader, signature.def);

    let symbol = if let Some(impl_map) = writer.reader.method_def_impl_map(signature.def) { writer.reader.impl_map_import_name(impl_map) } else { name };

    let link_name = if symbol != name {
        quote! { #[link_name = #symbol] }
    } else {
        quote! {}
    };

    let params = signature.params.iter().map(|p| {
        let name = writer.param_name(p.def);
        let tokens = if p.kind == SignatureParamKind::ValueType { writer.type_default_name(&p.ty) } else { writer.type_abi_name(&p.ty) };
        quote! { #name: #tokens }
    });

    let return_type = writer.return_sig(signature);

    let vararg = if writer.sys && signature.call_flags.contains(MethodCallAttributes::VARARG) {
        "...".into()
    } else {
        quote! {}
    };

    if writer.std || !namespace.starts_with("Windows.") {
        let library = library.trim_end_matches(".dll");

        quote! {
            #[link(name = #library)]
            extern #abi {
                #link_name
                pub fn #ident(#(#params,)* #vararg) #return_type;
            }
        }
    } else if let Some(library) = method_def_static_lib(writer.reader, signature.def) {
        quote! {
            #[link(name = #library, kind = "static")]
            extern #abi {
                #link_name
                pub fn #ident(#(#params,)* #vararg) #return_type;
            }
        }
    } else {
        let symbol = if symbol != name { format!(" \"{symbol}\"") } else { String::new() };

        let doc = if writer.sys { writer.cfg_doc(cfg).0 } else { String::new() };

        let mut tokens = String::new();
        for param in params {
            tokens.push_str(&format!("{}, ", param.as_str()));
        }
        tokens.push_str(&vararg.0);
        let tokens = tokens.trim_end_matches(", ");
        format!(r#"::windows_targets::link!("{library}" "{abi}"{symbol}{doc} fn {name}({tokens}){return_type});"#).into()
    }
}

fn does_not_return(writer: &Writer, def: MethodDef) -> TokenStream {
    if writer.reader.has_attribute(def, "DoesNotReturnAttribute") {
        quote! { -> ! }
    } else {
        quote! {}
    }
}

fn handle_last_error(writer: &Writer, def: MethodDef, signature: &Signature) -> bool {
    if let Some(map) = writer.reader.method_def_impl_map(def) {
        if writer.reader.impl_map_flags(map).contains(PInvokeAttributes::SupportsLastError) {
            if let Type::TypeDef(return_type, _) = &signature.return_type {
                if type_def_is_handle(writer.reader, *return_type) {
                    if writer.reader.type_def_underlying_type(*return_type).is_pointer() {
                        return true;
                    }
                    if !type_def_invalid_values(writer.reader, *return_type).is_empty() {
                        return true;
                    }
                }
            }
        }
    }
    false
}
