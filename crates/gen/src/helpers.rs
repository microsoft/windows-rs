use crate::*;

pub fn gen_method_constraints(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

    for (position, param) in params.iter().enumerate() {
        if param.is_convertible() {
            let name = format_token!("Param{}", position);
            let into = gen_name(&param.signature.kind, gen);
            tokens.combine(&quote! { #name: ::windows::IntoParam<'a, #into>, });
        }
    }

    if !tokens.is_empty() {
        quote! { 'a, #tokens }
    } else {
        TokenStream::new()
    }
}

pub fn gen_win32_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_sig_with_const(&param.signature, gen, !param.param.flags().output())
}

pub fn gen_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    gen_sig_with_const(sig, gen, sig.is_const)
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

    let kind = gen_name(&sig.kind, gen);

    if sig.kind.is_nullable() {
        tokens.combine(&quote! {
            ::std::option::Option<#kind>
        });
    } else {
        tokens.combine(&kind)
    }

    tokens
}

pub fn gen_abi_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

    for _ in 0..sig.pointers {
        if sig.is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    tokens.combine(&gen_abi_type_name(&sig.kind, gen));
    tokens
}

pub fn method_features(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    if !gen.features {
        return TokenStream::new();
    }

    let mut features = sig.method_features();

    let mut relative = gen.relative;

    while !relative.is_empty() {
        features.remove(relative);
        if let Some(pos) = relative.rfind('.') {
            relative = &relative[..pos];
        } else {
            relative = "";
        }
    }

    if features.is_empty() {
        return TokenStream::new(); 
    }

    let mut tokens = TokenStream::with_capacity();

    // TODO: build cfg feature macro 

    // TODO: code should be shared with toml builder

    

    tokens
}