use super::*;

pub fn gen_sys(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let functions = gen_functions(tree, gen);
    let types = gen_types(tree, gen);

    quote! {
        #functions
        #types
    }
}

fn gen_types(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for entry in tree.types.values() {
        for def in &entry.def {
            tokens.combine(&gen_type(def, gen));
        }
    }

    tokens
}

fn gen_type(entry: &ElementType, gen: &Gen) -> TokenStream {
    match entry {
        ElementType::Field(def) => gen_constant(def, gen),
        ElementType::TypeDef(def) => gen_type_def(&def.clone().with_generics(), gen),
        _ => quote! {}
    }
}

fn gen_type_def(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_name(def, gen);
    let features = features(def, gen);
    let cfg = gen.gen_struct_cfg(def, &features);

    quote! { #cfg pub struct #name(i32); }
}

fn gen_constant(def: &Field, gen: &Gen) -> TokenStream {
    let name = def.name();
    let name = to_ident(name);
    let signature = def.signature(None);

    let features = signature_features(&signature, gen);
    let cfg = gen.gen_cfg(&features);
    let doc = gen.gen_cfg_doc(&features);

    if let Some(constant) = def.constant() {
        if signature.kind == constant.value_type() {
            let value = gen_constant_type_value(&constant.value());

            quote! {
                #cfg
                #doc
                pub const #name: #value;
            }
        } else {
            let kind = gen_sys_sig(&signature, gen);
            let value = gen_constant_value(&constant.value());

            quote! {
                #cfg
                #doc
                pub const #name: #kind = #kind(#value as _);
            }
        }
    } else if let Some(guid) = GUID::from_attributes(def.attributes()) {
        let guid = gen_sys_guid(&guid);
        quote! { pub const #name: ::windows_sys::core::GUID = #guid; }
    } else if let Some(pkey) = PropertyKey::from_attributes(def.attributes()) {
        let kind = gen_sys_sig(&signature, gen);
        let fmtid = gen_sys_guid(&pkey.fmtid);
        let pid = pkey.pid;
        quote! {
            #cfg
            #doc
            pub const #name: #kind = #kind {
                fmtid: #fmtid,
                pid: #pid,
            };
        }
    } else {
        quote! {}
    }
}

fn gen_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut functions = tree.types.values().map(|entry| gen_function_if(entry, gen)).peekable();

    if functions.peek().is_some() {
        quote! {
            #[link(name = "windows")]
            extern "system" {
                #(#functions)*
            }
        }
    } else {
        quote! {}
    }
}

fn gen_function_if(entry: &TypeEntry, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for def in &entry.def {
        if let ElementType::MethodDef(def) = def {
            tokens.combine(&gen_function(def, gen));
        }
    }

    tokens
}

fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = to_ident(def.name());
    let signature = def.signature(&[]);
    let return_type = gen_sys_return_sig(&signature, gen);
    let cfg = gen.gen_function_cfg(def.attributes(), &signature);

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_sys_param(p, gen);
        quote! { #name: #tokens }
    });

    quote! {
        #cfg
        pub fn #name(#(#params),*) #return_type;
    }
}

pub fn gen_sys_guid(guid: &GUID) -> TokenStream {
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
        ::windows_sys::GUID { data1:#a, data2:#b, data3:#c, data4:[#d, #e, #f, #g, #h, #i, #j, #k] }
    }
}

fn gen_sys_return_sig(signature: &MethodSignature, gen: &Gen) -> TokenStream {
    if let Some(return_sig) = &signature.return_sig {
        let tokens = gen_sys_sig(return_sig, gen);
        quote! { -> #tokens }
    } else {
        quote! {}
    }
}

fn gen_sys_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

    for _ in 0..sig.pointers {
        if sig.is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    let kind = gen_sys_name(&sig.kind, gen);

    if sig.kind.is_nullable() {
        tokens.combine(&quote! {
            ::core::option::Option<#kind>
        });
    } else {
        tokens.combine(&kind)
    }

    tokens
}

fn gen_sys_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();
    let is_const = !param.param.flags().output();

    for _ in 0..param.signature.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    tokens.combine(&gen_sys_name(&param.signature.kind, gen));
    tokens
}

fn gen_sys_name(def: &ElementType, gen: &Gen) -> TokenStream {
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
            quote! { ::windows_sys::core::HSTRING }
        }
        ElementType::IInspectable => {
            quote! { ::windows_sys::core::IInspectable }
        }
        ElementType::GUID => {
            quote! { ::windows_sys::core::GUID }
        }
        ElementType::IUnknown => {
            quote! { ::windows_sys::core::IUnknown }
        }
        ElementType::HRESULT => {
            quote! { ::windows_sys::core::HRESULT }
        }
        ElementType::Array((kind, len)) => {
            let name = gen_sys_sig(kind, gen);
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
