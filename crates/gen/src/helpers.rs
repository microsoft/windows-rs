use crate::*;

pub fn gen_method_constraints(params: &[MethodParam]) -> TokenStream {
    if params.iter().any(|param| param.is_convertible()) {
        quote! { 'a, }
    } else {
        quote! {}
    }
}

pub fn gen_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    // TODO: this isn't correct since the signature alone isn't enough to tell whether its const - the param might be const as well
    for _ in 0..sig.pointers {
        if sig.is_const {
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
