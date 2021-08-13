// TODO: sort these out

use crate::*;

pub fn gen_field(def: &Field, gen: &Gen) -> TokenStream {
    let name = def.name();
    let name = to_ident(name);
    let signature = def.signature();

    if let Some(constant) = def.constant() {
        if signature.kind == constant.value_type() {
            let value = gen_constant_value_with_type(&constant.value());

            quote! {
                pub const #name: #value;
            }
        } else {
            let kind = signature.gen_win32(gen);
            let value = gen_constant_value(&constant.value());

            quote! {
                pub const #name: #kind = #kind(#value as _);
            }
        }
    } else if let Some(guid) = Guid::from_attributes(def.attributes()) {
        let guid = guid.gen();
        quote! { pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid); }
    } else if let Some(pkey) = PropertyKey::from_attributes(def.attributes()) {
        let kind = signature.gen_win32(gen);
        let fmtid = pkey.fmtid.gen();
        let pid = pkey.pid;
        quote! {
            pub const #name: #kind = #kind {
                fmtid: ::windows::Guid::from_values(#fmtid),
                pid: #pid,
            };
        }
    } else {
        quote! {}
    }
}

pub fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_method_name(def, gen); // TODO: this probably doesn't need gen
    let signature = def.signature(&[]);

    let constraints = signature.gen_constraints(&signature.params);
    let params = signature.gen_win32_params(&signature.params, gen);

    let abi_params = signature.params.iter().map(|p| {
        let name = p.param.gen_name();
        let tokens = p.gen_win32_abi_param(gen);
        quote! { #name: #tokens }
    });

    let abi_return_type = if let Some(t) = &signature.return_type {
        // TODO: This should be gen_win32_abi?
        let tokens = t.gen_win32(gen);
        quote! { -> #tokens }
    } else {
        TokenStream::new()
    };

    let args = signature.params.iter().map(|p| p.gen_win32_abi_arg());
    let mut link = def.impl_map().expect("Function").scope().name();

    let raw_dylib = cfg!(feature = "raw_dylib");

    // TODO: remove this whole block once raw-dylib has stabilized as the workarounds
    // will no longer be necessary.
    if !raw_dylib
        && (link.contains("-ms-win-") || link == "D3DCOMPILER_47" || link == "SspiCli")
    {
        link = "onecoreuap";
    }

    let static_lib = def
        .attributes()
        .filter_map(|attribute| match attribute.name() {
            "StaticLibraryAttribute" => Some(gen_constant_value(&attribute.args()[0].1)),
            _ => None,
        })
        .next();

    let link_attr = match static_lib {
        Some(link) => quote! { #[link(name = #link, kind = "static")] },
        None => {
            // TODO: switch to always using raw-dylib once it has stabilized
            if raw_dylib {
                quote! { #[link(name = #link, kind="raw-dylib")] }
            } else {
                quote! { #[link(name = #link)] }
            }
        }
    };

    if signature.has_query_interface() {
        let leading_params = &signature.params[..signature.params.len() - 2];
        let args = leading_params.iter().map(|p| p.gen_win32_abi_arg());
        let params = signature.gen_win32_params(leading_params, gen);

        quote! {
            pub unsafe fn #name<#constraints T: ::windows::Interface>(#params) -> ::windows::Result<T> {
                #[cfg(windows)]
                {
                    #link_attr
                    extern "system" {
                        fn #name(#(#abi_params),*) #abi_return_type;
                    }
                    let mut result__ = ::std::option::Option::None;
                    #name(#(#args,)* &<T as ::windows::Interface>::IID, ::windows::Abi::set_abi(&mut result__)).and_some(result__)
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
        }
    } else if signature.has_retval() {
        let leading_params = &signature.params[..signature.params.len() - 1];
        let args = leading_params.iter().map(|p| p.gen_win32_abi_arg());
        let params = signature.gen_win32_params(leading_params, gen);

        let return_type_tokens = signature
            .params
            .last()
            .unwrap()
            .signature
            .kind
            .gen_name(gen);

        quote! {
            pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<#return_type_tokens> {
                #[cfg(windows)]
                {
                    #link_attr
                    extern "system" {
                        fn #name(#(#abi_params),*) #abi_return_type;
                    }
                    let mut result__: <#return_type_tokens as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    #name(#(#args,)* &mut result__).from_abi::<#return_type_tokens>(result__)
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
        }
    } else if let Some(return_type) = &signature.return_type {
        match &return_type.kind {
            ElementType::HRESULT => {
                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<()> {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) -> ::windows::HRESULT;
                            }
                            #name(#(#args),*).ok()
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
            ElementType::TypeDef(def) if def.type_name() == TypeName::NTSTATUS => {
                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<()> {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) #abi_return_type;
                            }
                            #name(#(#args),*).ok()
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
            _ => {
                let return_type = return_type.gen_win32(gen);

                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> #return_type {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) #abi_return_type;
                            }
                            #name(#(#args),*)
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
        }
    } else {
        quote! {
            pub unsafe fn #name<#constraints>(#params) {
                #[cfg(windows)]
                {
                    #link_attr
                    extern "system" {
                        fn #name(#(#abi_params),*);
                    }
                    #name(#(#args),*)
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
        }
    }
}

pub fn gen_field_name(def:&Field) -> TokenStream {
    // TODO: This should simply return: def.name().to_string()
    let name = format_ident!("{}", def.name());
    quote! { #name }
}

pub fn gen_method_name(def:&MethodDef, gen: &Gen) -> TokenStream {
    let namespace = gen.namespace(def.parent().namespace());
    let name = format_ident!("{}", def.name());
    quote! { #namespace #name }
}

pub fn gen_type_name(def:&TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_ident, false)
}

pub fn gen_abi_name(def:&TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_abi_ident, false)
}

pub fn gen_turbo_abi_name(def:&TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_abi_ident, true)
}

fn format_name<F>(def:&TypeDef, gen: &Gen, format_name: F, turbo: bool) -> TokenStream
where
    F: FnOnce(&str) -> Ident,
{
    let type_name = def.type_name();

    if type_name.namespace.is_empty() {
        let name = format_name(&scoped_name(def));
        quote! { #name }
    } else {
        let namespace = gen.namespace(type_name.namespace);
        let name = format_name(type_name.name);

        if def.generics.is_empty() {
            quote! { #namespace#name }
        } else {
            let colon_separated = if turbo || !namespace.as_str().is_empty() {
                quote! { :: }
            } else {
                quote! {}
            };

            let generics = def.generics.iter().map(|g| g.gen_name(gen));
            quote! { #namespace#name#colon_separated<#(#generics),*> }
        }
    }
}

fn scoped_name(def:&TypeDef) -> String {
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

pub fn gen_guid(def:&TypeDef, gen: &Gen) -> TokenStream {
    if def.generics.is_empty() {
        match Guid::from_attributes(def.attributes()) {
            Some(guid) => {
                let guid = guid.gen();

                quote! {
                    ::windows::Guid::from_values(#guid)
                }
            }
            None => {
                quote! {
                    ::windows::Guid::zeroed()
                }
            }
        }
    } else {
        let tokens = gen_type_name(def, gen);

        quote! {
            ::windows::Guid::from_signature(<#tokens as ::windows::RuntimeType>::SIGNATURE)
        }
    }
}

pub fn gen_type(def:&TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    // TODO: all the cloning here is ridiculous
    match def.kind() {
        TypeKind::Interface => {
            if def.is_winrt() {
                types::Interface(def.clone().with_generics()).gen(gen, include)
            } else {
                types::ComInterface(def.clone()).gen(gen, include)
            }
        }
        TypeKind::Class => types::Class(def.clone().with_generics()).gen(gen, include),
        TypeKind::Enum => types::Enum(def.clone()).gen(gen, include),
        TypeKind::Struct => types::Struct(def.clone()).gen(gen),
        TypeKind::Delegate => {
            if def.is_winrt() {
                types::Delegate(def.clone().with_generics()).gen(gen)
            } else {
                types::Callback(def.clone()).gen(gen)
            }
        }
    }
}

pub fn gen_abi_type(def:&TypeDef, gen: &Gen) -> TokenStream {
    match def.kind() {
        TypeKind::Enum => gen_type_name(def, gen),
        TypeKind::Struct => {
            if def.is_blittable() {
                gen_type_name(def, gen)
            } else {
                gen_abi_name(def, gen)
            }
        }
        _ => quote! { ::windows::RawPtr },
    }
}

pub fn gen_signature(def:&TypeDef, signature: &str) -> TokenStream {
    let signature = Literal::byte_string(signature.as_bytes());

    if def.generics.is_empty() {
        return quote! { ::windows::ConstBuffer::from_slice(#signature) };
    }

    let generics = def.generics.iter().enumerate().map(|(index, g)| {
        let g = g.gen_name(&Gen::Absolute);
        let semi = if index != def.generics.len() - 1 {
            Some(quote! {
                .push_slice(b";")
            })
        } else {
            None
        };

        quote! {
            .push_other(<#g as ::windows::RuntimeType>::SIGNATURE)
            #semi
        }
    });

    quote! {
        {
            ::windows::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(#signature)
            .push_slice(b";")
            #(#generics)*
            .push_slice(b")")
        }
    }
}

pub fn gen_phantoms(def:&TypeDef) -> impl Iterator<Item = TokenStream> + '_ {
    def.generics.iter().map(move |g| {
        let g = g.gen_name(&Gen::Absolute);
        quote! { ::std::marker::PhantomData::<#g> }
    })
}

pub fn gen_constraints(def:&TypeDef) -> TokenStream {
    def.generics
        .iter()
        .map(|g| {
            let g = g.gen_name(&Gen::Absolute);
            quote! { #g: ::windows::RuntimeType + 'static, }
        })
        .collect()
}

pub fn gen_constant_value_with_type(value:&ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { bool = #value },
        ConstantValue::U8(value) => quote! { u8 = #value },
        ConstantValue::I8(value) => quote! { i8 = #value },
        ConstantValue::U16(value) => quote! { u16 = #value },
        ConstantValue::I16(value) => quote! { i16 = #value },
        ConstantValue::U32(value) => quote! { u32 = #value },
        ConstantValue::I32(value) => quote! { i32 = #value },
        ConstantValue::U64(value) => quote! { u64 = #value },
        ConstantValue::I64(value) => quote! { i64 = #value },
        ConstantValue::F32(value) => quote! { f32 = #value },
        ConstantValue::F64(value) => quote! { f64 = #value },
        ConstantValue::String(value) => quote! { &'static str = #value },
        _ => unimplemented!(),
    }
}

pub fn gen_constant_value(value:&ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { #value },
        ConstantValue::U8(value) => quote! { #value },
        ConstantValue::I8(value) => quote! { #value },
        ConstantValue::U16(value) => quote! { #value },
        ConstantValue::I16(value) => quote! { #value },
        ConstantValue::U32(value) => quote! { #value },
        ConstantValue::I32(value) => quote! { #value },
        ConstantValue::U64(value) => quote! { #value },
        ConstantValue::I64(value) => quote! { #value },
        ConstantValue::F32(value) => quote! { #value },
        ConstantValue::F64(value) => quote! { #value },
        ConstantValue::String(value) => quote! { #value },
        _ => unimplemented!(),
    }
}