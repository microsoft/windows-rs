use super::*;

pub fn gen_winrt_method(def: &TypeDef, kind: InterfaceKind, method: &MethodDef, vtable_offset: usize, method_names: &mut BTreeMap<String, u32>, gen: &Gen) -> TokenStream {
    let signature = method.signature(&def.generics);

    let params = if kind == InterfaceKind::Composable { &signature.params[..signature.params.len() - 2] } else { &signature.params };

    let name = if kind == InterfaceKind::Composable && signature.params.len() == 2 {
        "new".into()
    } else {
        let name = method.rust_name();
        let overload = method_names.entry(name.to_string()).or_insert(0);
        *overload += 1;
        if *overload > 1 {
            format!("{}{}", name, overload).into()
        } else {
            gen_ident(&name)
        }
    };

    let constraints = gen_param_constraints(params, gen);
    let cfg = gen.method_cfg(def, method).gen_with_doc(gen);
    let vtable_offset = Literal::usize_unsuffixed(vtable_offset);
    let args = params.iter().map(gen_winrt_abi_arg);
    let params = gen_winrt_params(params, gen);
    let interface_name = gen_type_name(def, gen);

    let return_type_tokens = if let Some(return_sig) = &signature.return_sig {
        let tokens = gen_result_sig(return_sig, gen);
        if return_sig.is_array {
            quote! { ::windows::core::Array<#tokens> }
        } else {
            quote! { #tokens }
        }
    } else {
        quote! { () }
    };

    let return_arg = if let Some(return_sig) = &signature.return_sig {
        if return_sig.is_array {
            let return_sig = gen_element_name(&return_sig.kind, gen);
            quote! { ::windows::core::Array::<#return_sig>::set_abi_len(&mut result__), &mut result__ as *mut _ as _ }
        } else {
            quote! { &mut result__ }
        }
    } else {
        quote! {}
    };

    // The ABI obviously still has the two composable parameters. Here we just pass the default in and out
    // arguments to ensure the call succeeds in the non-aggregating case.
    let composable_args = match kind {
        InterfaceKind::Composable => quote! {
            ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
        },
        _ => quote! {},
    };

    // TODO: don't use Interface::vtable trait for winrt/winrt calls - just stamp out vtable type directly

    let vcall = if let Some(return_sig) = &signature.return_sig {
        if return_sig.is_array {
            quote! {
                let mut result__: #return_type_tokens = ::core::mem::zeroed();
                (::windows::core::Interface::vtable(this).#vtable_offset)(::core::mem::transmute_copy(this), #(#args,)* #composable_args #return_arg)
                    .and_then(|| result__ )
            }
        } else {
            let abi_type_name = gen_abi_element_name(return_sig, gen);

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

    match kind {
        InterfaceKind::Default => quote! {
            #cfg
            pub fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                let this = self;
                unsafe {
                    #vcall
                }
            }
        },
        InterfaceKind::NonDefault | InterfaceKind::Base => {
            quote! {
                #cfg
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
                #cfg
                pub fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type_tokens> {
                    Self::#interface_name(|this| unsafe { #vcall })
                }
            }
        }
        InterfaceKind::Extend | InterfaceKind::Overridable => unimplemented!(), // TODO: should remove these flags
    }
}

pub fn gen_com_method(def: &TypeDef, method: &MethodDef, vtable_offset: usize, method_names: &mut BTreeMap<String, u32>, gen: &Gen) -> TokenStream {
    let signature = method.signature(&def.generics);

    let name = method.rust_name();
    let overload = method_names.entry(name.to_string()).or_insert(0);
    *overload += 1;
    let name = if *overload > 1 { format!("{}{}", name, overload).into() } else { gen_ident(&name) };

    let constraints = gen_param_constraints(&signature.params, gen);
    let cfg = gen.method_cfg(def, method).gen_with_doc(gen);
    let vtable_offset = Literal::usize_unsuffixed(vtable_offset);

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #cfg
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params) -> ::windows::core::Result<T> {
                    let mut result__ = ::core::option::Option::None;
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #cfg
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            let mut return_sig = signature.params[signature.params.len() - 1].signature.clone();
            return_sig.pointers -= 1;
            let return_type_tokens = gen_result_sig(&return_sig, gen);
            let abi_return_type_tokens = gen_abi_sig(&return_sig, gen);

            quote! {
                #cfg
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    let mut result__: #abi_return_type_tokens = ::core::mem::zeroed();
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* ::core::mem::transmute(&mut result__))
                    .from_abi::<#return_type_tokens>(result__ )
                }
            }
        }
        SignatureKind::ResultVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #cfg
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*).ok()
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_abi_element_name(&signature.return_sig.unwrap(), gen);

            quote! {
                #cfg
                pub unsafe fn #name<#constraints>(&self, #params) -> #return_sig {
                    let mut result__: #return_sig = :: core::mem::zeroed();
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), &mut result__ #(,#args)*);
                    result__
                }
            }
        }
        SignatureKind::PreserveSig => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_return_sig(&signature, gen);

            quote! {
                #cfg
                pub unsafe fn #name<#constraints>(&self, #params) #return_sig {
                    ::core::mem::transmute((::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*))
                }
            }
        }
        SignatureKind::ReturnVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #cfg
                pub unsafe fn #name<#constraints>(&self, #params) {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*)
                }
            }
        }
    }
}

pub fn gen_win32_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut tokens = quote! {};

    for (position, param) in params.iter().enumerate() {
        let name = gen_param_name(&param.param);
        let kind = if param.is_convertible() { format!("Param{}", position).into() } else { gen_param_sig(param, gen) };
        tokens.combine(&quote! { #name: #kind, });
    }

    tokens
}

pub fn gen_winrt_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut result = quote! {};

    for (position, param) in params.iter().enumerate() {
        let name = gen_param_name(&param.param);
        let kind = gen_element_name(&param.signature.kind, gen);

        if param.signature.is_array {
            if param.param.is_input() {
                result.combine(&quote! { #name: &[<#kind as ::windows::core::DefaultType>::DefaultType], });
            } else if param.signature.by_ref {
                result.combine(&quote! { #name: &mut ::windows::core::Array<#kind>, });
            } else {
                result.combine(&quote! { #name: &mut [<#kind as ::windows::core::DefaultType>::DefaultType], });
            }
        } else if param.param.is_input() {
            if param.is_convertible() {
                let kind: TokenStream = format!("Param{}", position).into();
                result.combine(&quote! { #name: #kind, });
            } else {
                result.combine(&quote! { #name: #kind, });
            }
        } else if param.signature.kind.is_nullable() {
            result.combine(&quote! { #name: &mut ::core::option::Option<#kind>, });
        } else if let ElementType::GenericParam(_) = param.signature.kind {
            result.combine(&quote! { &mut <#kind as ::windows::core::DefaultType>::DefaultType, });
        } else {
            result.combine(&quote! { #name: &mut #kind, });
        };
    }

    result
}

pub fn gen_win32_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.param);

    if param.is_convertible() {
        quote! { #name.into_param().abi() }
    } else {
        quote! { ::core::mem::transmute(#name) }
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
