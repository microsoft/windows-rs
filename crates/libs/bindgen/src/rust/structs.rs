use super::*;
use metadata::HasAttributes;

pub fn writer(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    if def.has_attribute("ApiContractAttribute") {
        return quote! {};
    }

    if metadata::type_def_is_handle(def) {
        return handles::writer(writer, def);
    }

    if let Some(guid) = clsid(def) {
        let ident = to_ident(def.name());
        let value = writer.guid(&guid);
        let guid = writer.type_name(&metadata::Type::GUID);
        return quote! {
            pub const #ident: #guid = #value;
        };
    }

    gen_struct_with_name(writer, def, def.name(), &cfg::Cfg::default())
}

fn gen_struct_with_name(writer: &Writer, def: metadata::TypeDef, struct_name: &str, cfg: &cfg::Cfg) -> TokenStream {
    let name = to_ident(struct_name);

    if def.fields().next().is_none() {
        let mut tokens = quote! {
            #[repr(C)]
            pub struct #name(pub u8);
            impl Copy for #name {}
            impl Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        };
        if !writer.sys {
            tokens.combine(&quote! {
                impl windows_core::TypeKind for #name {
                    type TypeKind = windows_core::CopyType;
                }
            });
        }
        return tokens;
    }

    let flags = def.flags();
    let cfg = cfg.union(&cfg::type_def_cfg(writer, def, &[]));

    let repr = if let Some(layout) = def.class_layout() {
        let packing = Literal::usize_unsuffixed(layout.packing_size());
        quote! { #[repr(C, packed(#packing))] }
    } else {
        quote! { #[repr(C)] }
    };

    let fields = def.fields().map(|f| {
        let name = to_ident(f.name());
        let ty = f.ty(Some(def));

        if f.flags().contains(metadata::FieldAttributes::Literal) {
            quote! {}
        } else if !writer.sys && flags.contains(metadata::TypeAttributes::ExplicitLayout) && !metadata::field_is_copyable(f, def) {
            let ty = writer.type_default_name(&ty);
            quote! { pub #name: std::mem::ManuallyDrop<#ty>, }
        } else if !writer.sys && !flags.contains(metadata::TypeAttributes::WindowsRuntime) && !metadata::field_is_blittable(f, def) {
            if let metadata::Type::Win32Array(ty, len) = ty {
                let ty = writer.type_default_name(&ty);
                quote! { pub #name: [std::mem::ManuallyDrop<#ty>; #len], }
            } else {
                let ty = writer.type_default_name(&ty);
                quote! { pub #name: std::mem::ManuallyDrop<#ty>, }
            }
        } else {
            let ty = writer.type_default_name(&ty);
            quote! { pub #name: #ty, }
        }
    });

    let struct_or_union = if flags.contains(metadata::TypeAttributes::ExplicitLayout) {
        quote! { union }
    } else {
        quote! { struct }
    };

    let features = writer.cfg_features(&cfg);

    let mut tokens = quote! {
        #repr
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
            impl Default for #name {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
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

fn gen_windows_traits(writer: &Writer, def: metadata::TypeDef, name: &TokenStream, cfg: &cfg::Cfg) -> TokenStream {
    if writer.sys {
        quote! {}
    } else {
        let features = writer.cfg_features(cfg);
        let is_copy = metadata::type_def_is_blittable(def);

        let type_kind = if is_copy {
            quote! { CopyType }
        } else {
            quote! { ValueType }
        };

        let mut tokens = quote! {
            #features
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::#type_kind;
            }
        };

        if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
            let signature = Literal::byte_string(metadata::type_def_signature(def, &[]).as_bytes());

            tokens.combine(&quote! {
                #features
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#signature);
                }
            });
        }

        tokens
    }
}

fn gen_compare_traits(writer: &Writer, def: metadata::TypeDef, name: &TokenStream, cfg: &cfg::Cfg) -> TokenStream {
    let features = writer.cfg_features(cfg);

    if writer.sys || metadata::type_def_has_explicit_layout(def) || metadata::type_def_has_packing(def) || metadata::type_def_has_callback(def) {
        quote! {}
    } else {
        let fields = def.fields().filter_map(|f| {
            let name = to_ident(f.name());
            if f.flags().contains(metadata::FieldAttributes::Literal) {
                None
            } else {
                Some(quote! { self.#name == other.#name })
            }
        });

        quote! {
            #features
            impl PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    #(#fields)&&*
                }
            }
            #features
            impl Eq for #name {}
        }
    }
}

fn gen_debug(writer: &Writer, def: metadata::TypeDef, ident: &TokenStream, cfg: &cfg::Cfg) -> TokenStream {
    if writer.sys || metadata::type_def_has_explicit_layout(def) || metadata::type_def_has_packing(def) {
        quote! {}
    } else {
        let name = ident.as_str();
        let features = writer.cfg_features(cfg);

        let fields = def.fields().filter_map(|f| {
            if f.flags().contains(metadata::FieldAttributes::Literal) {
                None
            } else {
                let name = f.name();
                let ident = to_ident(name);
                let ty = f.ty(Some(def));
                if metadata::type_has_callback(&ty) {
                    None
                } else {
                    Some(quote! { .field(#name, &self.#ident) })
                }
            }
        });

        quote! {
            #features
            impl core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct(#name) #(#fields)* .finish()
                }
            }
        }
    }
}

fn gen_copy_clone(writer: &Writer, def: metadata::TypeDef, name: &TokenStream, cfg: &cfg::Cfg) -> TokenStream {
    let features = writer.cfg_features(cfg);

    if writer.sys || metadata::type_def_is_copyable(def) {
        quote! {
            #features
            impl Copy for #name {}
            #features
            impl Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    } else if def.class_layout().is_some() {
        // Don't support copy/clone of packed structs: https://github.com/rust-lang/rust/issues/82523
        quote! {}
    } else if !def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
        quote! {
            #features
            impl Clone for #name {
                fn clone(&self) -> Self {
                    unsafe { core::mem::transmute_copy(self) }
                }
            }
        }
    } else {
        let fields = def.fields().map(|f| {
            let name = to_ident(f.name());
            if f.flags().contains(metadata::FieldAttributes::Literal) {
                quote! {}
            } else if metadata::field_is_blittable(f, def) {
                quote! { #name: self.#name }
            } else {
                quote! { #name: self.#name.clone() }
            }
        });

        quote! {
            #features
            impl Clone for #name {
                fn clone(&self) -> Self {
                    Self { #(#fields),* }
                }
            }
        }
    }
}

fn gen_struct_constants(writer: &Writer, def: metadata::TypeDef, struct_name: &TokenStream, cfg: &cfg::Cfg) -> TokenStream {
    let features = writer.cfg_features(cfg);

    let constants = def.fields().filter_map(|f| {
        if f.flags().contains(metadata::FieldAttributes::Literal) {
            if let Some(constant) = f.constant() {
                let name = to_ident(f.name());
                let value = writer.typed_value(&constant.value());

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

fn clsid(def: metadata::TypeDef) -> Option<metadata::Guid> {
    if def.fields().next().is_none() {
        return metadata::type_def_guid(def);
    }
    None
}
