use super::*;

// TODO take Signature instead of MethodDef (wherever MethodDef is found)
pub fn gen(
    gen: &Gen,
    def: TypeDef,
    generic_types: &[Type],
    kind: InterfaceKind,
    method: MethodDef,
    method_names: &mut MethodNames,
    virtual_names: &mut MethodNames,
) -> TokenStream {
    let signature = gen.reader.method_def_signature(method, generic_types);
    let params = &signature.params;
    let name = method_names.add(gen, method);
    let interface_name = gen.type_def_name(def, generic_types);
    let vname = virtual_names.add(gen, method);
    let generics = gen.constraint_generics(params);
    let where_clause = gen.where_clause(params);
    let mut cfg = gen.reader.signature_cfg(&signature);
    gen.reader
        .type_def_cfg_combine(def, generic_types, &mut cfg);
    let doc = gen.cfg_method_doc(&cfg);
    let features = gen.cfg_features(&cfg);
    let args = gen_winrt_abi_args(gen, params);
    let params = gen_winrt_params(gen, params);

    let return_type_tokens = match &signature.return_type {
        Type::Void => quote! { () },
        _ => {
            let tokens = gen.type_name(&signature.return_type);
            if signature.return_type.is_winrt_array() {
                quote! { ::windows_core::Array<#tokens> }
            } else {
                quote! { #tokens }
            }
        }
    };

    let return_arg = match &signature.return_type {
        Type::Void => quote! {},
        _ => {
            if signature.return_type.is_winrt_array() {
                let return_type = gen.type_name(&signature.return_type);
                quote! { ::windows_core::Array::<#return_type>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _ }
            } else {
                quote! { &mut result__ }
            }
        }
    };

    let vcall = match &signature.return_type {
        Type::Void => {
            quote! {
                (::windows_core::Interface::vtable(this).#vname)(::windows_core::Interface::as_raw(this), #args).ok()
            }
        }
        _ if signature.return_type.is_winrt_array() => {
            quote! {
                let mut result__ = ::core::mem::MaybeUninit::zeroed();
                (::windows_core::Interface::vtable(this).#vname)(::windows_core::Interface::as_raw(this), #args #return_arg)
                    .and_then(|| result__.assume_init())
            }
        }
        _ => {
            let return_type = gen.type_name(&signature.return_type);
            quote! {
                let mut result__ = ::windows_core::zeroed::<#return_type>();
                    (::windows_core::Interface::vtable(this).#vname)(::windows_core::Interface::as_raw(this), #args #return_arg)
                        .from_abi(result__)
            }
        }
    };

    match kind {
        InterfaceKind::Default => quote! {
            #doc
            #features
            pub fn #name<#generics>(&self, #params) -> ::windows_core::Result<#return_type_tokens> #where_clause {
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
                pub fn #name<#generics>(&self, #params) -> ::windows_core::Result<#return_type_tokens> #where_clause {
                    let this = &::windows_core::ComInterface::cast::<#interface_name>(self)?;
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
                pub fn #name<#generics>(#params) -> ::windows_core::Result<#return_type_tokens> #where_clause {
                    Self::#interface_name(|this| unsafe { #vcall })
                }
            }
        }
    }
}

fn gen_winrt_params(gen: &Gen, params: &[SignatureParam]) -> TokenStream {
    let mut result = quote! {};

    let mut generic_params = gen.generic_params(params);
    for param in params.iter() {
        let name = gen.param_name(param.def);
        let kind = gen.type_name(&param.ty);
        let default_type = gen.type_default_name(&param.ty);

        if gen
            .reader
            .param_flags(param.def)
            .contains(ParamAttributes::INPUT)
        {
            if param.ty.is_winrt_array() {
                result.combine(&quote! { #name: &[#default_type], });
            } else if gen.reader.signature_param_is_convertible(param) {
                let (position, _) = generic_params.next().unwrap();
                let kind: TokenStream = format!("P{position}").into();
                result.combine(&quote! { #name: #kind, });
            } else if gen.reader.type_is_blittable(&param.ty) {
                result.combine(&quote! { #name: #kind, });
            } else {
                result.combine(&quote! { #name: &#kind, });
            }
        } else if param.ty.is_winrt_array() {
            result.combine(&quote! { #name: &mut [#default_type], });
        } else if param.ty.is_winrt_array_ref() {
            result.combine(&quote! { #name: &mut ::windows_core::Array<#kind>, });
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

        let param = if gen
            .reader
            .param_flags(param.def)
            .contains(ParamAttributes::INPUT)
        {
            if param.ty.is_winrt_array() {
                if gen.reader.type_is_blittable(&param.ty) {
                    quote! { #name.len() as u32, #name.as_ptr(), }
                } else {
                    quote! { #name.len() as u32, ::core::mem::transmute(#name.as_ptr()), }
                }
            } else if gen.reader.signature_param_is_failible_param(param) {
                quote! { #name.try_into_param()?.abi(), }
            } else if gen.reader.signature_param_is_borrowed(param) {
                quote! { #name.into_param().abi(), }
            } else if gen.reader.type_is_blittable(&param.ty) {
                if param.ty.is_const_ref() {
                    quote! { &#name, }
                } else {
                    quote! { #name, }
                }
            } else {
                quote! { ::core::mem::transmute_copy(#name), }
            }
        } else if param.ty.is_winrt_array() {
            if gen.reader.type_is_blittable(&param.ty) {
                quote! { #name.len() as u32, #name.as_mut_ptr(), }
            } else {
                quote! { #name.len() as u32, ::core::mem::transmute_copy(&#name), }
            }
        } else if param.ty.is_winrt_array_ref() {
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
    let invoke_args = sig
        .params
        .iter()
        .map(|param| gen_winrt_invoke_arg(gen, param));

    match &sig.return_type {
        Type::Void => quote! {
            #inner(#(#invoke_args,)*).into()
        },
        _ if sig.return_type.is_winrt_array() => {
            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::core::result::Result::Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        // use `core::ptr::write` since `result` could be uninitialized
                        ::core::ptr::write(result__, ok_data__);
                        ::core::ptr::write(result_size__, ok_data_len__);
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into()
                }
            }
        }
        _ => {
            let forget = if gen.reader.type_is_blittable(&sig.return_type) {
                quote! {}
            } else {
                quote! { ::core::mem::forget(ok__); }
            };

            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::core::result::Result::Ok(ok__) => {
                        // use `core::ptr::write` since `result` could be uninitialized
                        ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                        #forget
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into()
                }
            }
        }
    }
}

fn gen_winrt_invoke_arg(gen: &Gen, param: &SignatureParam) -> TokenStream {
    let name = gen.param_name(param.def);
    let abi_size_name: TokenStream =
        format!("{}_array_size", gen.reader.param_name(param.def)).into();

    if gen
        .reader
        .param_flags(param.def)
        .contains(ParamAttributes::INPUT)
    {
        if param.ty.is_winrt_array() {
            quote! { ::core::slice::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name as _) }
        } else if gen.reader.type_is_primitive(&param.ty) {
            quote! { #name }
        } else if param.ty.is_const_ref() {
            quote! { ::core::mem::transmute_copy(&#name) }
        } else if gen.reader.type_is_nullable(&param.ty) {
            quote! { ::windows_core::from_raw_borrowed(&#name) }
        } else {
            quote! { ::core::mem::transmute(&#name) }
        }
    } else if param.ty.is_winrt_array() {
        quote! { ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&#name), #abi_size_name as _) }
    } else if param.ty.is_winrt_array_ref() {
        quote! { ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name).as_array() }
    } else {
        quote! { ::core::mem::transmute_copy(&#name) }
    }
}
