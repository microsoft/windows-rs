use crate::*;

pub fn gen_method_constraints(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (position, param) in params.iter().enumerate() {
        if param.is_convertible() {
            let name = format_token!("Param{}", position);
            let into = gen_name(&param.signature.kind, gen);
            tokens.combine(&quote! { #name: ::windows::core::IntoParam<'a, #into>, });
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
            ::core::option::Option<#kind>
        });
    } else {
        tokens.combine(&kind)
    }

    tokens
}

pub fn gen_abi_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

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

pub fn interface_method_features(def: &TypeDef, sig: &MethodSignature, gen: &Gen) -> BTreeSet<&'static str> {
    if gen.root.is_empty() {
        return BTreeSet::new();
    }

    let mut features = sig.method_features();
    features.insert(def.namespace());
    features
}

// TODO: should have option to return "not" version to avoid calculating method_features multiple times
pub fn method_features(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    if gen.root.is_empty() {
        return TokenStream::new();
    }

    gen.gen_cfg(&sig.method_features())
}

// TODO: wasteful - don't do this
pub fn not_method_features(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    if gen.root.is_empty() {
        return TokenStream::new();
    }

    gen.gen_cfg_not(&sig.method_features())
}

pub fn signature_features(sig: &Signature, gen: &Gen) -> BTreeSet<&'static str> {
    if gen.root.is_empty() {
        return BTreeSet::new();
    }

    let mut features = BTreeSet::new();
    let mut keys = std::collections::HashSet::new();
    sig.kind.features(&mut features, &mut keys);
    features
}

pub fn features(def: &TypeDef, gen: &Gen) -> BTreeSet<&'static str> {
    if gen.root.is_empty() {
        return BTreeSet::new();
    }

    let mut features = BTreeSet::new();
    let mut keys = std::collections::HashSet::new();
    def.features(&mut features, &mut keys);
    features
}
