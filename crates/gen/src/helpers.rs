use crate::*;

pub fn gen_phantoms(def: &TypeDef) -> impl Iterator<Item = TokenStream> + '_ {
    def.generics.iter().map(move |g| {
        let g = gen_name(g, &Gen::Absolute);
        quote! { ::std::marker::PhantomData::<#g> }
    })
}

pub fn gen_constraints(def: &TypeDef) -> TokenStream {
    def.generics
        .iter()
        .map(|g| {
            let g = gen_name(g, &Gen::Absolute);
            quote! { #g: ::windows::RuntimeType + 'static, }
        })
        .collect()
}

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

pub fn gen_sig_default(sig: &Signature) -> TokenStream {
    if sig.pointers > 0 {
        quote! { ::std::ptr::null_mut() }
    } else {
        gen_default(&sig.kind)
    }
}

fn gen_default(def: &ElementType) -> TokenStream {
    match def {
        ElementType::Bool => quote! { false },
        ElementType::Char
        | ElementType::I8
        | ElementType::U8
        | ElementType::I16
        | ElementType::U16
        | ElementType::I32
        | ElementType::U32
        | ElementType::I64
        | ElementType::U64
        | ElementType::ISize
        | ElementType::USize => quote! { 0 },
        ElementType::F32 | ElementType::F64 => quote! { 0.0 },
        ElementType::Array((kind, len)) => {
            let default = gen_sig_default(kind);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#default; #len] }
        }
        _ => quote! { ::std::default::Default::default() },
    }
}
