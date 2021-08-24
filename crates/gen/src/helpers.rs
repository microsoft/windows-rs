use crate::*;

pub fn gen_method_constraints(params: &[MethodParam]) -> TokenStream {
    if params.iter().any(|param| param.is_convertible()) {
        quote! { 'a, }
    } else {
        quote! {}
    }
}

pub fn gen_win32_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_sig_with_const(&param.signature, gen, !param.param.flags().output())
}

pub fn gen_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    gen_sig_with_const(sig, gen, false)
}

// TODO: mostly the same as gen_win32_abi_param
pub fn gen_abi_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    // TODO: this isn't correct since the signature alone isn't enough to tell whether its const - the param might be const as well
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

fn gen_sig_with_const(sig: &Signature, gen: &Gen, is_const: bool) -> TokenStream {
    let mut tokens = TokenStream::new();

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
