use super::*;

// TODO take Signature instead of MethodDef (wherever MethodDef is found)
pub fn gen(gen: &Gen, def: TypeDef, generics: &[Type], kind: InterfaceKind, method: MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames) -> TokenStream {
    let signature = gen.reader.method_def_signature(method, generics);
    let params = if kind == InterfaceKind::Composable { &signature.params[..signature.params.len() - 2] } else { &signature.params };

    let (name, name_compose) = if kind == InterfaceKind::Composable && signature.params.len() == 2 {
        ("new".into(), "compose".into())
    } else {
        let name = method_names.add(gen, method);
        let name_compose = name.join("_compose");
        (name, name_compose)
    };

    let interface_name = gen.type_def_name(def, generics);
    let vname = virtual_names.add(gen, method);
    let constraints = gen.param_constraints(params);
    let mut cfg = gen.reader.method_def_cfg(method);
    cfg.add_feature(gen.reader.type_def_namespace(def));
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);
    let args = gen_winrt_abi_args(gen, params);
    let params = gen_winrt_params(gen, params);

    let return_type_tokens = if let Some(return_type) = &signature.return_type {
        let tokens = gen.type_name(return_type);
        if gen.reader.type_is_winrt_array(return_type) {
            quote! { ::windows::core::Array<#tokens> }
        } else {
            quote! { #tokens }
        }
    } else {
        quote! { () }
    };

    let return_arg = if let Some(return_type) = &signature.return_type {
        if gen.reader.type_is_winrt_array(return_type) {
            let return_type = gen.type_name(return_type);
            quote! { ::windows::core::Array::<#return_type>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _ }
        } else {
            quote! { result__.as_mut_ptr() }
        }
    } else {
        quote! {}
    };

    let composable_args = match kind {
        InterfaceKind::Composable => quote! {
            ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _,
        },
        _ => quote! {},
    };

    let (vcall, vcall_none) = if let Some(return_type) = &signature.return_type {
        if gen.reader.type_is_winrt_array(return_type) {
            (
                quote! {
                    let mut result__ = ::core::mem::MaybeUninit::<#return_type_tokens>::zeroed();
                    (::windows::core::Interface::vtable(this).#vname)(::windows::core::Interface::as_raw(this), #args #composable_args #return_arg)
                        .and_then(|| result__.assume_init())
                },
                quote! {},
            )
        } else {
            let abi_type_name = gen.type_abi_name(return_type);

            (
                quote! {
                    let mut result__ = ::core::mem::MaybeUninit::<#abi_type_name>::zeroed();
                        (::windows::core::Interface::vtable(this).#vname)(::windows::core::Interface::as_raw(this), #args #composable_args #return_arg)
                            .from_abi::<#return_type_tokens>(result__ )
                },
                quote! {
                    let mut result__ = ::core::mem::MaybeUninit::<#abi_type_name>::zeroed();
                        (::windows::core::Interface::vtable(this).#vname)(::windows::core::Interface::as_raw(this), #args ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, #return_arg)
                            .from_abi::<#return_type_tokens>(result__ )
                },
            )
        }
    } else {
        (
            quote! {
                (::windows::core::Interface::vtable(this).#vname)(::windows::core::Interface::as_raw(this), #args #composable_args).ok()
            },
            quote! {},
        )
    };

    match kind {
        InterfaceKind::Default => quote! {
            #doc
            #features
            pub fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                let this = self;
                unsafe {
                    #vcall
                }
            }
        },
        InterfaceKind::None | InterfaceKind::Base | InterfaceKind::Overridable => {
            quote! {
                #doc
                #features
                pub fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    let this = &::windows::core::Interface::cast::<#interface_name>(self)?;
                    unsafe {
                        #vcall
                    }
                }
            }
        }
        InterfaceKind::Static => {
            quote! {
                #doc
                #features
                pub fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type_tokens> {
                    Self::#interface_name(|this| unsafe { #vcall })
                }
            }
        }
        InterfaceKind::Composable => {
            quote! {
                #doc
                #features
                pub fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type_tokens> {
                    Self::#interface_name(|this| unsafe { #vcall_none })
                }
                #doc
                #features
                pub fn #name_compose<#constraints T: ::windows::core::Compose>(#params  compose: T) -> ::windows::core::Result<#return_type_tokens> {
                    Self::#interface_name(|this| unsafe {
                        let (derived__, base__) = ::windows::core::Compose::compose(compose);
                        #vcall
                    })
                }
            }
        }
    }
}

fn gen_winrt_params(gen: &Gen, params: &[SignatureParam]) -> TokenStream {
    let mut result = quote! {};

    for (position, param) in params.iter().enumerate() {
        let name = gen.param_name(param.def);
        let kind = gen.type_name(&param.ty);
        let default_type = gen.type_default_name(&param.ty);

        if gen.reader.param_flags(param.def).input() {
            if gen.reader.type_is_winrt_array(&param.ty) {
                result.combine(&quote! { #name: &[#default_type], });
            } else if gen.reader.signature_param_is_convertible(param) {
                let kind: TokenStream = format!("Param{}", position).into();
                result.combine(&quote! { #name: #kind, });
            } else {
                result.combine(&quote! { #name: #kind, });
            }
        } else if gen.reader.type_is_winrt_array(&param.ty) {
            result.combine(&quote! { #name: &mut [#default_type], });
        } else if gen.reader.type_is_winrt_array_ref(&param.ty) {
            result.combine(&quote! { #name: &mut ::windows::core::Array<#kind>, });
        } else {
            result.combine(&quote! { #name: &mut #default_type, });
        }
    }

    result
}

fn gen_winrt_abi_args(gen: &Gen, params: &[SignatureParam]) -> TokenStream {
    let mut tokens = TokenStream::new();
    for param in params {
        let name = gen.param_name(param.def);

        let param = if gen.reader.param_flags(param.def).input() {
            if gen.reader.type_is_winrt_array(&param.ty) {
                quote! { #name.len() as u32, ::core::mem::transmute(#name.as_ptr()), }
            } else if gen.reader.signature_param_is_convertible(param) {
                if gen.reader.type_is_winrt_const_ref(&param.ty) {
                    quote! { &#name.into_param().abi(), }
                } else {
                    quote! { #name.into_param().abi(), }
                }
            } else if gen.reader.type_is_blittable(&param.ty) {
                quote! { #name, }
            } else {
                quote! { ::core::mem::transmute_copy(#name), }
            }
        } else if gen.reader.type_is_winrt_array(&param.ty) {
            quote! { #name.len() as u32, ::core::mem::transmute_copy(&#name), }
        } else if gen.reader.type_is_winrt_array_ref(&param.ty) {
            quote! { #name.set_abi_len(), #name as *mut _ as _, }
        } else if gen.reader.type_is_blittable(&param.ty) {
            quote! { #name, }
        } else {
            quote! { #name as *mut _ as _, }
        };
        tokens.combine(&param);
    }
    tokens
}

pub fn gen_upcall(gen: &Gen, sig: &Signature, inner: TokenStream) -> TokenStream {
    let invoke_args = sig.params.iter().map(|param| gen_winrt_invoke_arg(gen, param));

    match &sig.return_type {
        Some(return_type) if gen.reader.type_is_winrt_array(return_type) => {
            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::core::result::Result::Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        // use `core::ptr::write` since `result` could be uninitialized
                        ::core::ptr::write(result__, ok_data__);
                        ::core::ptr::write(result_size__, ok_data_len__);
                        ::windows::core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into()
                }
            }
        }
        Some(_) => {
            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::core::result::Result::Ok(ok__) => {
                        // use `core::ptr::write` since `result` could be uninitialized
                        ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                        ::core::mem::forget(ok__);
                        ::windows::core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into()
                }
            }
        }
        None => quote! {
            #inner(#(#invoke_args,)*).into()
        },
    }
}

fn gen_winrt_invoke_arg(gen: &Gen, param: &SignatureParam) -> TokenStream {
    let name = gen.param_name(param.def);
    let abi_size_name: TokenStream = format!("{}_array_size", gen.reader.param_name(param.def)).into();

    if gen.reader.param_flags(param.def).input() {
        if gen.reader.type_is_winrt_array(&param.ty) {
            quote! { ::core::slice::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name as _) }
        } else if gen.reader.type_is_primitive(&param.ty) {
            quote! { #name }
        } else if gen.reader.type_is_winrt_const_ref(&param.ty) {
            quote! { ::core::mem::transmute_copy(&#name) }
        } else {
            quote! { ::core::mem::transmute(&#name) }
        }
    } else if gen.reader.type_is_winrt_array(&param.ty) {
        quote! { ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&#name), #abi_size_name as _) }
    } else if gen.reader.type_is_winrt_array_ref(&param.ty) {
        quote! { ::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name).as_array() }
    } else {
        quote! { ::core::mem::transmute_copy(&#name) }
    }
}
