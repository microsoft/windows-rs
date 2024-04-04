use super::*;

pub fn writer(writer: &Writer, def: metadata::TypeDef, kind: metadata::InterfaceKind, method: metadata::MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames) -> TokenStream {
    let signature = metadata::method_def_signature(def.namespace(), method, &[]);

    let name = method_names.add(method);
    let vname = virtual_names.add(method);
    let generics = writer.constraint_generics(&signature.params);
    let where_clause = writer.where_clause(&signature.params);
    let mut cfg = cfg::signature_cfg(writer, method);
    cfg.add_feature(def.namespace());
    let features = writer.cfg_features(&cfg);

    if kind == metadata::InterfaceKind::None {
        return quote! {};
    }

    let kind = signature.kind();
    match kind {
        metadata::SignatureKind::Query(_) => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: windows_core::Interface));

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<T> #where_clause {
                    let mut result__ = std::ptr::null_mut();
                    (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args).and_then(||windows_core::Type::from_abi(result__))
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
                pub unsafe fn #name<#generics>(&self, #params result__: *mut Option<T>) -> windows_core::Result<()> #where_clause {
                    (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args).ok()
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
                pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<#return_type> #where_clause {
                    let mut result__ = std::mem::zeroed();
                    (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args).#map
                }
            }
        }
        metadata::SignatureKind::ResultVoid => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<()> #where_clause {
                    (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args).ok()
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
                    pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<#return_type> #where_clause {
                        let mut result__ = std::mem::zeroed();
                        (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args);
                        windows_core::Type::from_abi(result__)
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
                    pub unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                        let mut result__ = std::mem::zeroed();
                        (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args);
                        #map
                    }
                }
            }
        }
        metadata::SignatureKind::ReturnStruct => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let return_type = writer.type_name(&signature.return_type);

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                    let mut result__: #return_type = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), &mut result__, #args);
                    result__
                }
            }
        }
        metadata::SignatureKind::PreserveSig => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);
            let return_type = writer.return_sig(&signature);

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) #return_type #where_clause {
                    (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args)
                }
            }
        }
        metadata::SignatureKind::ReturnVoid => {
            let args = writer.win32_args(&signature.params, kind);
            let params = writer.win32_params(&signature.params, kind);

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) #where_clause {
                    (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args)
                }
            }
        }
    }
}

pub fn gen_upcall(writer: &Writer, sig: &metadata::Signature, inner: TokenStream) -> TokenStream {
    match sig.kind() {
        metadata::SignatureKind::ResultValue => {
            let invoke_args = sig.params[..sig.params.len() - 1].iter().map(|param| gen_win32_invoke_arg(writer, param));

            let result = writer.param_name(sig.params[sig.params.len() - 1].def);

            quote! {
                match #inner(this, #(#invoke_args,)*) {
                    Ok(ok__) => {
                        // use `core::ptr::write` since the result could be uninitialized
                        core::ptr::write(#result, core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into()
                }
            }
        }
        metadata::SignatureKind::Query(_) | metadata::SignatureKind::QueryOptional(_) | metadata::SignatureKind::ResultVoid => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(writer, param));

            quote! {
                #inner(this, #(#invoke_args,)*).into()
            }
        }
        metadata::SignatureKind::ReturnStruct => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(writer, param));

            quote! {
                *result__ = #inner(this, #(#invoke_args,)*)
            }
        }
        _ => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(writer, param));

            quote! {
                #inner(this, #(#invoke_args,)*)
            }
        }
    }
}

fn gen_win32_invoke_arg(writer: &Writer, param: &metadata::SignatureParam) -> TokenStream {
    let name = writer.param_name(param.def);

    if param.def.flags().contains(metadata::ParamAttributes::In) && metadata::type_is_nullable(&param.ty) {
        quote! { windows_core::from_raw_borrowed(&#name) }
    } else if (!param.ty.is_pointer() && metadata::type_is_nullable(&param.ty)) || (param.def.flags().contains(metadata::ParamAttributes::In) && !metadata::type_is_primitive(&param.ty)) {
        quote! { core::mem::transmute(&#name) }
    } else {
        quote! { core::mem::transmute_copy(&#name) }
    }
}
