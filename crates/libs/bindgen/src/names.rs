use super::*;

pub fn gen_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" | "try" | "async" | "await" | "dyn" => format!("r#{}", name).into(),
        "Self" | "self" => format!("{}_", name).into(),
        "_" => "unused".into(),
        _ => trim_tick(name).into(),
    }
}

// TODO: use above instead
pub fn gen_type_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "")
}

// TODO: use above instead
pub fn gen_vtbl_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "_Vtbl")
}

// TODO: use above instead
pub fn gen_impl_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "_Impl")
}

fn gen_type_ident_impl(def: &TypeDef, gen: &Gen, vtbl: &str) -> TokenStream {
    let mut name = gen_ident(def.name());
    name.push_str(vtbl);

    if gen.sys || def.generics.is_empty() {
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

pub fn gen_phantoms(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { ::core::marker::PhantomData::<#name>, }
        })
        .collect()
}

pub fn gen_named_phantoms(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { #name: ::core::marker::PhantomData::<#name>, }
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

pub fn gen_element_name(def: &Type, gen: &Gen) -> TokenStream {
    match def {
        Type::Void => quote! { ::core::ffi::c_void },
        Type::Bool => quote! { bool },
        Type::Char => quote! { u16 },
        Type::I8 => quote! { i8 },
        Type::U8 => quote! { u8 },
        Type::I16 => quote! { i16 },
        Type::U16 => quote! { u16 },
        Type::I32 => quote! { i32 },
        Type::U32 => quote! { u32 },
        Type::I64 => quote! { i64 },
        Type::U64 => quote! { u64 },
        Type::F32 => quote! { f32 },
        Type::F64 => quote! { f64 },
        Type::ISize => quote! { isize },
        Type::USize => quote! { usize },
        Type::String => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::HSTRING }
        }
        Type::IInspectable => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::IInspectable }
        }
        Type::GUID => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::GUID }
        }
        Type::IUnknown => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::IUnknown }
        }
        Type::HRESULT => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::HRESULT }
        }
        Type::Win32Array((kind, len)) => {
            let name = gen_sig(kind, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        Type::GenericParam(generic) => generic.into(),
        Type::MethodDef(def) => def.name().into(),
        Type::Field(field) => field.name().into(),
        Type::TypeDef(t) => gen_type_name(t, gen),
        Type::MutPtr((kind, pointers)) => {
            let pointers = gen_mut_ptrs(*pointers);
            let kind = gen_default_type(kind, gen);
            quote! { #pointers #kind }
        }
        Type::ConstPtr((kind, pointers)) => {
            let pointers = gen_const_ptrs(*pointers);
            let kind = gen_default_type(kind, gen);
            quote! { #pointers #kind }
        }
        Type::WinrtArray(kind) => gen_element_name(kind, gen),
        Type::WinrtArrayRef(kind) => gen_element_name(kind, gen),
        Type::WinrtConstRef(kind) => gen_element_name(kind, gen),
        Type::TypeName => unimplemented!(),
    }
}

fn gen_mut_ptrs(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).into()
}

fn gen_const_ptrs(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).into()
}

pub fn gen_abi_element_name(sig: &Type, gen: &Gen) -> TokenStream {
    gen_abi_element_name_impl(sig, false, gen)
}

// TODO: this is only because we're trying to avoid the ManuallyDrop below - I don't think that matters so may want to scrap this once we have parity.
fn gen_abi_element_name_impl(sig: &Type, ptr: bool, gen: &Gen) -> TokenStream {
    match sig {
        Type::String => {
            quote! { ::core::mem::ManuallyDrop<::windows::core::HSTRING> }
        }
        Type::IUnknown | Type::IInspectable => {
            quote! { *mut ::core::ffi::c_void }
        }
        Type::Win32Array((kind, len)) => {
            let name = gen_abi_element_name_impl(kind, ptr, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        Type::GenericParam(generic) => {
            let name = gen_ident(generic);
            quote! { <#name as ::windows::core::Abi>::Abi }
        }
        Type::TypeDef(def) => match def.kind() {
            TypeKind::Enum => gen_type_name(def, gen),
            TypeKind::Struct => {
                let tokens = gen_type_name(def, gen);
                if def.is_blittable() || ptr {
                    tokens
                } else {
                    quote! { ::core::mem::ManuallyDrop<#tokens> }
                }
            }
            _ => quote! { ::windows::core::RawPtr },
        },
        Type::MutPtr((kind, pointers)) => {
            let pointers = gen_mut_ptrs(*pointers);
            let kind = gen_abi_element_name_impl(kind, true, gen);
            quote! { #pointers #kind }
        }
        Type::ConstPtr((kind, pointers)) => {
            let pointers = gen_const_ptrs(*pointers);
            let kind = gen_abi_element_name_impl(kind, true, gen);
            quote! { #pointers #kind }
        }
        Type::WinrtArray(kind) => gen_abi_element_name_impl(kind, ptr, gen),
        Type::WinrtArrayRef(kind) => gen_abi_element_name_impl(kind, ptr, gen),
        _ => gen_element_name(sig, gen),
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
