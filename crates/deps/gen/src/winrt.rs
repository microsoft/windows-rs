use super::*;

pub fn gen_winrt_upcall(sig: &MethodSignature, inner: TokenStream, gen: &Gen) -> TokenStream {
    let invoke_args = sig.params.iter().map(|param| gen_winrt_invoke_arg(param, gen));

    match &sig.return_sig {
        Some(return_sig) if return_sig.is_array => {
            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::core::result::Result::Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        *result__ = ok_data__;
                        *result_size__ = ok_data_len__;
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
                        *result__ = ::core::mem::transmute_copy(&ok__);
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

pub fn gen_winrt_abi(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    let params = sig
        .params
        .iter()
        .map(|p| {
            let name = gen_param_name(&p.param);
            let abi = gen_abi_sig(&p.signature, gen);

            if p.signature.is_array {
                let abi_size_name = to_ident(&format!("{}_array_size", p.param.name()));

                if p.param.is_input() {
                    quote! { #abi_size_name: u32, #name: *const #abi }
                } else if p.signature.by_ref {
                    quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi }
                } else {
                    quote! { #abi_size_name: u32, #name: *mut #abi }
                }
            } else if p.param.is_input() {
                if p.signature.is_const {
                    quote! { #name: &#abi }
                } else {
                    quote! { #name: #abi }
                }
            } else {
                quote! { #name: *mut #abi }
            }
        })
        .chain(sig.return_sig.iter().map(|signature| {
            let abi = gen_abi_sig(signature, gen);

            if signature.is_array {
                quote! { result_size__: *mut u32, result__: *mut *mut #abi }
            } else {
                quote! { result__: *mut #abi }
            }
        }));

    quote! {
        (this: ::windows::core::RawPtr, #(#params),*) -> ::windows::core::HRESULT
    }
}

fn gen_winrt_invoke_arg(param: &MethodParam, gen: &Gen) -> TokenStream {
    let name = gen_param_name(&param.param);
    let kind = gen_name(&param.signature.kind, gen);

    if param.signature.is_array {
        let abi_size_name = to_ident(&format!("{}_array_size", param.param.name()));

        if param.param.is_input() {
            quote! { ::core::slice::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name as _) }
        } else if param.signature.by_ref {
            quote! { ::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name).as_array() }
        } else {
            quote! { ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&#name), #abi_size_name as _) }
        }
    } else if param.param.is_input() {
        if param.signature.kind.is_primitive() {
            quote! { #name }
        } else if param.signature.is_const {
            quote! { &*(#name as *const <#kind as ::windows::core::Abi>::Abi as *const <#kind as ::windows::core::DefaultType>::DefaultType) }
        } else {
            quote! { &*(&#name as *const <#kind as ::windows::core::Abi>::Abi as *const <#kind as ::windows::core::DefaultType>::DefaultType) }
        }
    } else {
        quote! { ::core::mem::transmute_copy(&#name) }
    }
}

pub fn gen_winrt_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    params
        .iter()
        .enumerate()
        .map(|(position, param)| {
            let name = gen_param_name(&param.param);
            let tokens = gen_name(&param.signature.kind, gen);

            if param.signature.is_array {
                if param.param.is_input() {
                    quote! { #name: &[<#tokens as ::windows::core::DefaultType>::DefaultType], }
                } else if param.signature.by_ref {
                    quote! { #name: &mut ::windows::core::Array<#tokens>, }
                } else {
                    quote! { #name: &mut [<#tokens as ::windows::core::DefaultType>::DefaultType], }
                }
            } else if param.param.is_input() {
                if param.is_convertible() {
                    let into = format_token!("Param{}", position);
                    quote! { #name: #into, }
                } else {
                    quote! { #name: #tokens, }
                }
            } else if param.signature.kind.is_nullable() {
                quote! { #name: &mut ::core::option::Option<#tokens>, }
            } else if let ElementType::GenericParam(_) = param.signature.kind {
                quote! { &mut <#tokens as ::windows::core::DefaultType>::DefaultType, }
            } else if param.signature.pointers > 0 {
                let tokens = gen_abi_sig(&param.signature, gen);
                quote! { #name: #tokens, }
            } else {
                quote! { #name: &mut #tokens, }
            }
        })
        .collect()
}

pub fn gen_winrt_method(sig: &MethodSignature, method: &MethodInfo, interface: &InterfaceInfo, gen: &Gen) -> TokenStream {
    let params = if interface.kind == InterfaceKind::Composable || interface.kind == InterfaceKind::Extend { &sig.params[..sig.params.len() - 2] } else { &sig.params };

    let name = if (interface.kind == InterfaceKind::Composable || interface.kind == InterfaceKind::Extend) && sig.params.len() == 2 {
        "new".into()
    } else if method.overload > 1 {
        format_token!("{}{}", &method.name, method.overload)
    } else {
        method.name.clone().into()
    };

    let vtable_offset = Literal::u32_unsuffixed(method.vtable_offset);
    let constraints = gen_method_constraints(params, gen);
    let args = params.iter().map(gen_winrt_abi_arg);
    let params = gen_winrt_params(params, gen);
    let interface_name = gen_type_name(&interface.def, gen);

    let return_type_tokens = if let Some(return_sig) = &sig.return_sig {
        let tokens = gen_name(&return_sig.kind, gen);

        if return_sig.is_array {
            quote! { ::windows::core::Array<#tokens> }
        } else {
            tokens
        }
    } else {
        quote! { () }
    };

    let return_arg = if let Some(return_sig) = &sig.return_sig {
        if return_sig.is_array {
            let return_sig = gen_name(&return_sig.kind, gen);
            quote! { ::windows::core::Array::<#return_sig>::set_abi_len(&mut result__), &mut result__ as *mut _ as _ }
        } else {
            quote! { &mut result__ }
        }
    } else {
        quote! {}
    };

    // The ABI obviously still has the two composable parameters. Here we just pass the default in and out
    // arguments to ensure the call succeeds in the non-aggregating case.
    let composable_args = match interface.kind {
        InterfaceKind::Composable => quote! {
            ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
        },
        InterfaceKind::Extend => quote! {
            ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _,
        },
        _ => quote! {},
    };

    let vcall = if let Some(return_sig) = &sig.return_sig {
        if return_sig.is_array {
            quote! {
                let mut result__: #return_type_tokens = ::core::mem::zeroed();
                (::windows::core::Interface::vtable(this).#vtable_offset)(::core::mem::transmute_copy(this), #(#args,)* #composable_args #return_arg)
                    .and_then(|| result__ )
            }
        } else {
            let abi_type_name = gen_abi_type_name(&return_sig.kind, gen);

            quote! {
                let mut result__: #abi_type_name = ::core::mem::zeroed();
                    (::windows::core::Interface::vtable(this).#vtable_offset)(::core::mem::transmute_copy(this), #(#args,)* #composable_args #return_arg)
                        .from_abi::<#return_type_tokens>(result__ )
            }
        }
    } else {
        quote! {
            (::windows::core::Interface::vtable(this).#vtable_offset)(::core::mem::transmute_copy(this), #(#args,)* #composable_args).ok()
        }
    };

    // TODO: need to consolidate this cfg generation so we

    let features = interface_method_features(&interface.def, sig, gen);
    let cfg = gen.gen_cfg(&features);
    let doc = gen.gen_cfg_doc(&features);

    let deprecated = if method.is_deprecated {
        quote! { #[cfg(feature = "deprecated")] }
    } else {
        quote! {}
    };

    match interface.kind {
        InterfaceKind::Default => quote! {
            #deprecated
            #cfg
            #doc
            pub fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                let this = self;
                unsafe {
                    #vcall
                }
            }
        },
        InterfaceKind::NonDefault | InterfaceKind::Overridable | InterfaceKind::Base => {
            quote! {
                #deprecated
                #cfg
                #doc
                pub fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    let this = &::windows::core::Interface::cast::<#interface_name>(self)?;
                    unsafe {
                        #vcall
                    }
                }
            }
        }
        InterfaceKind::Static | InterfaceKind::Composable => {
            quote! {
                #deprecated
                #cfg
                #doc
                pub fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type_tokens> {
                    Self::#interface_name(|this| unsafe { #vcall })
                }
            }
        }
        InterfaceKind::Extend => {
            let interface_name = to_ident(interface.def.name());
            quote! {
                // TODO: why no deprecated?
                #cfg
                #doc
                pub fn #name<#constraints>(self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    unsafe {
                        let (derived__, base__) = ::windows::core::Compose::compose(self);
                        #return_type_tokens::#interface_name(|this| unsafe { #vcall })
                    }
                }
            }
        }
    }
}

pub fn gen_winrt_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.param);

    if param.signature.is_array {
        if param.param.is_input() {
            quote! { #name.len() as u32, ::core::mem::transmute(#name.as_ptr()) }
        } else if param.signature.by_ref {
            quote! { #name.set_abi_len(), #name as *mut _ as _ }
        } else {
            quote! { #name.len() as u32, ::core::mem::transmute_copy(&#name) }
        }
    } else if param.param.is_input() {
        if param.is_convertible() {
            if param.signature.is_const {
                quote! { &#name.into_param().abi() }
            } else {
                quote! { #name.into_param().abi() }
            }
        } else if param.signature.kind.is_blittable() {
            quote! { #name }
        } else if param.signature.pointers == 0 {
            quote! { ::core::mem::transmute_copy(#name) }
        } else {
            quote! { ::core::mem::transmute(#name) }
        }
    } else if param.signature.kind.is_blittable() || (param.signature.pointers > 0 && !param.signature.kind.is_nullable()) {
        quote! { #name }
    } else {
        quote! { #name as *mut _ as _ }
    }
}

pub fn gen_winrt_produce_type(param: &MethodParam, gen: &Gen) -> TokenStream {
    let tokens = gen_name(&param.signature.kind, gen);

    if param.signature.is_array {
        if param.param.is_input() {
            quote! { &[<#tokens as ::windows::core::DefaultType>::DefaultType] }
        } else if param.signature.by_ref {
            quote! { &mut ::windows::core::Array<#tokens> }
        } else {
            quote! { &mut [<#tokens as ::windows::core::DefaultType>::DefaultType] }
        }
    } else if param.param.is_input() {
        if let ElementType::GenericParam(_) = param.signature.kind {
            quote! { &<#tokens as ::windows::core::DefaultType>::DefaultType }
        } else if param.signature.kind.is_primitive() {
            quote! { #tokens }
        } else if param.signature.kind.is_nullable() {
            quote! { &::core::option::Option<#tokens> }
        } else {
            quote! { &#tokens }
        }
    } else if param.signature.kind.is_nullable() {
        quote! { &mut ::core::option::Option<#tokens> }
    } else {
        quote! { &mut #tokens }
    }
}

pub fn gen_phantoms(def: &TypeDef) -> impl Iterator<Item = TokenStream> + '_ {
    def.generics.iter().map(move |g| {
        let g = gen_name(g, &Gen::absolute());
        quote! { ::core::marker::PhantomData::<#g> }
    })
}

pub fn gen_constraints(def: &TypeDef) -> TokenStream {
    def.generics
        .iter()
        .map(|g| {
            let g = gen_name(g, &Gen::absolute());
            quote! { #g: ::windows::core::RuntimeType + 'static, }
        })
        .collect()
}
