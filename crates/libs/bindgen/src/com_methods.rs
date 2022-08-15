use super::*;

pub fn gen(gen: &Gen, def: TypeDef, kind: InterfaceKind, method: MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames, base_count: usize) -> TokenStream {
    let signature = gen.reader.method_def_signature(method, &[]);
    let name = method_names.add(gen, method);
    let vname = virtual_names.add(gen, method);
    let generics = gen.constraint_generics(&signature.params);
    let where_clause = gen.where_clause(&signature.params);
    let mut cfg = gen.reader.signature_cfg(&signature);
    cfg.add_feature(gen.reader.type_def_namespace(def));
    let features = gen.cfg_features(&cfg);

    if kind == InterfaceKind::None {
        return quote! {};
    }

    let mut bases = quote! {};

    for _ in 0..base_count {
        bases.combine(&quote! { .base__ });
    }

    let kind = gen.reader.signature_kind(&signature);
    match kind {
        SignatureKind::Query(_) => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: ::windows::core::Interface));

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> ::windows::core::Result<T> #where_clause {
                    let mut result__ = ::core::option::Option::None;
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::windows::core::Interface::as_raw(self), #args).and_some(result__)
                }
            }
        }
        SignatureKind::QueryOptional(_) => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: ::windows::core::Interface));

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> #where_clause {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::windows::core::Interface::as_raw(self), #args).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = gen.win32_args(leading_params, kind);
            let params = gen.win32_params(leading_params, kind);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let return_type_tokens = gen.type_name(&return_type);

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> ::windows::core::Result<#return_type_tokens> #where_clause {
                    let mut result__ = ::core::mem::MaybeUninit::zeroed();
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::windows::core::Interface::as_raw(self), #args ::core::mem::transmute(result__.as_mut_ptr()))
                    .from_abi::<#return_type_tokens>(result__ )
                }
            }
        }
        SignatureKind::ResultVoid => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> ::windows::core::Result<()> #where_clause {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::windows::core::Interface::as_raw(self), #args).ok()
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let return_type = gen.type_name(&signature.return_type.unwrap());

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                    let mut result__: #return_type = :: core::mem::zeroed();
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::windows::core::Interface::as_raw(self), &mut result__, #args);
                    result__
                }
            }
        }
        SignatureKind::PreserveSig => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);
            let return_type = gen.return_sig(&signature);

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) #return_type #where_clause {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::windows::core::Interface::as_raw(self), #args)
                }
            }
        }
        SignatureKind::ReturnVoid => {
            let args = gen.win32_args(&signature.params, kind);
            let params = gen.win32_params(&signature.params, kind);

            quote! {
                #features
                pub unsafe fn #name<#generics>(&self, #params) #where_clause {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::windows::core::Interface::as_raw(self), #args)
                }
            }
        }
    }
}

pub fn gen_upcall(gen: &Gen, sig: &Signature, inner: TokenStream) -> TokenStream {
    match gen.reader.signature_kind(sig) {
        SignatureKind::ResultValue => {
            let invoke_args = sig.params[..sig.params.len() - 1].iter().map(|param| gen_win32_invoke_arg(gen, param));

            let result = gen.param_name(sig.params[sig.params.len() - 1].def);

            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::core::result::Result::Ok(ok__) => {
                        // use `core::ptr::write` since the result could be uninitialized
                        ::core::ptr::write(#result, ::core::mem::transmute(ok__));
                        ::windows::core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into()
                }
            }
        }
        SignatureKind::Query(_) | SignatureKind::QueryOptional(_) | SignatureKind::ResultVoid => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(gen, param));

            quote! {
                #inner(#(#invoke_args,)*).into()
            }
        }
        SignatureKind::ReturnStruct => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(gen, param));

            quote! {
                *result__ = #inner(#(#invoke_args,)*)
            }
        }
        _ => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(gen, param));

            quote! {
                #inner(#(#invoke_args,)*)
            }
        }
    }
}

fn gen_win32_invoke_arg(gen: &Gen, param: &SignatureParam) -> TokenStream {
    let name = gen.param_name(param.def);

    if (!param.ty.is_pointer() && gen.reader.type_is_nullable(&param.ty)) || (gen.reader.param_flags(param.def).input() && !gen.reader.type_is_primitive(&param.ty)) {
        quote! { ::core::mem::transmute(&#name) }
    } else {
        quote! { ::core::mem::transmute_copy(&#name) }
    }
}
