use super::*;

// TODO: replace with gen_default_type?
pub fn gen_sig(ty: &Type, gen: &Gen) -> TokenStream {
    let kind = gen_element_name(ty, gen);

    if ty.is_nullable() && !gen.sys {
        quote! {
            ::core::option::Option<#kind>
        }
    } else {
        kind
    }
}

// TODO: suspect - check for UDT
pub fn gen_return_sig(signature: &Signature, gen: &Gen) -> TokenStream {
    if let Some(return_type) = &signature.return_type {
        let tokens = gen_sig(return_type, gen);
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
            let into = gen_element_name(&param.ty, gen);
            tokens.combine(&quote! { #name: ::windows::core::IntoParam<'a, #into>, });
        }
    }

    if !tokens.is_empty() {
        quote! { 'a, #tokens }
    } else {
        tokens
    }
}
