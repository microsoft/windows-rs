use super::*;

pub fn gen_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    gen_sig_with_const(sig, gen, sig.is_const)
}

pub fn gen_param_sig(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_sig_with_const(&param.signature, gen, !param.param.flags().output())
}

pub fn gen_abi_param_sig(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_abi_sig_with_const(&param.signature, gen, !param.param.flags().output())
}

pub fn gen_return_sig(signature: &MethodSignature, gen: &Gen) -> TokenStream {
    if let Some(return_sig) = &signature.return_sig {
        let tokens = gen_sig(return_sig, gen);
        quote! { -> #tokens }
    } else {
        quote! {}
    }
}

fn gen_abi_sig_with_const(sig: &Signature, gen: &Gen, is_const: bool) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

    for _ in 0..sig.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    tokens.combine(&gen_element_name(&sig.kind, gen));
    tokens
}

fn gen_sig_with_const(sig: &Signature, gen: &Gen, is_const: bool) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

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
