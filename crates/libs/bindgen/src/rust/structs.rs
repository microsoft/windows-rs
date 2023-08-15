use super::*;

pub fn writer(writer: &Writer, def: TypeDef) -> TokenStream {
    if writer.reader.has_attribute(def, "ApiContractAttribute") {
        return quote! {};
    }

    if type_def_is_handle(writer.reader, def) {
        return handles::writer(writer, def);
    }

    gen_struct_with_name(writer, def, writer.reader.type_def_name(def), &Cfg::default())
}

fn gen_struct_with_name(writer: &Writer, def: TypeDef, struct_name: &str, cfg: &Cfg) -> TokenStream {
    let name = to_ident(struct_name);

    if writer.reader.type_def_fields(def).next().is_none() {
        let mut tokens = quote! {
            #[repr(C)]
            pub struct #name(pub u8);
            impl ::core::marker::Copy for #name {}
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        };
        if !writer.sys {
            tokens.combine(&quote! {
                impl ::windows_core::TypeKind for #name {
                    type TypeKind = ::windows_core::CopyType;
                }
            });
        }
        return tokens;
    }

    let flags = writer.reader.type_def_flags(def);
    let cfg = cfg.union(&type_def_cfg(writer.reader, def, &[]));

    let repr = if let Some(layout) = writer.reader.type_def_class_layout(def) {
        let packing = Literal::usize_unsuffixed(writer.reader.class_layout_packing_size(layout));
        quote! { #[repr(C, packed(#packing))] }
    } else {
        quote! { #[repr(C)] }
    };

    let fields = writer.reader.type_def_fields(def).map(|f| {
        let name = to_ident(writer.reader.field_name(f));
        let ty = writer.reader.field_type(f, Some(def));

        if writer.reader.field_flags(f).contains(FieldAttributes::Literal) {
            quote! {}
        } else if !writer.sys && flags.contains(TypeAttributes::ExplicitLayout) && !field_is_copyable(writer.reader, f, def) {
            let ty = writer.type_default_name(&ty);
            quote! { pub #name: ::std::mem::ManuallyDrop<#ty>, }
        } else if !writer.sys && !flags.contains(TypeAttributes::WindowsRuntime) && !field_is_blittable(writer.reader, f, def) {
            if let Type::Win32Array(ty, len) = ty {
                let ty = writer.type_default_name(&ty);
                quote! { pub #name: [::std::mem::ManuallyDrop<#ty>; #len], }
            } else {
                let ty = writer.type_default_name(&ty);
                quote! { pub #name: ::std::mem::ManuallyDrop<#ty>, }
            }
        } else {
            let ty = writer.type_default_name(&ty);
            quote! { pub #name: #ty, }
        }
    });

    let struct_or_union = if flags.contains(TypeAttributes::ExplicitLayout) {
        quote! { union }
    } else {
        quote! { struct }
    };

    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);

    let mut tokens = quote! {
        #repr
        #doc
        #features
        pub #struct_or_union #name {#(#fields)*}
    };

    tokens.combine(&gen_struct_constants(writer, def, &name, &cfg));
    tokens.combine(&gen_copy_clone(writer, def, &name, &cfg));
    tokens.combine(&gen_debug(writer, def, &name, &cfg));
    tokens.combine(&gen_windows_traits(writer, def, &name, &cfg));
    tokens.combine(&gen_compare_traits(writer, def, &name, &cfg));

    if !writer.sys {
        tokens.combine(&quote! {
            #features
            impl ::core::default::Default for #name {
                fn default() -> Self {
                    unsafe { ::core::mem::zeroed() }
                }
            }
        });
    }

    for (index, nested_type) in writer.reader.nested_types(def).enumerate() {
        let nested_name = format!("{struct_name}_{index}");
        tokens.combine(&gen_struct_with_name(writer, nested_type, &nested_name, &cfg));
    }

    tokens
}

fn gen_windows_traits(writer: &Writer, def: TypeDef, name: &TokenStream, cfg: &Cfg) -> TokenStream {
    if writer.sys {
        quote! {}
    } else {
        let features = writer.cfg_features(cfg);
        let is_copy = type_def_is_blittable(writer.reader, def);

        let type_kind = if is_copy {
            quote! { CopyType }
        } else {
            quote! { ValueType }
        };

        let mut tokens = quote! {
            #features
            impl ::windows_core::TypeKind for #name {
                type TypeKind = ::windows_core::#type_kind;
            }
        };

        if writer.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
            let signature = Literal::byte_string(type_def_signature(writer.reader, def, &[]).as_bytes());

            tokens.combine(&quote! {
                #features
                impl ::windows_core::RuntimeType for #name {
                    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(#signature);
                }
            });
        }

        tokens
    }
}

fn gen_compare_traits(writer: &Writer, def: TypeDef, name: &TokenStream, cfg: &Cfg) -> TokenStream {
    let features = writer.cfg_features(cfg);

    if writer.sys || type_def_has_explicit_layout(writer.reader, def) || type_def_has_packing(writer.reader, def) || type_def_has_callback(writer.reader, def) {
        quote! {}
    } else {
        let fields = writer.reader.type_def_fields(def).filter_map(|f| {
            let name = to_ident(writer.reader.field_name(f));
            if writer.reader.field_flags(f).contains(FieldAttributes::Literal) {
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

fn gen_debug(writer: &Writer, def: TypeDef, ident: &TokenStream, cfg: &Cfg) -> TokenStream {
    if writer.sys || type_def_has_explicit_layout(writer.reader, def) || type_def_has_packing(writer.reader, def) {
        quote! {}
    } else {
        let name = ident.as_str();
        let features = writer.cfg_features(cfg);

        let fields = writer.reader.type_def_fields(def).filter_map(|f| {
            if writer.reader.field_flags(f).contains(FieldAttributes::Literal) {
                None
            } else {
                let name = writer.reader.field_name(f);
                let ident = to_ident(name);
                let ty = writer.reader.field_type(f, Some(def));
                if type_has_callback(writer.reader, &ty) {
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

fn gen_copy_clone(writer: &Writer, def: TypeDef, name: &TokenStream, cfg: &Cfg) -> TokenStream {
    let features = writer.cfg_features(cfg);

    if writer.sys || type_def_is_copyable(writer.reader, def) {
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
    } else if writer.reader.type_def_class_layout(def).is_some() {
        // Don't support copy/clone of packed structs: https://github.com/rust-lang/rust/issues/82523
        quote! {}
    } else if !writer.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
        quote! {
            #features
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    unsafe { ::core::mem::transmute_copy(self) }
                }
            }
        }
    } else {
        let fields = writer.reader.type_def_fields(def).map(|f| {
            let name = to_ident(writer.reader.field_name(f));
            if writer.reader.field_flags(f).contains(FieldAttributes::Literal) {
                quote! {}
            } else if field_is_blittable(writer.reader, f, def) {
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

fn gen_struct_constants(writer: &Writer, def: TypeDef, struct_name: &TokenStream, cfg: &Cfg) -> TokenStream {
    let features = writer.cfg_features(cfg);

    let constants = writer.reader.type_def_fields(def).filter_map(|f| {
        if writer.reader.field_flags(f).contains(FieldAttributes::Literal) {
            if let Some(constant) = writer.reader.field_constant(f) {
                let name = to_ident(writer.reader.field_name(f));
                let value = writer.typed_value(&writer.reader.constant_value(constant));

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
