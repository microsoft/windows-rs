#![allow(clippy::many_single_char_names)]

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
            let kind = gen_win32_sig(&signature, gen);
            let value = gen_constant_value(&constant.value());

            quote! {
                pub const #name: #kind = #kind(#value as _);
            }
        }
    } else if let Some(guid) = Guid::from_attributes(def.attributes()) {
        let guid = gen_guid(&guid);
        quote! { pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid); }
    } else if let Some(pkey) = PropertyKey::from_attributes(def.attributes()) {
        let kind = gen_win32_sig(&signature, gen);
        let fmtid = gen_guid(&pkey.fmtid);
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
    let name =  format_token!("{}", def.name());
    let signature = def.signature(&[]);

    let constraints = gen_method_constraints(&signature.params);
    let params = gen_win32_params(&signature.params, gen);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let abi_return_type = if let Some(t) = &signature.return_type {
        // TODO: This should be gen_win32_abi?
        let tokens = gen_win32_sig(t, gen);
        quote! { -> #tokens }
    } else {
        TokenStream::new()
    };

    let args = signature.params.iter().map(|p| gen_win32_abi_arg(p));
    let mut link = def.impl_map().expect("Function").scope().name();

    let raw_dylib = cfg!(feature = "raw_dylib");

    // TODO: remove this whole block once raw-dylib has stabilized as the workarounds
    // will no longer be necessary.
    if !raw_dylib && (link.contains("-ms-win-") || link == "D3DCOMPILER_47" || link == "SspiCli") {
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
        let args = leading_params.iter().map(|p| gen_win32_abi_arg(p));
        let params = gen_win32_params(leading_params, gen);

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
        let args = leading_params.iter().map(|p| gen_win32_abi_arg(p));
        let params = gen_win32_params(leading_params, gen);

        let return_type_tokens = gen_name(&signature.params.last().unwrap().signature.kind, gen);

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
                let return_type = gen_win32_sig(return_type, gen);

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

pub fn gen_field_name(def: &Field) -> TokenStream {
    def.name().into()
}


pub fn gen_type_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_ident, false)
}

pub fn gen_abi_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_abi_ident, false)
}

pub fn gen_turbo_abi_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, to_abi_ident, true)
}

fn format_name<F>(def: &TypeDef, gen: &Gen, format_name: F, turbo: bool) -> TokenStream
where
    F: FnOnce(&str) -> TokenStream,
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

pub fn gen_type_guid(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.generics.is_empty() {
        match Guid::from_attributes(def.attributes()) {
            Some(guid) => {
                let guid = gen_guid(&guid);

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

pub fn gen_type(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    // TODO: all the cloning here is ridiculous
    match def.kind() {
        TypeKind::Interface => {
            if def.is_winrt() {
                Interface(def.clone().with_generics()).gen(gen, include)
            } else {
                ComInterface(def.clone()).gen(gen, include)
            }
        }
        TypeKind::Class => Class(def.clone().with_generics()).gen(gen, include),
        TypeKind::Enum => Enum(def.clone()).gen(gen, include),
        TypeKind::Struct => Struct(def.clone()).gen(gen),
        TypeKind::Delegate => {
            if def.is_winrt() {
                Delegate(def.clone().with_generics()).gen(gen)
            } else {
                Callback(def.clone()).gen(gen)
            }
        }
    }
}

pub fn gen_abi_type(def: &TypeDef, gen: &Gen) -> TokenStream {
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

pub fn gen_signature(def: &TypeDef, signature: &str) -> TokenStream {
    let signature = Literal::byte_string(signature.as_bytes());

    if def.generics.is_empty() {
        return quote! { ::windows::ConstBuffer::from_slice(#signature) };
    }

    let generics = def.generics.iter().enumerate().map(|(index, g)| {
        let g = gen_name(g, &Gen::Absolute);
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

pub fn gen_constant_value_with_type(value: &ConstantValue) -> TokenStream {
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

pub fn gen_constant_value(value: &ConstantValue) -> TokenStream {
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

pub fn gen_default(def: &ElementType) -> TokenStream {
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
            let default = gen_win32_default(kind);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#default; #len] }
        }
        _ => quote! { ::std::default::Default::default() },
    }
}

pub fn gen_name(def: &ElementType, gen: &Gen) -> TokenStream {
    match def {
        ElementType::Void => quote! { ::std::ffi::c_void },
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
            quote! { ::windows::HSTRING }
        }
        ElementType::IInspectable => {
            quote! { ::windows::IInspectable }
        }
        ElementType::Guid => {
            quote! { ::windows::Guid }
        }
        ElementType::IUnknown => {
            quote! { ::windows::IUnknown }
        }
        ElementType::HRESULT => {
            quote! { ::windows::HRESULT }
        }
        ElementType::Array((kind, len)) => {
            // TODO: rename all Signature vars to "sig"
            let name = gen_win32_sig(kind, gen);
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

pub fn gen_abi_type_name(def: &ElementType, gen: &Gen) -> TokenStream {
    match def {
        ElementType::Void => quote! { ::std::ffi::c_void },
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
            quote! { ::windows::RawPtr }
        }
        ElementType::IInspectable => {
            quote! { ::windows::RawPtr }
        }
        ElementType::Guid => {
            quote! { ::windows::Guid }
        }
        ElementType::IUnknown => {
            quote! { ::windows::RawPtr }
        }
        ElementType::HRESULT => {
            quote! { ::windows::HRESULT }
        }
        ElementType::Array((kind, len)) => {
            let name = gen_win32_abi_sig(kind, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        ElementType::GenericParam(generic) => {
            let name =  format_token!("{}", generic);
            quote! { <#name as ::windows::Abi>::Abi }
        }
        ElementType::TypeDef(def) => gen_abi_type(def, gen),
        _ => unimplemented!(),
    }
}

fn gen_namespaces<'a>(
    namespaces: &'a BTreeMap<&'static str, TypeTree>,
) -> impl Iterator<Item = TokenStream> + 'a {
    namespaces.iter().map(move |(name, tree)| {
        if tree.include {
            let name = to_ident(name);

            let tokens = gen_tree(tree);

            quote! {
                // TODO: https://github.com/microsoft/windows-rs/issues/212
                // TODO: https://github.com/microsoft/win32metadata/issues/380
                #[allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
                pub mod #name {
                    #(#tokens)*
                }
            }
         } else {
             TokenStream::new()
         }
    })
}

pub fn gen_tree(tree: &TypeTree) -> impl Iterator<Item = TokenStream> + '_ {
    let gen = Gen::Relative(tree.namespace);

    tree.types
        .iter()
        .map(move |t| gen_type_entry(t.1, &gen))
        .chain(gen_namespaces(&tree.namespaces))
}

pub fn gen_type_entry(entry: &TypeEntry, gen: &Gen) -> TokenStream {
    if entry.include == TypeInclude::None {
        return TokenStream::new();
    }

    match &entry.def {
        ElementType::TypeDef(def) => gen_type(&def.clone().with_generics(), gen, entry.include),
        ElementType::MethodDef(def) => gen_function(def, gen),
        ElementType::Field(def) => gen_field(def, gen),
        _ => unimplemented!(),
    }
}

pub fn gen_win32_sig(sig: &Signature, gen: &Gen) -> TokenStream {
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

pub fn gen_winrt_sig(sig: &Signature, gen: &Gen) -> TokenStream {
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

pub fn gen_win32_abi_sig(sig: &Signature, gen: &Gen) -> TokenStream {
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

pub fn gen_winrt_abi_sig(sig: &Signature, gen: &Gen) -> TokenStream {
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

pub fn gen_win32_default(sig: &Signature) -> TokenStream {
    if sig.pointers > 0 {
        quote! { ::std::ptr::null_mut() }
    } else {
        gen_default(&sig.kind)
    }
}

pub fn gen_winrt_default(sig: &Signature) -> TokenStream {
    if sig.pointers > 0 {
        quote! { ::std::ptr::null_mut() }
    } else {
        gen_default(&sig.kind)
    }
}

pub fn gen_winrt_constraint(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    let params = sig.params.iter().map(|p| gen_winrt_produce_type(p, gen));

    let return_type = if let Some(return_type) = &sig.return_type {
        let tokens = gen_name(&return_type.kind, gen);

        if return_type.is_array {
            quote! { ::windows::Array<#tokens> }
        } else {
            tokens
        }
    } else {
        quote! { () }
    };

    quote! { F: FnMut(#(#params),*) -> ::windows::Result<#return_type> + 'static }
}

pub fn gen_win32_abi(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    // TODO: param insead of p consistency
    let params = sig.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let (udt_return_type, return_type) = if let Some(t) = &sig.return_type {
        if t.is_udt() {
            let mut t = t.clone();
            t.pointers += 1;
            let tokens = gen_win32_abi_sig(&t, gen);
            (quote! { result__: #tokens }, quote! {})
        } else {
            let tokens = gen_win32_abi_sig(t, gen);
            (quote! {}, quote! { -> #tokens })
        }
    } else {
        (TokenStream::new(), TokenStream::new())
    };

    quote! {
        (this: ::windows::RawPtr, #(#params,)* #udt_return_type) #return_type
    }
}

pub fn gen_winrt_abi(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    let params = sig
        .params
        .iter()
        .map(|p| {
            let name = gen_param_name(&p.param);
            let abi = gen_winrt_abi_sig(&p.signature, gen);

            if p.signature.is_array {
                let abi_size_name = gen_param_abi_size_name(&p.param);
                if p.param.is_input() {
                    quote! { #abi_size_name: u32, #name: *const #abi }
                } else if p.signature.by_ref {
                    quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi }
                } else {
                    quote! { #abi_size_name: u32, #name: *mut #abi }
                }
            } else if p.param.is_input() {
                // WinRT only uses const to mean that structs are passed by reference.
                if p.is_const() {
                    quote! { #name: &#abi }
                } else {
                    quote! { #name: #abi }
                }
            } else {
                quote! { #name: *mut #abi }
            }
        })
        .chain(sig.return_type.iter().map(|signature| {
            let abi = gen_winrt_abi_sig(signature, gen);

            if signature.is_array {
                quote! { result_size__: *mut u32, result__: *mut *mut #abi }
            } else {
                quote! { result__: *mut #abi }
            }
        }));

    quote! {
        (this: ::windows::RawPtr, #(#params),*) -> ::windows::HRESULT
    }
}

pub fn gen_win32_invoke_arg(param: &MethodParam, gen: &Gen) -> TokenStream {
    let name = gen_param_name(&param.param);
    let kind = gen_name(&param.signature.kind, gen);

    if param.param.is_input() {
        if param.signature.kind.is_blittable() {
            quote! { #name }
        } else {
            quote! { &*(&#name as *const <#kind as ::windows::Abi>::Abi as *const <#kind as ::windows::Abi>::DefaultType) }
        }
    } else {
        quote! { ::std::mem::transmute_copy(&#name) }
    }
}

pub fn gen_winrt_invoke_arg(param: &MethodParam, gen: &Gen) -> TokenStream {
    let name = gen_param_name(&param.param);
    let kind = gen_name(&param.signature.kind, gen);

    // TODO: This compiles but doesn't property handle delegates with array parameters.
    // https://github.com/microsoft/windows-rs/issues/212

    if param.signature.is_array {
        quote! { ::std::mem::transmute_copy(&#name) }
    } else if param.param.is_input() {
        if param.signature.kind.is_primitive() {
            quote! { #name }
        } else if param.is_const() {
            quote! { &*(#name as *const <#kind as ::windows::Abi>::Abi as *const <#kind as ::windows::Abi>::DefaultType) }
        } else {
            quote! { &*(&#name as *const <#kind as ::windows::Abi>::Abi as *const <#kind as ::windows::Abi>::DefaultType) }
        }
    } else {
        quote! { ::std::mem::transmute_copy(&#name) }
    }
}

pub fn gen_winrt_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    params
        .iter()
        .map(|param| {
            let name = gen_param_name(&param.param);
            let tokens = gen_name(&param.signature.kind, gen);

            if param.signature.is_array {
                if param.param.is_input() {
                    quote! { #name: &[<#tokens as ::windows::Abi>::DefaultType], }
                } else if param.signature.by_ref {
                    quote! { #name: &mut ::windows::Array<#tokens>, }
                } else {
                    quote! { #name: &mut [<#tokens as ::windows::Abi>::DefaultType], }
                }
            } else if param.param.is_input() {
                if param.is_convertible() {
                    let into = gen_name(&param.signature.kind, gen);
                    quote! { #name: impl ::windows::IntoParam<'a, #into>, }
                } else {
                    let mut signature = quote! {};

                    for _ in 0..param.signature.pointers {
                        if param.is_const() {
                            signature.combine(&quote! { *const });
                        } else {
                            signature.combine(&quote! { *mut });
                        }
                    }

                    signature.combine(&tokens);
                    quote! { #name: #signature, }
                }
            } else if param.signature.kind.is_nullable() {
                quote! { #name: &mut ::std::option::Option<#tokens>, }
            } else if let ElementType::GenericParam(_) = param.signature.kind {
                quote! { &mut <#tokens as ::windows::Abi>::DefaultType, }
            } else if param.signature.pointers > 0 {
                let tokens = gen_winrt_abi_sig(&param.signature, gen);
                quote! { #name: #tokens, }
            } else {
                quote! { #name: &mut #tokens, }
            }
        })
        .collect()
}

pub fn gen_winrt_method(
    sig: &MethodSignature,
    method: &MethodInfo,
    interface: &InterfaceInfo,
    gen: &Gen,
) -> TokenStream {
    let params =
        if interface.kind == InterfaceKind::Composable || interface.kind == InterfaceKind::Extend {
            &sig.params[..sig.params.len() - 2]
        } else {
            &sig.params
        };

    let name = gen_method_info_name(sig, method, interface);

    let vtable_offset = Literal::u32_unsuffixed(method.vtable_offset);
    let constraints = gen_method_constraints(params);
    let args = params.iter().map(|p| gen_winrt_abi_arg(p));
    let params = gen_winrt_params(params, gen);
    let interface_name = gen_type_name(&interface.def, gen);

    let return_type_tokens = if let Some(return_type) = &sig.return_type {
        let tokens = gen_name(&return_type.kind, gen);

        if return_type.is_array {
            quote! { ::windows::Array<#tokens> }
        } else {
            tokens
        }
    } else {
        quote! { () }
    };

    let return_arg = if let Some(return_type) = &sig.return_type {
        if return_type.is_array {
            let return_type = gen_name(&return_type.kind, gen);
            quote! { ::windows::Array::<#return_type>::set_abi_len(&mut result__), ::windows::Array::<#return_type>::set_abi(&mut result__) }
        } else {
            quote! { &mut result__ }
        }
    } else {
        quote! {}
    };

    // The ABI obviously still has the two composable parameters. Here we just pass the default in and out
    // arguments to ensure the call succeeds in the non-aggregating case.
    let composable_args = match interface.kind {
        InterfaceKind::Composable => quote! {
            ::std::ptr::null_mut(), ::windows::Abi::set_abi(&mut ::std::option::Option::<::windows::IInspectable>::None),
        },
        InterfaceKind::Extend => quote! {
            ::windows::Abi::abi(&derived__), ::windows::Abi::set_abi(base__),
        },
        _ => quote! {},
    };

    let vcall = if let Some(return_type) = &sig.return_type {
        if return_type.is_array {
            quote! {
                let mut result__: #return_type_tokens = ::std::mem::zeroed();
                (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args,)* #composable_args #return_arg)
                    .and_then(|| result__ )
            }
        } else {
            quote! {
                let mut result__: <#return_type_tokens as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args,)* #composable_args #return_arg)
                        .from_abi::<#return_type_tokens>(result__ )
            }
        }
    } else {
        quote! {
            (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args,)* #composable_args).ok()
        }
    };

    let deprecated = if method.is_deprecated {
        quote! { #[cfg(feature = "deprecated")] }
    } else {
        quote! {}
    };

    match interface.kind {
        InterfaceKind::Default => quote! {
            #deprecated
            pub fn #name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                let this = self;
                unsafe {
                    #vcall
                }
            }
        },
        InterfaceKind::NonDefault | InterfaceKind::Overridable => {
            quote! {
                #deprecated
                pub fn #name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                    let this = &::windows::Interface::cast::<#interface_name>(self).unwrap();
                    unsafe {
                        #vcall
                    }
                }
            }
        }
        InterfaceKind::Static | InterfaceKind::Composable => {
            quote! {
                #deprecated
                pub fn #name<#constraints>(#params) -> ::windows::Result<#return_type_tokens> {
                    Self::#interface_name(|this| unsafe { #vcall })
                }
            }
        }
        InterfaceKind::Extend => {
            let interface_name = to_ident(interface.def.name());
            quote! {
                pub fn #name<#constraints>(self, #params) -> ::windows::Result<#return_type_tokens> {
                    unsafe {
                        let (derived__, base__) = ::windows::Compose::compose(self);
                        #return_type_tokens::#interface_name(|this| unsafe { #vcall })
                    }
                }
            }
        }
    }
}

fn gen_method_info_name(
    sig: &MethodSignature,
    method: &MethodInfo,
    interface: &InterfaceInfo,
) -> TokenStream {
    if (interface.kind == InterfaceKind::Composable || interface.kind == InterfaceKind::Extend)
        && sig.params.len() == 2
    {
        "new".into()
    } else if method.overload > 1 {
        format!("{}{}", &method.name, method.overload).into()
    } else {
        method.name.clone().into()
    }
}

pub fn gen_method_constraints(params: &[MethodParam]) -> TokenStream {
    if params.iter().any(|param| param.is_convertible()) {
        quote! { 'a, }
    } else {
        quote! {}
    }
}

pub fn gen_winrt_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.param);

    if param.signature.is_array {
        if param.param.is_input() {
            quote! { #name.len() as u32, ::std::mem::transmute(#name.as_ptr()) }
        } else if param.signature.by_ref {
            quote! { #name.set_abi_len(), #name.set_abi() }
        } else {
            quote! { #name.len() as u32, ::std::mem::transmute_copy(&#name) }
        }
    } else if param.param.is_input() {
        if param.is_convertible() {
            if param.is_const() {
                quote! { &#name.into_param().abi() }
            } else {
                quote! { #name.into_param().abi() }
            }
        } else if param.signature.kind.is_blittable() {
            quote! { #name }
        } else if param.signature.pointers == 0 {
            quote! { ::windows::Abi::abi(#name) }
        } else {
            quote! { ::std::mem::transmute(#name) }
        }
    } else if param.signature.kind.is_blittable()
        || (param.signature.pointers > 0 && !param.signature.kind.is_nullable())
    {
        quote! { #name }
    } else {
        quote! { ::windows::Abi::set_abi(#name) }
    }
}

pub fn gen_win32_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();
    let is_const = param.is_const();

    for _ in 0..param.signature.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    let kind = gen_name(&param.signature.kind, gen);

    if param.signature.kind.is_nullable() {
        tokens.combine(&quote! {
            ::std::option::Option<#kind>
        });
    } else {
        tokens.combine(&kind)
    }

    tokens
}

pub fn gen_win32_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    params
        .iter()
        .map(|param| {
            let name = gen_param_name(&param.param);

            if param.is_convertible() {
                let into = gen_name(&param.signature.kind, gen);
                quote! { #name: impl ::windows::IntoParam<'a, #into>, }
            } else {
                let tokens = gen_win32_param(param, gen);
                quote! { #name: #tokens, }
            }
        })
        .collect()
}

pub fn gen_win32_abi_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for _ in 0..param.signature.pointers {
        if param.is_const() {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    tokens.combine(&gen_abi_type_name(&param.signature.kind, gen));
    tokens
}

pub fn gen_win32_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.param);

    if param.is_convertible() {
        quote! { #name.into_param().abi() }
    } else {
        quote! { ::std::mem::transmute(#name) }
    }
}

pub fn gen_winrt_produce_type(param: &MethodParam, gen: &Gen) -> TokenStream {
    let tokens = gen_name(&param.signature.kind, gen);

    if param.signature.is_array {
        if param.param.is_input() {
            quote! { &[#tokens] }
        } else if param.signature.by_ref {
            quote! { &mut ::windows::Array<#tokens> }
        } else {
            quote! { &mut [#tokens] }
        }
    } else if param.param.is_input() {
        if let ElementType::GenericParam(_) = param.signature.kind {
            quote! { &<#tokens as ::windows::Abi>::DefaultType }
        } else if param.signature.kind.is_primitive() {
            quote! { #tokens }
        } else if param.signature.kind.is_nullable() {
            quote! { &::std::option::Option<#tokens> }
        } else {
            quote! { &#tokens }
        }
    } else {
        quote! { &mut #tokens }
    }
}

pub fn gen_win32_upcall(sig: &MethodSignature, inner: TokenStream, gen: &Gen) -> TokenStream {
    if sig.has_query_interface() {
        quote! {
            unimplemented!("one")
        }
    } else if sig.has_retval() {
        let invoke_args = sig.params[..sig.params.len() - 1]
            .iter()
            .map(|param| gen_win32_invoke_arg(param, gen));

        let result = gen_param_name(&sig.params[sig.params.len() - 1].param);

        quote! {
            match #inner(#(#invoke_args,)*) {
                ::std::result::Result::Ok(ok__) => {
                    *#result = ::std::mem::transmute_copy(&ok__);
                    ::std::mem::forget(ok__);
                    ::windows::HRESULT(0)
                }
                ::std::result::Result::Err(err) => err.into()
            }
        }
    } else if sig.has_udt_return() {
        quote! {
            unimplemented!("three")
        }
    } else if let Some(return_type) = &sig.return_type {
        if return_type.kind == ElementType::HRESULT {
            let invoke_args = sig
                .params
                .iter()
                .map(|param| gen_win32_invoke_arg(param, gen));

            quote! {
                #inner(#(#invoke_args,)*).into()
            }
        } else {
            quote! {
                unimplemented!("five")
            }
        }
    } else {
        quote! {
            unimplemented!("six")
        }
    }
}

pub fn gen_winrt_upcall(sig: &MethodSignature, inner: TokenStream, gen: &Gen) -> TokenStream {
    let invoke_args = sig
        .params
        .iter()
        .map(|param| gen_winrt_invoke_arg(param, gen));

    match &sig.return_type {
        Some(return_type) if return_type.is_array => {
            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::std::result::Result::Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        *result__ = ok_data__;
                        *result_size__ = ok_data_len__;
                        ::windows::HRESULT(0)
                    }
                    ::std::result::Result::Err(err) => err.into()
                }
            }
        }
        Some(_) => {
            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::std::result::Result::Ok(ok__) => {
                        *result__ = ::std::mem::transmute_copy(&ok__);
                        ::std::mem::forget(ok__);
                        ::windows::HRESULT(0)
                    }
                    ::std::result::Result::Err(err) => err.into()
                }
            }
        }
        None => quote! {
            #inner(#(#invoke_args,)*).into()
        },
    }
}

pub fn gen_param_name(param: &Param) -> TokenStream {
    to_ident(&param.name().to_lowercase())
}

pub fn gen_param_abi_size_name(param: &Param) -> TokenStream {
    to_ident(&format!("{}_array_size", param.name()))
}

pub fn gen_guid(guid: &Guid) -> TokenStream {
    let a = Literal::u32_unsuffixed(guid.0);
    let b = Literal::u16_unsuffixed(guid.1);
    let c = Literal::u16_unsuffixed(guid.2);
    let d = Literal::u8_unsuffixed(guid.3);
    let e = Literal::u8_unsuffixed(guid.4);
    let f = Literal::u8_unsuffixed(guid.5);
    let g = Literal::u8_unsuffixed(guid.6);
    let h = Literal::u8_unsuffixed(guid.7);
    let i = Literal::u8_unsuffixed(guid.8);
    let j = Literal::u8_unsuffixed(guid.9);
    let k = Literal::u8_unsuffixed(guid.10);

    quote! {
        #a, #b, #c, [#d, #e, #f, #g, #h, #i, #j, #k],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        assert_eq!(
            gen_name(&ElementType::Bool, &Gen::Absolute).as_str(),
            "bool"
        );
    }

    fn get_method(interface: &types::Interface, method: &str) -> MethodDef {
        interface.0.methods().find(|m| m.name() == method).unwrap()
    }

    #[test]
    fn test_method() {
        let i =
            TypeReader::get().expect_type_def(TypeName::new("Windows.Foundation", "IStringable"));
        let i = Interface(i);
        let m = get_method(&i, "ToString");
        assert_eq!(m.name(), "ToString");

        let s = m.signature(&[]);
        assert_eq!(s.params.len(), 0);

        let s = s.return_type.unwrap();
        assert!(s.kind == ElementType::String);
        assert_eq!(s.pointers, 0);
        assert!(!s.by_ref);
        assert!(!s.is_const);
        assert!(!s.is_array);
    }

    #[test]
    fn test_generic() {
        let i = TypeReader::get()
            .expect_type_def(TypeName::new("Windows.Foundation.Collections", "IMap`2"));
        let i = Interface(i.with_generics());
        let m = get_method(&i, "Lookup");

        let s = m.signature(&i.0.generics);
        assert_eq!(s.params.len(), 1);

        let r = s.return_type.unwrap();
        assert_eq!(gen_name(&r.kind, &Gen::Absolute).as_str(), "V");
        assert_eq!(r.pointers, 0);
        assert!(!r.by_ref);
        assert!(!r.is_const);
        assert!(!r.is_array);

        let p = &s.params[0];
        assert_eq!(p.param.name(), "key");
        assert_eq!(gen_name(&p.signature.kind, &Gen::Absolute).as_str(), "K");
        assert_eq!(p.signature.pointers, 0);
        assert!(!p.signature.by_ref);
        assert!(!p.signature.is_const);
        assert!(!p.signature.is_array);
    }
}
