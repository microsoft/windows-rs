use super::*;

pub fn gen_winrt_method(def: &TypeDef, method: &MethodDef, gen: &Gen) -> TokenStream {
    let signature = method.signature(&def.generics);
    let name = gen_ident(&method.rust_name());
    let constraints = gen_param_constraints(&signature.params, gen);
    let arch_cfg = gen.arch_cfg(method.attributes());
    let feature_cfg = gen.method_cfg(&method).0;
    let params = gen_params(&signature.params, gen);

    let return_type = if let Some(return_sig) = &signature.return_sig {
        let tokens = gen_result_sig(return_sig, gen);
        quote! { -> ::windows::core::Result<#tokens> }
    } else {
        quote! { -> ::windows::core::Result<()> }
    };

    quote! {
        #arch_cfg
        #feature_cfg
        pub fn #name<#constraints>(&self, #params) #return_type {
            unimplemented!()
        }
    }
}

pub fn gen_com_method(def: &TypeDef, method: &MethodDef, vtable_offset: usize, method_names: &mut BTreeMap<String, u32>, gen: &Gen) -> TokenStream {
    let signature = method.signature(&def.generics);

    let name = method.rust_name();
    let overload = method_names.entry(name.to_string()).or_insert(0);
    *overload += 1;
    let name = if *overload > 1 { format!("{}{}", name, overload).into() } else { gen_ident(&name) };

    let constraints = gen_param_constraints(&signature.params, gen);
    let arch_cfg = gen.arch_cfg(method.attributes());
    let feature_cfg = gen.method_cfg(&method).0;
    let vtable_offset = Literal::usize_unsuffixed(vtable_offset);

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_params(leading_params, gen);

            quote! {
                #arch_cfg
                #feature_cfg
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params) -> ::windows::core::Result<T> {
                    let mut result__ = ::core::option::Option::None;
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_params(leading_params, gen);

            quote! {
                #arch_cfg
                #feature_cfg
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_params(leading_params, gen);
            let return_type_tokens = gen_result_sig(&signature.params[signature.params.len() - 1].signature, gen);

            quote! {
                #arch_cfg
                #feature_cfg
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    let mut result__: <#return_type_tokens as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &mut result__)
                    .from_abi::<#return_type_tokens>(result__ )
                }
            }
        }
        SignatureKind::ResultVoid => {
            let params = gen_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #arch_cfg
                #feature_cfg
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*).ok()
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let params = gen_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_abi_element_name(&signature.return_sig.unwrap().kind, gen);

            quote! {
                #arch_cfg
                #feature_cfg
                pub unsafe fn #name<#constraints>(&self, #params) -> #return_sig {
                    let mut result__: #return_sig = :: core::mem::zeroed();
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), &mut result__ #(,#args)*);
                    result__
                }
            }
        }
        SignatureKind::PreserveSig => {
            let params = gen_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_return_sig(&signature, gen);

            quote! {
                #arch_cfg
                #feature_cfg
                pub unsafe fn #name<#constraints>(&self, #params) #return_sig {
                    ::core::mem::transmute((::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*))
                }
            }
        }
    }
}

pub fn gen_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut tokens = quote! {};

    for (position, param) in params.iter().enumerate() {
        let name = gen_param_name(&param.param);
        let kind = if param.is_convertible() { format!("Param{}", position).into() } else { gen_param_sig(param, gen) };
        tokens.combine(&quote! { #name: #kind, });
    }

    tokens
}

pub fn gen_win32_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.param);

    if param.is_convertible() {
        quote! { #name.into_param().abi() }
    } else {
        quote! { ::core::mem::transmute(#name) }
    }
}
