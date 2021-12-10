use super::*;

pub fn gen_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    gen_sig_with_const(sig, gen, sig.is_const)
}

pub fn gen_abi_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    gen_abi_sig_with_const(sig, gen, sig.is_const)
}

pub fn gen_param_sig(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_sig_with_const(&param.signature, gen, !param.param.flags().output())
}

pub fn gen_abi_param_sig(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_abi_sig_with_const(&param.signature, gen, !param.param.flags().output())
}

// TODO: suspect - check for UDT
pub fn gen_return_sig(signature: &MethodSignature, gen: &Gen) -> TokenStream {
    if let Some(return_sig) = &signature.return_sig {
        let tokens = gen_sig(return_sig, gen);
        quote! { -> #tokens }
    } else {
        quote! {}
    }
}

pub fn gen_param_constraints(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut tokens = quote! {};

    for (position, param) in params.iter().enumerate() {
        if param.is_convertible() {
            let name: TokenStream = format!("Param{}", position).into();
            let into = gen_element_name(&param.signature.kind, gen);
            tokens.combine(&quote! { #name: ::windows::core::IntoParam<'a, #into>, });
        }
    }

    if !tokens.is_empty() {
        quote! { 'a, #tokens }
    } else {
        tokens
    }
}

fn gen_abi_sig_with_const(sig: &Signature, gen: &Gen, is_const: bool) -> TokenStream {
    let mut tokens = TokenStream::new();

    for _ in 0..sig.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    tokens.combine(&gen_abi_element_name(sig, gen));
    tokens
}

pub fn gen_result_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for _ in 0..sig.pointers {
        if sig.is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    tokens.combine(&gen_element_name(&sig.kind, gen));
    tokens
}

fn gen_sig_with_const(sig: &Signature, gen: &Gen, is_const: bool) -> TokenStream {
    let mut tokens = TokenStream::new();

    for _ in 0..sig.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    let kind = gen_element_name(&sig.kind, gen);

    // TODO: harmonize these across sys/win
    if sig.kind.is_nullable() && !gen.sys {
        tokens.combine(&quote! {
            ::core::option::Option<#kind>
        });
    } else {
        tokens.combine(&kind)
    }

    tokens
}
