use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.reader.type_def_is_contract(def) {
        return quote! {};
    }

    if gen.reader.type_def_is_handle(def) {
        return handles::gen(gen, def);
    }

    gen_struct_with_name(gen, def, gen.reader.type_def_name(def), &Cfg::default())
}

fn gen_struct_with_name(gen: &Gen, def: TypeDef, struct_name: &str, cfg: &Cfg) -> TokenStream {
    let name = to_ident(struct_name);

    if gen.reader.type_def_fields(def).next().is_none() {
        return quote! {
            #[repr(C)]
            pub struct #name(pub u8);
        };
    }

    let flags = gen.reader.type_def_flags(def);
    let cfg = cfg.union(&gen.reader.type_def_cfg(def, &[]));

    let repr = if let Some(packing_size) = gen.reader.type_def_packing_size(def) {
        let packing = Literal::usize_unsuffixed(packing_size);
        quote! { #[repr(C, packed(#packing))] }
    } else if TypeName::CONTEXT == TypeName::new(gen.namespace, struct_name) {
        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/1044
        if cfg.arches.contains("x86") {
            quote! { #[repr(C, align(4))] }
        } else {
            quote! { #[repr(C, align(16))] }
        }
    } else {
        quote! { #[repr(C)] }
    };

    let fields = gen.reader.type_def_fields(def).map(|f| {
        let name = to_ident(gen.reader.field_name(f));
        let ty = gen.reader.field_type(f, Some(def));

        if gen.reader.field_flags(f).contains(FieldAttributes::LITERAL) {
            quote! {}
        } else if !gen.sys
            && flags.contains(TypeAttributes::EXPLICIT_LAYOUT)
            && !gen.reader.field_is_copyable(f, def)
        {
            // Rust can't tell that the type is copyable and won't accept windows::core::ManuallyDrop
            let ty = gen.type_default_name(&ty);
            quote! { pub #name: ::std::mem::ManuallyDrop<#ty>, }
        } else if !gen.sys
            && !flags.contains(TypeAttributes::WINRT)
            && !gen.reader.field_is_blittable(f, def)
        {
            if let Type::Win32Array((ty, len)) = ty {
                let ty = gen.type_name(&ty);
                quote! { pub #name: [::windows::core::ManuallyDrop<#ty>; #len], }
            } else {
                let ty = gen.type_name(&ty);
                quote! { pub #name: ::windows::core::ManuallyDrop<#ty>, }
            }
        } else {
            let ty = gen.type_default_name(&ty);
            quote! { pub #name: #ty, }
        }
    });

    let struct_or_union = if flags.contains(TypeAttributes::EXPLICIT_LAYOUT) {
        quote! { union }
    } else {
        quote! { struct }
    };

    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    let mut tokens = quote! {
        #repr
        #doc
        #features
        pub #struct_or_union #name {#(#fields)*}
    };

    tokens.combine(&gen_struct_constants(gen, def, &name, &cfg));
    tokens.combine(&gen_copy_clone(gen, def, &name, &cfg));
    tokens.combine(&gen_debug(gen, def, &name, &cfg));
    tokens.combine(&gen_windows_traits(gen, def, &name, &cfg));
    tokens.combine(&gen_compare_traits(gen, def, &name, &cfg));

    if !gen.sys {
        tokens.combine(&quote! {
            #features
            impl ::core::default::Default for #name {
                fn default() -> Self {
                    unsafe { ::core::mem::zeroed() }
                }
            }
        });
    }

    for (index, nested_type) in gen.reader.nested_types(def).enumerate() {
        let nested_name = format!("{struct_name}_{index}");
        tokens.combine(&gen_struct_with_name(gen, nested_type, &nested_name, &cfg));
    }

    tokens
}

fn gen_windows_traits(gen: &Gen, def: TypeDef, name: &TokenStream, cfg: &Cfg) -> TokenStream {
    if gen.sys {
        quote! {}
    } else {
        let features = gen.cfg_features(cfg);
        let is_copy = gen.reader.type_def_is_blittable(def);

        let type_kind = if is_copy {
            quote! { CopyType }
        } else {
            quote! { ValueType }
        };

        let mut tokens = quote! {
            #features
            impl ::windows::core::TypeKind for #name {
                type TypeKind = ::windows::core::#type_kind;
            }
        };

        if gen
            .reader
            .type_def_flags(def)
            .contains(TypeAttributes::WINRT)
        {
            let signature =
                Literal::byte_string(gen.reader.type_def_signature(def, &[]).as_bytes());

            tokens.combine(&quote! {
                #features
                impl ::windows::core::RuntimeType for #name {
                    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(#signature);
                }
            });
        }

        tokens
    }
}

fn gen_compare_traits(gen: &Gen, def: TypeDef, name: &TokenStream, cfg: &Cfg) -> TokenStream {
    let features = gen.cfg_features(cfg);

    if gen.sys
        || gen.reader.type_def_has_explicit_layout(def)
        || gen.reader.type_def_has_packing(def)
        || gen.reader.type_def_has_callback(def)
    {
        quote! {}
    } else {
        let fields = gen.reader.type_def_fields(def).filter_map(|f| {
            let name = to_ident(gen.reader.field_name(f));
            if gen.reader.field_flags(f).contains(FieldAttributes::LITERAL) {
                None
            } else {
                Some(quote! { self.#name == other.#name })
            }
        });

        quote! {
            #features
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    #(#fields)&&*
                }
            }
            #features
            impl ::core::cmp::Eq for #name {}
        }
    }
}

fn gen_debug(gen: &Gen, def: TypeDef, ident: &TokenStream, cfg: &Cfg) -> TokenStream {
    if gen.sys
        || gen.reader.type_def_has_explicit_layout(def)
        || gen.reader.type_def_has_packing(def)
    {
        quote! {}
    } else {
        let name = ident.as_str();
        let features = gen.cfg_features(cfg);

        let fields = gen.reader.type_def_fields(def).filter_map(|f| {
            if gen.reader.field_flags(f).contains(FieldAttributes::LITERAL) {
                None
            } else {
                let name = gen.reader.field_name(f);
                let ident = to_ident(name);
                let ty = gen.reader.field_type(f, Some(def));
                if gen.reader.type_has_callback(&ty) {
                    None
                } else {
                    Some(quote! { .field(#name, &self.#ident) })
                }
            }
        });

        quote! {
            #features
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct(#name) #(#fields)* .finish()
                }
            }
        }
    }
}

fn gen_copy_clone(gen: &Gen, def: TypeDef, name: &TokenStream, cfg: &Cfg) -> TokenStream {
    let features = gen.cfg_features(cfg);

    if gen.sys || gen.reader.type_def_is_copyable(def) {
        quote! {
            #features
            impl ::core::marker::Copy for #name {}
            #features
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    } else if gen.reader.type_def_class_layout(def).is_some() {
        // Don't support copy/clone of packed structs: https://github.com/rust-lang/rust/issues/82523
        quote! {}
    } else if !gen
        .reader
        .type_def_flags(def)
        .contains(TypeAttributes::WINRT)
    {
        quote! {
            #features
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    unsafe { ::core::mem::transmute_copy(self) }
                }
            }
        }
    } else {
        let fields = gen.reader.type_def_fields(def).map(|f| {
            let name = to_ident(gen.reader.field_name(f));
            if gen.reader.field_flags(f).contains(FieldAttributes::LITERAL) {
                quote! {}
            } else if gen.reader.field_is_blittable(f, def) {
                quote! { #name: self.#name }
            } else {
                quote! { #name: self.#name.clone() }
            }
        });

        quote! {
            #features
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self { #(#fields),* }
                }
            }
        }
    }
}

fn gen_struct_constants(
    gen: &Gen,
    def: TypeDef,
    struct_name: &TokenStream,
    cfg: &Cfg,
) -> TokenStream {
    let features = gen.cfg_features(cfg);

    let constants = gen.reader.type_def_fields(def).filter_map(|f| {
        if gen.reader.field_flags(f).contains(FieldAttributes::LITERAL) {
            if let Some(constant) = gen.reader.field_constant(f) {
                let name = to_ident(gen.reader.field_name(f));
                let value = gen.typed_value(&gen.reader.constant_value(constant));

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
            #features
            impl #struct_name {
                #tokens
            }
        };
    }

    tokens
}
