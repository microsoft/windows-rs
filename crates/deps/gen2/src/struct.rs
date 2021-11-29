use super::*;

pub fn gen_struct(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.is_api_contract() {
        return quote! {};
    }

    if gen.sys {
        gen_sys_struct_with_name(def, def.name(), gen, &quote! {}, &quote! {})
    } else {
        quote! {}
    }
}

fn gen_sys_struct_with_name(def: &TypeDef, struct_name: &str, gen: &Gen, arch_cfg: &TokenStream, feature_cfg: &TokenStream) -> TokenStream {
    let name = gen_ident(struct_name);

    if def.is_handle() {
        let signature = if def.type_name() == TypeName::HANDLE {
            quote! { *mut ::core::ffi::c_void }
        } else {
            let signature = def.fields().next().map(|field| field.signature(Some(def))).unwrap();
            gen_sig(&signature, gen)
        };

        return quote! {
            pub type #name = #signature;
        };
    }

    let is_union = def.is_explicit();

    let arch_cfg = if arch_cfg.is_empty() { gen.arch_cfg(def.attributes()) } else { arch_cfg.clone() };
    let feature_cfg = if feature_cfg.is_empty() { gen.type_cfg(def) } else { feature_cfg.clone() };

    let fields: Vec<(Field, Signature, TokenStream)> = def
        .fields()
        .filter_map(move |f| {
            if f.is_literal() {
                None
            } else {
                let signature = f.signature(Some(def));
                let name = f.name();
                Some((f, signature, gen_ident(name)))
            }
        })
        .collect();

    if fields.is_empty() {
        if let Some(guid) = GUID::from_attributes(def.attributes()) {
            let guid = gen_guid(&guid, gen);

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
        let kind = gen_sig(signature, gen);

        quote! {
            pub #name: #kind
        }
    });

    let struct_or_union = if is_union {
        quote! { union }
    } else {
        quote! { struct }
    };

    let nested_structs = gen_nested_sys_structs(struct_name, def, gen, &arch_cfg, &feature_cfg);
    let constants = gen_struct_constants(def, &name, &arch_cfg, &feature_cfg);

    quote! {
        #repr
        #arch_cfg
        #feature_cfg
        pub #struct_or_union #name {#(#fields),*}
        #constants
        #arch_cfg
        #feature_cfg
        impl ::core::marker::Copy for #name {}
        #arch_cfg
        #feature_cfg
        impl ::core::clone::Clone for #name {
            fn clone(&self) -> Self {
                *self
            }
        }
        #nested_structs
    }
}

fn gen_struct_constants(def: &TypeDef, struct_name: &TokenStream, arch_cfg: &TokenStream, feature_cfg: &TokenStream) -> TokenStream {
    let constants = def.fields().filter_map(|f| {
        if f.is_literal() {
            if let Some(constant) = f.constant() {
                let name = gen_ident(f.name());
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
            #arch_cfg
            #feature_cfg
            impl #struct_name {
                #tokens
            }
        };
    }

    tokens
}

fn gen_nested_sys_structs<'a>(enclosing_name: &'a str, enclosing_type: &'a TypeDef, gen: &Gen, arch_cfg: &TokenStream, feature_cfg: &TokenStream) -> TokenStream {
    if let Some(nested_types) = enclosing_type.nested_types() {
        nested_types
            .iter()
            .enumerate()
            .map(|(index, (_, nested_type))| {
                let nested_name = format!("{}_{}", enclosing_name, index);
                gen_sys_struct_with_name(nested_type, &nested_name, gen, arch_cfg, feature_cfg)
            })
            .collect()
    } else {
        TokenStream::new()
    }
}
