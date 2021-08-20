use super::*;

pub fn gen_win32_abi(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    // TODO: param insead of p consistency
    let params = sig.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let (udt_return_type, return_type) = if let Some(t) = &sig.return_type {
        if t.is_udt() {
            let mut t = t.clone();
            t.pointers += 1;
            let tokens = gen_abi_sig(&t, gen);
            (quote! { result__: #tokens }, quote! {})
        } else {
            let tokens = gen_abi_sig(t, gen);
            (quote! {}, quote! { -> #tokens })
        }
    } else {
        (TokenStream::new(), TokenStream::new())
    };

    quote! {
        (this: ::windows::RawPtr, #(#params,)* #udt_return_type) #return_type
    }
}

pub fn gen_win32_invoke_arg(param: &MethodParam, gen: &Gen) -> TokenStream {
    let name = gen_param_name(&param.param);
    let kind = gen_name(&param.signature.kind, gen);

    if param.param.is_input() {
        if param.signature.kind.is_blittable() {
            quote! { #name }
        } else {
            quote! { &*(&#name as *const <#kind as ::windows::Abi>::Abi as *const <#kind as ::windows::Abi>::DefaultType) }
        }
    } else {
        quote! { ::std::mem::transmute_copy(&#name) }
    }
}

pub fn gen_win32_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();
    let is_const = param.is_const();

    for _ in 0..param.signature.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    let kind = gen_name(&param.signature.kind, gen);

    if param.signature.kind.is_nullable() {
        tokens.combine(&quote! {
            ::std::option::Option<#kind>
        });
    } else {
        tokens.combine(&kind)
    }

    tokens
}

pub fn gen_win32_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    params
        .iter()
        .map(|param| {
            let name = gen_param_name(&param.param);

            if param.is_convertible() {
                let into = gen_name(&param.signature.kind, gen);
                quote! { #name: impl ::windows::IntoParam<'a, #into>, }
            } else {
                let tokens = gen_win32_param(param, gen);
                quote! { #name: #tokens, }
            }
        })
        .collect()
}

pub fn gen_win32_abi_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for _ in 0..param.signature.pointers {
        if param.is_const() {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    if param.signature.pointers > 0 && param.signature.kind.is_udt() {
        tokens.combine(&gen_name(&param.signature.kind, gen));
    } else {
        tokens.combine(&gen_abi_type_name(&param.signature.kind, gen));
    }

    tokens
}

pub fn gen_win32_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.param);

    if param.is_convertible() {
        quote! { #name.into_param().abi() }
    } else {
        quote! { ::std::mem::transmute(#name) }
    }
}

pub fn gen_win32_upcall(sig: &MethodSignature, inner: TokenStream, gen: &Gen) -> TokenStream {
    if sig.has_query_interface() {
        quote! {
            unimplemented!("one")
        }
    } else if sig.has_retval() {
        let invoke_args = sig.params[..sig.params.len() - 1]
            .iter()
            .map(|param| gen_win32_invoke_arg(param, gen));

        let result = gen_param_name(&sig.params[sig.params.len() - 1].param);

        quote! {
            match #inner(#(#invoke_args,)*) {
                ::std::result::Result::Ok(ok__) => {
                    *#result = ::std::mem::transmute_copy(&ok__);
                    ::std::mem::forget(ok__);
                    ::windows::HRESULT(0)
                }
                ::std::result::Result::Err(err) => err.into()
            }
        }
    } else if sig.has_udt_return() {
        quote! {
            unimplemented!("three")
        }
    } else if let Some(return_type) = &sig.return_type {
        if return_type.kind == ElementType::HRESULT {
            let invoke_args = sig
                .params
                .iter()
                .map(|param| gen_win32_invoke_arg(param, gen));

            quote! {
                #inner(#(#invoke_args,)*).into()
            }
        } else {
            quote! {
                unimplemented!("five")
            }
        }
    } else {
        quote! {
            unimplemented!("six")
        }
    }
}
