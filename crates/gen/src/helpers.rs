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

pub fn interface_method_features(def: &TypeDef, sig: &MethodSignature, gen: &Gen) -> TokenStream {
    if gen.feature.is_empty() {
        return TokenStream::new();
    }

    let mut features = sig.method_features();
    features.insert(def.namespace());
    gen_cfg(features, false, gen)
}

// TODO: should have option to return "not" version to avoid calculating method_features multiple times
pub fn method_features(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    if gen.feature.is_empty() {
        return TokenStream::new();
    }

    gen_cfg(sig.method_features(), false, gen)
}

// TODO: wasteful - don't do this
pub fn not_method_features(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    if gen.feature.is_empty() {
        return TokenStream::new();
    }

    gen_cfg(sig.method_features(), true, gen)
}

pub fn signature_features(sig: &Signature, gen: &Gen) -> TokenStream {
    if gen.feature.is_empty() {
        return TokenStream::new();
    }

    let mut features = std::collections::BTreeSet::new();
    let mut keys = std::collections::HashSet::new();
    sig.kind.method_features(&mut features, &mut keys);

    gen_cfg(features, false, gen)
}

pub fn struct_features(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.feature.is_empty() {
        return TokenStream::new();
    }

    let mut features = std::collections::BTreeSet::new();
    let mut keys = std::collections::HashSet::new();
    def.struct_features(&mut features, &mut keys);

    gen_cfg(features, false, gen)
}

pub fn convert_features(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.feature.is_empty() {
        return TokenStream::new();
    }

    let mut features = std::collections::BTreeSet::new();
    features.insert(def.namespace());

    gen_cfg(features, false, gen)
}

pub fn class_features(def: &TypeDef, gen: &Gen) -> TokenStream {
        if gen.feature.is_empty() {
            return TokenStream::new();
        }
    
        let mut features = std::collections::BTreeSet::new();
        
        if let Some(def)  = def.default_interface() {
            features.insert(def.namespace());
        }
    
        gen_cfg(features, false, gen)
    }

fn gen_cfg(mut features: BTreeSet<&'static str>, not: bool, gen: &Gen) -> TokenStream {
    // TODO: remove any features that are already module features dependencies as these are redundant

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

    // TODO: don't use all(...) if there's only a single feature (as is often the case)

    let mut dependencies = if not {
        "#[cfg(not(all(".to_string()
    } else {
        "#[cfg(all(".to_string()
    };

    for feature in features {
        let feature = &feature[gen.feature.len() + 1..];
        dependencies.push_str(&format!("feature = \"{}\", ", feature.replace('.', "_")));
    }

    dependencies.truncate(dependencies.len() - 2);
    if not {
        dependencies.push(')');
    }
    dependencies.push_str("))]");

    dependencies.into()
}
