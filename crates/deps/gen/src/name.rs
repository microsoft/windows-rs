use super::*;

pub fn gen_type_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_ident, false)
}

pub fn gen_abi_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_abi_ident, false)
}

pub fn gen_turbo_abi_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_abi_ident, true)
}

fn to_abi_ident(name: &str) -> TokenStream {
    format_token!("{}_abi", name)
}

pub fn gen_name(def: &ElementType, gen: &Gen) -> TokenStream {
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
            quote! { ::windows::core::HSTRING }
        }
        ElementType::IInspectable => {
            quote! { ::windows::core::IInspectable }
        }
        ElementType::GUID => {
            quote! { ::windows::core::GUID }
        }
        ElementType::IUnknown => {
            quote! { ::windows::core::IUnknown }
        }
        ElementType::HRESULT => {
            quote! { ::windows::core::HRESULT }
        }
        ElementType::Array((kind, len)) => {
            // TODO: rename all Signature vars to "sig"
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

// TODO: only return windows_sys types
pub fn gen_abi_type_name(def: &ElementType, gen: &Gen) -> TokenStream {
    match def {
        ElementType::String => {
            quote! { ::core::mem::ManuallyDrop<::windows::core::HSTRING> }
        }
        ElementType::IUnknown | ElementType::IInspectable => {
            quote! { ::windows::core::RawPtr }
        }
        ElementType::Array((kind, len)) => {
            let name = gen_abi_sig(kind, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        ElementType::GenericParam(generic) => {
            let name = format_token!("{}", generic);
            quote! { <#name as ::windows::core::Abi>::Abi }
        }
        ElementType::TypeDef(def) => gen_abi_type(def, gen),
        _ => gen_name(def, gen),
    }
}

pub fn gen_param_name(param: &Param) -> TokenStream {
    to_ident(&param.name().to_lowercase())
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

        if def.generics.is_empty() {
            namespace.combine(&name);
            namespace
        } else {
            let colon_separated = if turbo || !namespace.as_str().is_empty() {
                quote! { :: }
            } else {
                quote! {}
            };

            let generics = def.generics.iter().map(|g| gen_name(g, gen));
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

fn gen_abi_type(def: &TypeDef, gen: &Gen) -> TokenStream {
    match def.kind() {
        TypeKind::Enum => gen_type_name(def, gen),
        TypeKind::Struct => {
            let tokens = gen_type_name(def, gen);
            if def.is_blittable() {
                tokens
            } else {
                quote! { ::core::mem::ManuallyDrop<#tokens> }
            }
        }
        _ => quote! { ::windows::core::RawPtr },
    }
}
