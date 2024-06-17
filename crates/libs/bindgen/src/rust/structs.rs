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
        let guid = writer.type_name(&metadata::Type::Name(metadata::TypeName::GUID));
        return quote! {
            pub const #ident: #guid = #value;
        };
    }

    gen_struct_with_name(writer, def, def.name(), &cfg::Cfg::default())
}

fn gen_struct_with_name(
    writer: &Writer,
    def: metadata::TypeDef,
    struct_name: &str,
    cfg: &cfg::Cfg,
) -> TokenStream {
    let name = to_ident(struct_name);
    let flags = def.flags();
    let cfg = cfg.union(cfg::type_def_cfg(writer, def, &[]));

    if !cfg.included(writer) {
        return quote! {};
    }

    let repr = if let Some(layout) = def.class_layout() {
        let packing = Literal::usize_unsuffixed(layout.packing_size());
        quote! { #[repr(C, packed(#packing))] }
    } else {
        quote! { #[repr(C)] }
    };

    let fields = if def.fields().next().is_none() {
        quote! { (pub u8); }
    } else {
        let fields = def.fields().map(|f| {
            let name = to_ident(f.name());
            let ty = f.ty(Some(def));

            if f.flags().contains(metadata::FieldAttributes::Literal) {
                quote! {}
            } else if !writer.sys
                && flags.contains(metadata::TypeAttributes::ExplicitLayout)
                && !metadata::field_is_copyable(f, def)
            {
                let ty = writer.type_default_name(&ty);
                quote! { pub #name: core::mem::ManuallyDrop<#ty>, }
            } else if !writer.sys
                && !flags.contains(metadata::TypeAttributes::WindowsRuntime)
                && !metadata::field_is_blittable(f, def)
            {
                if let metadata::Type::Win32Array(ty, len) = ty {
                    let ty = writer.type_default_name(&ty);
                    quote! { pub #name: [core::mem::ManuallyDrop<#ty>; #len], }
                } else {
                    let ty = writer.type_default_name(&ty);
                    quote! { pub #name: core::mem::ManuallyDrop<#ty>, }
                }
            } else {
                let ty = writer.type_default_name(&ty);
                quote! { pub #name: #ty, }
            }
        });

        quote! { {#(#fields)*} }
    };

    let struct_or_union = if flags.contains(metadata::TypeAttributes::ExplicitLayout) {
        quote! { union }
    } else {
        quote! { struct }
    };

    let features = writer.cfg_features(&cfg);
    let derive = gen_derive(writer, def);

    let mut tokens = quote! {
        #repr
        #features
        #derive
        pub #struct_or_union #name #fields
    };

    tokens.combine(&gen_struct_constants(writer, def, &name, &cfg));
    tokens.combine(&gen_clone(writer, def, &name, &cfg));
    tokens.combine(&gen_windows_traits(writer, def, &name, &cfg));

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
        tokens.combine(&gen_struct_with_name(
            writer,
            nested_type,
            &nested_name,
            &cfg,
        ));
    }

    tokens
}

fn gen_windows_traits(
    writer: &Writer,
    def: metadata::TypeDef,
    name: &TokenStream,
    cfg: &cfg::Cfg,
) -> TokenStream {
    if writer.sys {
        quote! {}
    } else {
        let features = writer.cfg_features(cfg);
        let is_copy = metadata::type_def_is_blittable(def);

        let type_kind = if is_copy {
            quote! { CopyType }
        } else {
            quote! { CloneType }
        };

        let mut tokens = quote! {
            #features
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::#type_kind;
            }
        };

        if def
            .flags()
            .contains(metadata::TypeAttributes::WindowsRuntime)
        {
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

fn gen_derive(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    let mut derive = std::collections::BTreeSet::new();

    if !writer.sys
        && !metadata::type_def_has_explicit_layout(def)
        && !metadata::type_def_has_packing(def)
    {
        derive.insert(to_ident("Debug"));
    }

    if writer.sys || metadata::type_def_is_copyable(def) {
        derive.insert(to_ident("Copy"));
        derive.insert(to_ident("Clone"));
    } else if def
        .flags()
        .contains(metadata::TypeAttributes::WindowsRuntime)
    {
        derive.insert(to_ident("Clone"));
    }

    if !writer.sys
        && !metadata::type_def_has_explicit_layout(def)
        && !metadata::type_def_has_packing(def)
        && !metadata::type_def_has_callback(def)
    {
        derive.insert(to_ident("PartialEq"));

        if !metadata::type_def_has_float(def) {
            derive.insert(to_ident("Eq"));
        }
    }

    if derive.is_empty() {
        quote! {}
    } else {
        quote! {
            #[derive(#(#derive),*)]
        }
    }
}

fn gen_clone(
    writer: &Writer,
    def: metadata::TypeDef,
    name: &TokenStream,
    cfg: &cfg::Cfg,
) -> TokenStream {
    if writer.sys
        || metadata::type_def_is_copyable(def)
        || def
            .flags()
            .contains(metadata::TypeAttributes::WindowsRuntime)
        || def.class_layout().is_some()
    {
        quote! {}
    } else {
        let features = writer.cfg_features(cfg);

        quote! {
            #features
            impl Clone for #name {
                fn clone(&self) -> Self {
                    unsafe { core::mem::transmute_copy(self) }
                }
            }
        }
    }
}

fn gen_struct_constants(
    writer: &Writer,
    def: metadata::TypeDef,
    struct_name: &TokenStream,
    cfg: &cfg::Cfg,
) -> TokenStream {
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
