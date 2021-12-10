use super::*;

pub fn gen_win32_abi(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    // TODO: param insead of p consistency
    let params = sig.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let (udt_return_type, return_sig) = if let Some(t) = &sig.return_sig {
        if t.is_udt() {
            let mut t = t.clone();
            t.pointers += 1;
            let tokens = gen_abi_sig(&t, gen);
            (quote! { , result__: #tokens }, quote! {})
        } else {
            let tokens = gen_abi_sig(t, gen);
            (quote! {}, quote! { -> #tokens })
        }
    } else {
        (TokenStream::new(), TokenStream::new())
    };

    quote! {
        (this: ::windows::core::RawPtr #udt_return_type #(,#params)*) #return_sig
    }
}

fn gen_win32_invoke_arg(param: &MethodParam, gen: &Gen) -> TokenStream {
    let name = gen_param_name(&param.param);
    let kind = gen_win32_param(param, gen);

    if param.signature.pointers == 0 && param.signature.kind.is_nullable() {
        quote! { ::core::mem::transmute::<_, &#kind>(&#name) }
    } else {
        quote! { ::core::mem::transmute_copy::<_, #kind>(&#name) }
    }
}

pub fn gen_win32_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    params
        .iter()
        .enumerate()
        .map(|(position, param)| {
            let name = gen_param_name(&param.param);

            if param.is_convertible() {
                let into = format_token!("Param{}", position);
                quote! { #name: #into, }
            } else {
                let tokens = gen_win32_param(param, gen);
                quote! { #name: #tokens, }
            }
        })
        .collect()
}

pub fn gen_win32_abi_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();
    let is_const = !param.param.flags().output();

    for _ in 0..param.signature.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    if param.signature.pointers > 1 && param.signature.kind.is_udt() {
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
        quote! { ::core::mem::transmute(#name) }
    }
}

pub fn gen_win32_upcall(sig: &MethodSignature, inner: TokenStream, gen: &Gen) -> TokenStream {
    match sig.kind() {
        SignatureKind::ResultValue => {
            let invoke_args = sig.params[..sig.params.len() - 1].iter().map(|param| gen_win32_invoke_arg(param, gen));

            let result = gen_param_name(&sig.params[sig.params.len() - 1].param);

            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::core::result::Result::Ok(ok__) => {
                        *#result = ::core::mem::transmute(ok__);
                        ::windows::core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into()
                }
            }
        }
        SignatureKind::ResultVoid => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(param, gen));

            quote! {
                #inner(#(#invoke_args,)*).into()
            }
        }
        SignatureKind::ReturnStruct => {
            unimplemented!("ReturnStruct")
        }
        _ => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(param, gen));

            quote! {
                #inner(#(#invoke_args,)*)
            }
        }
    }
}

pub fn gen_win32_result_type(signature: &MethodSignature, gen: &Gen) -> TokenStream {
    let mut return_param = signature.params[signature.params.len() - 1].clone();

    if return_param.signature.pointers > 1 {
        return_param.signature.pointers -= 1;
        gen_win32_param(&return_param, gen)
    } else {
        gen_name(&return_param.signature.kind, gen)
    }
}

pub fn gen_win32_return_sig(signature: &MethodSignature, gen: &Gen) -> TokenStream {
    if let Some(return_sig) = &signature.return_sig {
        let tokens = gen_sig(return_sig, gen);
        quote! { -> #tokens }
    } else {
        TokenStream::new()
    }
}
