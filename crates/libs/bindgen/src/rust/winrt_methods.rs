use super::*;

// TODO take Signature instead of MethodDef (wherever MethodDef is found)
pub fn writer(writer: &Writer, def: TypeDef, generic_types: &[Type], kind: InterfaceKind, method: MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames) -> TokenStream {
    let signature = method_def_signature(writer.reader, writer.reader.type_def_namespace(def), method, generic_types);
    let params = &signature.params;
    let name = method_names.add(writer, method);
    let interface_name = writer.type_def_name(def, generic_types);
    let vname = virtual_names.add(writer, method);
    let generics = writer.constraint_generics(params);
    let where_clause = writer.where_clause(params);
    let mut cfg = signature_cfg(writer.reader, method);
    type_def_cfg_combine(writer.reader, def, generic_types, &mut cfg);
    let doc = writer.cfg_method_doc(&cfg);
    let features = writer.cfg_features(&cfg);
    let args = gen_winrt_abi_args(writer, params);
    let params = gen_winrt_params(writer, params);

    let return_type_tokens = match &signature.return_type {
        Type::Void => quote! { () },
        _ => {
            let tokens = writer.type_name(&signature.return_type);
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
                let return_type = writer.type_name(&signature.return_type);
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
            quote! {
                let mut result__ = ::std::mem::zeroed();
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

fn gen_winrt_params(writer: &Writer, params: &[SignatureParam]) -> TokenStream {
    let mut result = quote! {};

    let mut generic_params = writer.generic_params(params);
    for param in params.iter() {
        let name = writer.param_name(param.def);
        let kind = writer.type_name(&param.ty);
        let default_type = writer.type_default_name(&param.ty);

        if writer.reader.param_flags(param.def).contains(ParamAttributes::In) {
            if param.ty.is_winrt_array() {
                result.combine(&quote! { #name: &[#default_type], });
            } else if signature_param_is_convertible(writer.reader, param) {
                let (position, _) = generic_params.next().unwrap();
                let kind: TokenStream = format!("P{position}").into();
                result.combine(&quote! { #name: #kind, });
            } else if type_is_blittable(writer.reader, &param.ty) {
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

fn gen_winrt_abi_args(writer: &Writer, params: &[SignatureParam]) -> TokenStream {
    let mut tokens = TokenStream::new();
    for param in params {
        let name = writer.param_name(param.def);

        let param = if writer.reader.param_flags(param.def).contains(ParamAttributes::In) {
            if param.ty.is_winrt_array() {
                if type_is_blittable(writer.reader, &param.ty) {
                    quote! { #name.len() as u32, #name.as_ptr(), }
                } else {
                    quote! { #name.len() as u32, ::core::mem::transmute(#name.as_ptr()), }
                }
            } else if type_is_non_exclusive_winrt_interface(writer.reader, &param.ty) {
                quote! { #name.try_into_param()?.abi(), }
            } else if signature_param_is_borrowed(writer.reader, param) {
                quote! { #name.into_param().abi(), }
            } else if type_is_blittable(writer.reader, &param.ty) {
                if param.ty.is_const_ref() {
                    quote! { &#name, }
                } else {
                    quote! { #name, }
                }
            } else {
                quote! { ::core::mem::transmute_copy(#name), }
            }
        } else if param.ty.is_winrt_array() {
            if type_is_blittable(writer.reader, &param.ty) {
                quote! { #name.len() as u32, #name.as_mut_ptr(), }
            } else {
                quote! { #name.len() as u32, ::core::mem::transmute_copy(&#name), }
            }
        } else if param.ty.is_winrt_array_ref() {
            quote! { #name.set_abi_len(), #name as *mut _ as _, }
        } else if type_is_blittable(writer.reader, &param.ty) {
            quote! { #name, }
        } else {
            quote! { #name as *mut _ as _, }
        };
        tokens.combine(&param);
    }
    tokens
}

pub fn gen_upcall(writer: &Writer, sig: &Signature, inner: TokenStream) -> TokenStream {
    let invoke_args = sig.params.iter().map(|param| gen_winrt_invoke_arg(writer, param));

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
            let forget = if type_is_blittable(writer.reader, &sig.return_type) {
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

fn gen_winrt_invoke_arg(writer: &Writer, param: &SignatureParam) -> TokenStream {
    let name = writer.param_name(param.def);
    let abi_size_name: TokenStream = format!("{}_array_size", writer.reader.param_name(param.def)).into();

    if writer.reader.param_flags(param.def).contains(ParamAttributes::In) {
        if param.ty.is_winrt_array() {
            quote! { ::core::slice::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name as usize) }
        } else if type_is_primitive(writer.reader, &param.ty) {
            quote! { #name }
        } else if param.ty.is_const_ref() {
            quote! { ::core::mem::transmute_copy(&#name) }
        } else if type_is_nullable(writer.reader, &param.ty) {
            quote! { ::windows_core::from_raw_borrowed(&#name) }
        } else {
            quote! { ::core::mem::transmute(&#name) }
        }
    } else if param.ty.is_winrt_array() {
        quote! { ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&#name), #abi_size_name as usize) }
    } else if param.ty.is_winrt_array_ref() {
        quote! { ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name).as_array() }
    } else {
        quote! { ::core::mem::transmute_copy(&#name) }
    }
}
