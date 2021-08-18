#![allow(clippy::many_single_char_names)]

// TODO: sort these out

use crate::*;

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

pub fn gen_type(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    match def.kind() {
        TypeKind::Interface => {
            if def.is_winrt() {
                gen_interface(&def.clone().with_generics(), gen, include)
            } else {
                gen_com_interface(def, gen, include)
            }
        }
        TypeKind::Class => Class(def.clone().with_generics()).gen(gen, include),
        TypeKind::Enum => gen_enum(def, gen, include),
        TypeKind::Struct => gen_struct(def, gen),
        TypeKind::Delegate => {
            if def.is_winrt() {
                gen_delegate(def, gen)
            } else {
                gen_callback(def, gen)
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
            let default = gen_sig_default(kind);
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
            let name = gen_abi_sig(kind, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        ElementType::GenericParam(generic) => {
            let name = format_token!("{}", generic);
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
        ElementType::Field(def) => gen_constant(def, gen),
        _ => unimplemented!(),
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
            let tokens = gen_abi_sig(&t, gen);
            (quote! { result__: #tokens }, quote! {})
        } else {
            let tokens = gen_abi_sig(t, gen);
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
            let abi = gen_abi_sig(&p.signature, gen);

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
            let abi = gen_abi_sig(signature, gen);

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
                let tokens = gen_abi_sig(&param.signature, gen);
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
        format_token!("{}{}", &method.name, method.overload)
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

