use super::*;
use metadata::HasAttributes;

pub fn writer(writer: &Writer, namespace: &str, def: metadata::MethodDef) -> TokenStream {
    // TODO: remove inline functions from metadata
    if def.module_name() == "forceinline" {
        return quote! {};
    }

    // TODO: remove ordinal functions from metadata
    if let Some(impl_map) = def.impl_map() {
        if impl_map.import_name().starts_with('#') {
            return quote! {};
        }
    }

    if writer.sys {
        gen_sys_function(writer, namespace, def)
    } else {
        gen_win_function(writer, namespace, def)
    }
}

fn gen_sys_function(writer: &Writer, namespace: &str, def: metadata::MethodDef) -> TokenStream {
    let signature = metadata::method_def_signature(namespace, def, &[]);
    let cfg = cfg::signature_cfg(writer, def);
    let mut tokens = writer.cfg_features(&cfg);
    tokens.combine(&gen_link(writer, namespace, &signature));
    tokens
}

fn gen_win_function(writer: &Writer, namespace: &str, def: metadata::MethodDef) -> TokenStream {
    let name = to_ident(def.name());
    let signature = metadata::method_def_signature(namespace, def, &[]);
    let generics = writer.constraint_generics(&signature.params);
    let where_clause = writer.where_clause(&signature.params);
    let abi_return_type = writer.return_sig(&signature);
    let cfg = cfg::signature_cfg(writer, def);
    let features = writer.cfg_features(&cfg);
    let link = gen_link(writer, namespace, &signature);

    let kind = signature.kind();
    match kind {
        metadata::SignatureKind::Query(_) => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: windows_core::Interface));

            quote! {
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params) -> windows_core::Result<T> #where_clause {
                    #link
                    let mut result__ = std::ptr::null_mut();
                    #name(#args).and_then(||windows_core::Type::from_abi(result__))
                }
            }
        }
        metadata::SignatureKind::QueryOptional(_) => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: windows_core::Interface));

            quote! {
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params result__: *mut Option<T>) -> windows_core::Result<()> #where_clause {
                    #link
                    #name(#args).ok()
                }
            }
        }
        metadata::SignatureKind::ResultValue => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();

            let map = if metadata::type_is_blittable(&return_type) {
                quote! { map(||result__) }
            } else {
                quote! { and_then(||windows_core::Type::from_abi(result__)) }
            };

            let return_type = writer.type_name(&return_type);

            quote! {
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                    #link
                    let mut result__ = std::mem::zeroed();
                    #name(#args).#map
                }
            }
        }
        metadata::SignatureKind::ResultVoid => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);

            quote! {
                #features
                #[inline]
                pub unsafe fn #name<#generics>(#params) -> windows_core::Result<()> #where_clause {
                    #link
                    #name(#args).ok()
                }
            }
        }
        metadata::SignatureKind::ReturnValue => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let is_nullable = metadata::type_is_nullable(&return_type);

            if is_nullable {
                let return_type = writer.type_name(&return_type);

                quote! {
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                        #link
                        let mut result__ = std::mem::zeroed();
                        #name(#args);
                        windows_core::Type::from_abi(result__.assume_init())
                    }
                }
            } else {
                let map = if metadata::type_is_blittable(&return_type) {
                    quote! { result__ }
                } else {
                    quote! { std::mem::transmute(result__) }
                };

                let return_type = writer.type_name(&return_type);

                quote! {
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> #return_type #where_clause {
                        #link
                        let mut result__ = std::mem::zeroed();
                        #name(#args);
                        #map
                    }
                }
            }
        }
        metadata::SignatureKind::ReturnStruct | metadata::SignatureKind::PreserveSig => {
            if handle_last_error(def, &signature) {
                let args = writer.win32_args(&signature.params, kind);
                let params = writer.win32_params(&signature.params, kind);
                let return_type = writer.type_name(&signature.return_type);

                quote! {
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                        #link
                        let result__ = #name(#args);
                        (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
                    }
                }
            } else {
                let args = writer.win32_args(&signature.params, kind);
                let params = writer.win32_params(&signature.params, kind);

                quote! {
                    #features
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) #abi_return_type #where_clause {
                        #link
                        #name(#args)
                    }
                }
            }
        }
        metadata::SignatureKind::ReturnVoid => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let does_not_return = does_not_return(def);

            quote! {
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

fn gen_link(writer: &Writer, namespace: &str, signature: &metadata::Signature) -> TokenStream {
    let name = signature.def.name();
    let ident = to_ident(name);
    let library = signature.def.module_name();
    let abi = method_def_extern_abi(signature.def);

    let symbol = if let Some(impl_map) = signature.def.impl_map() { impl_map.import_name() } else { name };

    let link_name = if symbol != name {
        quote! { #[link_name = #symbol] }
    } else {
        quote! {}
    };

    let params = signature.params.iter().map(|p| {
        let name = writer.param_name(p.def);
        let tokens = if p.kind == metadata::SignatureParamKind::ValueType { writer.type_default_name(&p.ty) } else { writer.type_abi_name(&p.ty) };
        quote! { #name: #tokens }
    });

    let return_type = writer.return_sig(signature);

    let vararg = if writer.sys && signature.call_flags.contains(metadata::MethodCallAttributes::VARARG) {
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
    } else {
        let symbol = if symbol != name { format!(" \"{symbol}\"") } else { String::new() };

        let mut tokens = String::new();
        for param in params {
            tokens.push_str(&format!("{}, ", param.as_str()));
        }
        tokens.push_str(&vararg.0);
        let tokens = tokens.trim_end_matches(", ");
        format!(r#"windows_targets::link!("{library}" "{abi}"{symbol} fn {name}({tokens}){return_type});"#).into()
    }
}

fn does_not_return(def: metadata::MethodDef) -> TokenStream {
    if def.has_attribute("DoesNotReturnAttribute") {
        quote! { -> ! }
    } else {
        quote! {}
    }
}

fn handle_last_error(def: metadata::MethodDef, signature: &metadata::Signature) -> bool {
    if let Some(map) = def.impl_map() {
        if map.flags().contains(metadata::PInvokeAttributes::SupportsLastError) {
            if let metadata::Type::TypeDef(return_type, _) = &signature.return_type {
                if metadata::type_def_is_handle(*return_type) {
                    // https://github.com/microsoft/windows-rs/issues/2392#issuecomment-1477765781
                    if def.name() == "LocalFree" {
                        return false;
                    }
                    if return_type.underlying_type().is_pointer() {
                        return true;
                    }
                    if !metadata::type_def_invalid_values(*return_type).is_empty() {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn method_def_extern_abi(def: metadata::MethodDef) -> &'static str {
    let impl_map = def.impl_map().expect("ImplMap not found");
    let flags = impl_map.flags();

    if flags.contains(metadata::PInvokeAttributes::CallConvPlatformapi) {
        "system"
    } else if flags.contains(metadata::PInvokeAttributes::CallConvCdecl) {
        "cdecl"
    } else {
        unimplemented!()
    }
}
