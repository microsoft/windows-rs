#![allow(clippy::many_single_char_names)]

use super::*;

// TODO: move tool-specific code gen into the tool itself to avoid carrying this extra code in the shared crates?

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
        _ => quote! {},
    }
}

fn gen_type_def(def: &TypeDef, gen: &Gen) -> TokenStream {
    match def.kind() {
        TypeKind::Interface => gen_interface(def),
        TypeKind::Class => gen_class(def),
        TypeKind::Enum => gen_enum(def, gen),
        TypeKind::Struct => gen_struct(def, gen),
        TypeKind::Delegate => {
            if def.is_winrt() {
                gen_interface(def)
            } else {
                gen_callback(def, gen)
            }
        }
    }
}

fn gen_interface(def: &TypeDef) -> TokenStream {
    if def.is_exclusive() {
        quote! {}
    } else {
        let name: TokenStream = if def.generics.is_empty() {
            def.name().into()
        } else {
            let name = def.name();
            name[..name.len() - 2].into()
        };

        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    }
}

fn gen_class(def: &TypeDef) -> TokenStream {
    let has_default = def.interface_impls().any(|interface| interface.is_default());

    if has_default {
        gen_interface(def)
    } else {
        quote! {}
    }
}

fn gen_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_name(def, gen);
    let underlying_type = def.underlying_type();
    let underlying_type = gen_name(&underlying_type, gen);

    if def.is_scoped() {
        let fields = def.fields().filter_map(|field| {
            if field.is_literal() {
                let field_name = to_ident(field.name());
                let constant = field.constant().unwrap();
                let value = gen_constant_value(&constant.value());

                Some(quote! {
                    pub const #field_name: Self = Self(#value);
                })
            } else {
                None
            }
        });

        quote! {
            #[repr(transparent)]
            pub struct #name(pub #underlying_type);
            impl #name {
                #(#fields)*
            }
            impl ::core::marker::Copy for #name {}
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    } else {
        let fields = def.fields().filter_map(|field| {
            if field.is_literal() {
                let field_name = to_ident(field.name());
                let constant = field.constant().unwrap();
                let value = gen_constant_value(&constant.value());

                Some(quote! {
                    pub const #field_name: #name = #value;
                })
            } else {
                None
            }
        });

        quote! {
            pub type #name = #underlying_type;
            #(#fields)*
        }
    }
}

fn gen_struct(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.is_api_contract() {
        return quote! {};
    }

    gen_struct_with_name(def, def.name(), gen, &TokenStream::new())
}

fn gen_struct_with_name(def: &TypeDef, struct_name: &str, gen: &Gen, cfg: &TokenStream) -> TokenStream {
    let name = to_ident(struct_name);

    if def.is_handle() {
        let signature = if def.type_name() == TypeName::HANDLE {
            quote! { *mut ::core::ffi::c_void }
        } else {
            let signature = def.fields().next().map(|field| field.signature(Some(def))).unwrap();
            gen_sys_sig(&signature, gen)
        };

        return quote! {
            pub type #name = #signature;
        };
    }

    let is_union = def.is_union();

    let (doc, cfg) = if cfg.is_empty() {
        let features = features(def, gen);
        let cfg = gen.gen_struct_cfg(def, &features);
        let doc = gen.gen_cfg_doc(&features);
        (doc, cfg)
    } else {
        (TokenStream::new(), cfg.clone())
    };

    let fields: Vec<(Field, Signature, TokenStream)> = def
        .fields()
        .filter_map(move |f| {
            if f.is_literal() {
                None
            } else {
                let signature = f.signature(Some(def));
                let name = f.name();
                Some((f, signature, to_ident(name)))
            }
        })
        .collect();

    if fields.is_empty() {
        if let Some(guid) = GUID::from_attributes(def.attributes()) {
            let guid = gen_sys_guid(&guid);

            return quote! {
                pub const #name: ::windows_sys::core::GUID = #guid;
            };
        } else {
            return quote! {
                #[repr(C)]
                pub struct #name(pub u8);
            };
        }
    }

    let repr = if let Some(layout) = def.class_layout() {
        let packing = Literal::u32_unsuffixed(layout.packing_size());
        quote! { #[repr(C, packed(#packing))] }
    } else {
        quote! { #[repr(C)] }
    };

    let fields = fields.iter().map(|(_, signature, name)| {
        let kind = gen_sys_sig(signature, gen);

        quote! {
            pub #name: #kind
        }
    });

    let struct_or_union = if is_union {
        quote! { union }
    } else {
        quote! { struct }
    };

    let nested_structs = gen_nested_structs(struct_name, def, gen, &cfg);
    let constants = gen_struct_constants(def, &name, &cfg);

    quote! {
        #repr
        #cfg
        #doc
        pub #struct_or_union #name {#(#fields),*}
        #constants
        #cfg
        impl ::core::marker::Copy for #name {}
        #cfg
        impl ::core::clone::Clone for #name {
            fn clone(&self) -> Self {
                *self
            }
        }
        #nested_structs
    }
}

fn gen_struct_constants(def: &TypeDef, struct_name: &TokenStream, cfg: &TokenStream) -> TokenStream {
    let constants = def.fields().filter_map(|f| {
        if f.is_literal() {
            if let Some(constant) = f.constant() {
                let name = to_ident(f.name());
                let value = gen_constant_type_value(&constant.value());

                return Some(quote! {
                    pub const #name: #value;
                });
            }
        }

        None
    });

    let mut tokens = quote! { #(#constants)* };

    if !tokens.is_empty() {
        tokens = quote! {
            #cfg
            impl #struct_name {
                #tokens
            }
        };
    }

    tokens
}

fn gen_nested_structs<'a>(enclosing_name: &'a str, enclosing_type: &'a TypeDef, gen: &Gen, cfg: &TokenStream) -> TokenStream {
    if let Some(nested_types) = enclosing_type.nested_types() {
        nested_types
            .iter()
            .enumerate()
            .map(|(index, (_, nested_type))| {
                let nested_name = format!("{}_{}", enclosing_name, index);
                gen_struct_with_name(nested_type, &nested_name, gen, cfg)
            })
            .collect()
    } else {
        TokenStream::new()
    }
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
                pub const #name: #value;
            }
        } else {
            let kind = gen_sys_sig(&signature, gen);
            let value = gen_constant_value(&constant.value());

            let value = if signature.kind.underlying_type() == constant.value_type() {
                value
            } else {
                quote! { #value as _ }
            };

            if signature.kind == constant.value_type() || signature.kind.is_handle() || signature.kind == ElementType::HRESULT {
                quote! {
                    #cfg
                    #doc
                    pub const #name: #kind = #value;
                }
            } else {
                quote! {
                    #cfg
                    #doc
                    pub const #name: #kind = #kind(#value);
                }
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

fn gen_callback(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_name(def, gen);
    let signature = def.invoke_method().signature(&[]);
    let return_sig = gen_sys_return_sig(&signature, gen);
    let cfg = gen.gen_function_cfg(def.attributes(), &signature);

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_sys_param(p, gen);
        quote! { #name: #tokens }
    });

    quote! {
        #cfg
        pub type #name = ::core::option::Option<unsafe extern "system" fn(#(#params),*) #return_sig>;
    }
}

fn gen_sys_guid(guid: &GUID) -> TokenStream {
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

    // TODO: once code complete measure how much longer it takes if-any to use from_u128 to produce a more compact package

    quote! {
        ::windows_sys::core::GUID { data1:#a, data2:#b, data3:#c, data4:[#d, #e, #f, #g, #h, #i, #j, #k] }
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
    gen_sys_param_with_const(sig, gen, sig.is_const)
}

fn gen_sys_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_sys_param_with_const(&param.signature, gen, !param.param.flags().output())
}

fn gen_sys_param_with_const(sig: &Signature, gen: &Gen, is_const: bool) -> TokenStream {
    let mut tokens = TokenStream::new();

    for _ in 0..sig.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    tokens.combine(&gen_sys_name(&sig.kind, gen));
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
        ElementType::TypeDef(def) => {
            let mut def = def.clone();
            def.generics.clear();
            gen_type_name(&def, gen)
        }
        ElementType::GenericParam(generic) => generic.into(),
        ElementType::MethodDef(def) => def.name().into(),
        ElementType::Field(field) => field.name().into(),
        _ => unimplemented!(),
    }
}
