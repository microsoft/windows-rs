use super::*;

pub fn gen_winrt_method(def: &TypeDef, kind: InterfaceKind, method: &MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames, gen: &Gen) -> TokenStream {
    let signature = method.signature(&def.generics);
    let params = if kind == InterfaceKind::Composable { &signature.params[..signature.params.len() - 2] } else { &signature.params };

    let (name, name_compose) = if kind == InterfaceKind::Composable && signature.params.len() == 2 {
        ("new".into(), "compose".into())
    } else {
        let name = method_names.add(method);
        let name_compose = name.join("_compose");
        (name, name_compose)
    };

    let vname = virtual_names.add(method);

    let constraints = gen_param_constraints(params, gen);
    let mut cfg = method.cfg();
    cfg.add_feature(def.namespace());
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);
    let args = params.iter().map(gen_winrt_abi_arg);
    let params = gen_winrt_params(params, gen);
    let interface_name = gen_type_name(def, gen);

    let return_type_tokens = if let Some(return_type) = &signature.return_type {
        let tokens = gen_element_name(return_type, gen);
        if return_type.is_winrt_array() {
            quote! { ::windows::core::Array<#tokens> }
        } else {
            quote! { #tokens }
        }
    } else {
        quote! { () }
    };

    let return_arg = if let Some(return_type) = &signature.return_type {
        if return_type.is_winrt_array() {
            let return_type = gen_element_name(return_type, gen);
            quote! { ::windows::core::Array::<#return_type>::set_abi_len(&mut result__), &mut result__ as *mut _ as _ }
        } else {
            quote! { &mut result__ }
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
        if return_type.is_winrt_array() {
            (
                quote! {
                    let mut result__: #return_type_tokens = ::core::mem::zeroed();
                    (::windows::core::Interface::vtable(this).#vname)(::core::mem::transmute_copy(this), #(#args,)* #composable_args #return_arg)
                        .and_then(|| result__ )
                },
                quote! {},
            )
        } else {
            let abi_type_name = gen_abi_element_name(return_type, gen);
            let args = quote! { #(#args,)* };

            (
                quote! {
                    let mut result__: #abi_type_name = ::core::mem::zeroed();
                        (::windows::core::Interface::vtable(this).#vname)(::core::mem::transmute_copy(this), #args #composable_args #return_arg)
                            .from_abi::<#return_type_tokens>(result__ )
                },
                quote! {
                    let mut result__: #abi_type_name = ::core::mem::zeroed();
                        (::windows::core::Interface::vtable(this).#vname)(::core::mem::transmute_copy(this), #args ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, #return_arg)
                            .from_abi::<#return_type_tokens>(result__ )
                },
            )
        }
    } else {
        (
            quote! {
                (::windows::core::Interface::vtable(this).#vname)(::core::mem::transmute_copy(this), #(#args,)* #composable_args).ok()
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
        InterfaceKind::NonDefault | InterfaceKind::Base => {
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

pub fn gen_com_method(def: &TypeDef, method: &MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames, base_count: usize, gen: &Gen) -> TokenStream {
    let signature = method.signature(&def.generics);
    let name = method_names.add(method);
    let vname = virtual_names.add(method);
    let constraints = gen_param_constraints(&signature.params, gen);
    let mut cfg = method.cfg();
    cfg.add_feature(def.namespace());
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    let mut bases = quote! {};

    for _ in 0..base_count {
        bases.combine(&quote! { .base });
    }

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = gen_win32_args(leading_params);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params) -> ::windows::core::Result<T> {
                    let mut result__ = ::core::option::Option::None;
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
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
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
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
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    let mut result__: #abi_return_type_tokens = ::core::mem::zeroed();
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args ::core::mem::transmute(&mut result__))
                    .from_abi::<#return_type_tokens>(result__ )
                }
            }
        }
        SignatureKind::ResultVoid => {
            let args = gen_win32_args(&signature.params);
            let params = gen_win32_params(&signature.params, gen);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args).ok()
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let args = gen_win32_args(&signature.params);
            let params = gen_win32_params(&signature.params, gen);
            // TODO: why is this using gen_abi_element_name?
            let return_type = gen_abi_element_name(&signature.return_type.unwrap(), gen);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) -> #return_type {
                    let mut result__: #return_type = :: core::mem::zeroed();
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), &mut result__, #args);
                    result__
                }
            }
        }
        SignatureKind::PreserveSig => {
            let args = gen_win32_args(&signature.params);
            let params = gen_win32_params(&signature.params, gen);
            // TODO: why gen_return_sig exists? Don't we always know it will be not ReturnVoid?
            let return_type = gen_return_sig(&signature, gen);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) #return_type {
                    ::core::mem::transmute((::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args))
                }
            }
        }
        SignatureKind::ReturnVoid => {
            let args = gen_win32_args(&signature.params);
            let params = gen_win32_params(&signature.params, gen);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args)
                }
            }
        }
    }
}

pub fn gen_win32_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut tokens = quote! {};

    for (position, param) in params.iter().enumerate() {
        let name = gen_param_name(&param.def);

        if let Some(ArrayInfo::Fixed(fixed)) = param.array_info {
            if fixed > 0 && param.def.free_with().is_none() {
                let ty = param.ty.deref();
                let ty = gen_default_type(&ty, gen);
                let len = Literal::u32_unsuffixed(fixed as _);

                let ty = if param.def.flags().output() {
                    quote! { &mut [#ty; #len] }
                } else {
                    quote! { &[#ty; #len] }
                };

                tokens.combine(&quote! { #name: #ty, });
                continue;
            }
        }

        if let Some(ArrayInfo::RelativeLen(_)) = param.array_info {
            let ty = param.ty.deref();
            let ty = gen_default_type(&ty, gen);
            let ty = if param.def.flags().output() {
                quote! { &mut [#ty] }
            } else {
                quote! { &[#ty] }
            };

            tokens.combine(&quote! { #name: #ty, });
            continue;
        }

        if let Some(ArrayInfo::RelativePtr(_)) = param.array_info {
            continue;
        }

        if param.is_convertible() {
            let kind: TokenStream = format!("Param{}", position).into();
            tokens.combine(&quote! { #name: #kind, });
            continue;
        }

        let kind = gen_default_type(&param.ty, gen);
        tokens.combine(&quote! { #name: #kind, });
    }

    tokens
}

pub fn gen_win32_args(params: &[MethodParam]) -> TokenStream {
    let mut tokens = quote! {};

    for param in params {
        let name = gen_param_name(&param.def);

        if let Some(ArrayInfo::Fixed(fixed)) = param.array_info {
            if fixed > 0 && param.def.free_with().is_none() {
                let signature = if param.def.flags().output() {
                    quote! { ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(#name)), }
                } else {
                    quote! { ::core::mem::transmute(::windows::core::as_ptr_or_null(#name)), }
                };

                tokens.combine(&signature);
                continue;
            }
        }

        if let Some(ArrayInfo::RelativeLen(_)) = param.array_info {
            let signature = if param.def.flags().output() {
                quote! { ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(#name)), }
            } else {
                quote! { ::core::mem::transmute(::windows::core::as_ptr_or_null(#name)), }
            };

            tokens.combine(&signature);
            continue;
        }

        if let Some(ArrayInfo::RelativePtr(relative)) = param.array_info {
            let name = gen_param_name(&params[relative].def);
            tokens.combine(&quote! { #name.len() as _, });
            continue;
        }

        if param.is_convertible() {
            tokens.combine(&quote! { #name.into_param().abi(), });
            continue;
        }

        tokens.combine(&quote! { ::core::mem::transmute(#name), });
    }

    tokens
}

pub fn gen_winrt_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut result = quote! {};

    for (position, param) in params.iter().enumerate() {
        let name = gen_param_name(&param.def);
        let kind = gen_element_name(&param.ty, gen);
        let default_type = gen_default_type(&param.ty, gen);

        if param.def.flags().input() {
            if param.ty.is_winrt_array() {
                result.combine(&quote! { #name: &[#default_type], });
            } else if param.is_convertible() {
                let kind: TokenStream = format!("Param{}", position).into();
                result.combine(&quote! { #name: #kind, });
            } else {
                result.combine(&quote! { #name: #kind, });
            }
        } else if param.ty.is_winrt_array() {
            result.combine(&quote! { #name: &mut [#default_type], });
        } else if param.ty.is_winrt_array_ref() {
            result.combine(&quote! { #name: &mut ::windows::core::Array<#kind>, });
        } else {
            result.combine(&quote! { #name: &mut #default_type, });
        }
    }

    result
}

pub fn gen_winrt_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.def);

    if param.def.flags().input() {
        if param.ty.is_winrt_array() {
            quote! { #name.len() as u32, ::core::mem::transmute(#name.as_ptr()) }
        } else if param.is_convertible() {
            if param.ty.is_winrt_const_ref() {
                quote! { &#name.into_param().abi() }
            } else {
                quote! { #name.into_param().abi() }
            }
        } else if param.ty.is_blittable() {
            quote! { #name }
        } else {
            quote! { ::core::mem::transmute_copy(#name) }
        }
    } else if param.ty.is_winrt_array() {
        quote! { #name.len() as u32, ::core::mem::transmute_copy(&#name) }
    } else if param.ty.is_winrt_array_ref() {
        quote! { #name.set_abi_len(), #name as *mut _ as _ }
    } else if param.ty.is_blittable() {
        quote! { #name }
    } else {
        quote! { #name as *mut _ as _ }
    }
}
