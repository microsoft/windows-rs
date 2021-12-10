use super::*;

pub fn gen_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" | "try" | "async" | "await" | "dyn" => format!("r#{}", name).into(),
        "Self" | "self" => format!("{}_", name).into(),
        "_" => "unused".into(),
        _ => name.into(),
    }
}

pub fn gen_type_ident2(def: &TypeDef) -> TokenStream {
    let mut name = gen_ident(def.name());

    if def.generics.is_empty() {
        name
    } else {
        name.0.truncate(name.0.len() - 2);
        name
    }
}

// TODO: use above instead
pub fn gen_type_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "")
}

// TODO: use above instead
pub fn gen_vtbl_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "Vtbl")
}

fn gen_type_ident_impl(def: &TypeDef, gen: &Gen, vtbl: &str) -> TokenStream {
    let mut name = gen_ident(def.name());

    if def.generics.is_empty() {
        name.push_str(vtbl);
        name
    } else {
        name.0.truncate(name.0.len() - 2);
        name.push_str(vtbl);

        if gen.sys {
            name
        } else {
            name.push('<');

            for g in &def.generics {
                name.push_str(gen_element_name(g, gen).as_str());
                name.push(',');
            }

            name.push('>');
            name
        }
    }
}

pub fn gen_phantoms(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { ::core::marker::PhantomData::<#name>, }
        })
        .collect()
}

pub fn gen_type_generics(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { #name, }
        })
        .collect()
}

pub fn gen_type_constraints(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { #name: ::windows::core::RuntimeType + 'static, }
        })
        .collect()
}

pub fn gen_guid_signature(def: &TypeDef, signature: &str, gen: &Gen) -> TokenStream {
    let signature = Literal::byte_string(signature.as_bytes());

    if def.generics.is_empty() {
        return quote! { ::windows::core::ConstBuffer::from_slice(#signature) };
    }

    let generics = def.generics.iter().enumerate().map(|(index, g)| {
        let g = gen_element_name(g, gen);
        let semi = if index != def.generics.len() - 1 {
            Some(quote! {
                .push_slice(b";")
            })
        } else {
            None
        };

        quote! {
            .push_other(<#g as ::windows::core::RuntimeType>::SIGNATURE)
            #semi
        }
    });

    quote! {
        {
            ::windows::core::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(#signature)
            .push_slice(b";")
            #(#generics)*
            .push_slice(b")")
        }
    }
}

pub fn gen_param_name(param: &Param) -> TokenStream {
    gen_ident(&param.name().to_lowercase())
}

pub fn gen_element_name(def: &ElementType, gen: &Gen) -> TokenStream {
    match def {
        ElementType::Void => quote! { ::core::ffi::c_void },
        ElementType::Bool => quote! { bool },
        ElementType::Char => quote! { u16 },
        ElementType::I8 => quote! { i8 },
        ElementType::U8 => quote! { u8 },
        ElementType::I16 => quote! { i16 },
        ElementType::U16 => quote! { u16 },
        ElementType::I32 => quote! { i32 },
        ElementType::U32 => quote! { u32 },
        ElementType::I64 => quote! { i64 },
        ElementType::U64 => quote! { u64 },
        ElementType::F32 => quote! { f32 },
        ElementType::F64 => quote! { f64 },
        ElementType::ISize => quote! { isize },
        ElementType::USize => quote! { usize },
        ElementType::String => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::HSTRING }
        }
        ElementType::IInspectable => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::IInspectable }
        }
        ElementType::GUID => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::GUID }
        }
        ElementType::IUnknown => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::IUnknown }
        }
        ElementType::HRESULT => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::HRESULT }
        }
        ElementType::Array((kind, len)) => {
            let name = gen_sig(kind, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        ElementType::GenericParam(generic) => generic.into(),
        ElementType::MethodDef(def) => def.name().into(),
        ElementType::Field(field) => field.name().into(),
        ElementType::TypeDef(t) => gen_type_name(t, gen),
        _ => unimplemented!(),
    }
}

pub fn gen_abi_element_name(sig: &Signature, gen: &Gen) -> TokenStream {
    match &sig.kind {
        ElementType::String => {
            quote! { ::core::mem::ManuallyDrop<::windows::core::HSTRING> }
        }
        ElementType::IUnknown | ElementType::IInspectable => {
            quote! { *mut ::core::ffi::c_void }
        }
        ElementType::Array((kind, len)) => {
            let name = gen_abi_sig(kind, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        ElementType::GenericParam(generic) => {
            let name = gen_ident(generic);
            quote! { <#name as ::windows::core::Abi>::Abi }
        }
        ElementType::TypeDef(def) => match def.kind() {
            TypeKind::Enum => gen_type_name(def, gen),
            TypeKind::Struct => {
                let tokens = gen_type_name(def, gen);
                if def.is_blittable() || sig.pointers > 0 {
                    tokens
                } else {
                    quote! { ::core::mem::ManuallyDrop<#tokens> }
                }
            }
            _ => quote! { ::windows::core::RawPtr },
        },
        _ => gen_element_name(&sig.kind, gen),
    }
}

pub fn gen_crate_name(gen: &Gen) -> TokenStream {
    if gen.sys {
        "windows_sys".into()
    } else {
        "windows".into()
    }
}

pub fn gen_type_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, gen_ident, false)
}

fn format_name<F>(def: &TypeDef, gen: &Gen, format_name: F, turbo: bool) -> TokenStream
where
    F: FnOnce(&str) -> TokenStream,
{
    let type_name = def.type_name();

    if type_name.namespace.is_empty() {
        format_name(&scoped_name(def))
    } else {
        let mut namespace = gen.namespace(type_name.namespace);
        let name = format_name(type_name.name);

        if def.generics.is_empty() || gen.sys {
            namespace.combine(&name);
            namespace
        } else {
            let colon_separated = if turbo || !namespace.as_str().is_empty() {
                quote! { :: }
            } else {
                quote! {}
            };

            let generics = def.generics.iter().map(|g| gen_element_name(g, gen));
            quote! { #namespace#name#colon_separated<#(#generics),*> }
        }
    }
}

fn scoped_name(def: &TypeDef) -> String {
    if let Some(enclosing_type) = def.enclosing_type() {
        if let Some(nested_types) = enclosing_type.nested_types() {
            for (index, (nested_type, _)) in nested_types.iter().enumerate() {
                if *nested_type == def.name() {
                    return format!("{}_{}", &scoped_name(&enclosing_type), index);
                }
            }
        }
    }

    def.name().to_string()
}
