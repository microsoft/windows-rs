use super::*;

pub fn gen_struct(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.is_api_contract() {
        return quote! {};
    }

    gen_struct_with_name(def, def.name(), gen, &quote! {}, &quote! {})
}

fn gen_struct_with_name(def: &TypeDef, struct_name: &str, gen: &Gen, arch_cfg: &TokenStream, feature_cfg: &TokenStream) -> TokenStream {
    if !gen.sys {
        if let Some(replacement) = replacements::gen(def) {
            return replacement;
        }
    }

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

    if def.fields().next().is_none() {
        if let Some(guid) = GUID::from_attributes(def.attributes()) {
            let value = gen_guid(&guid, gen);
            let guid = gen_element_name(&ElementType::GUID, gen);
            return quote! { pub const #name: #guid = #value; };
        } else if name.as_str().ends_with("Vtbl") {
            return quote! {};
        } else {
            return quote! {
                #[repr(C)]
                pub struct #name(pub u8);
            };
        }
    }

    let is_union = def.is_union();
    let arch_cfg = if arch_cfg.is_empty() { gen.arch_cfg(def.attributes()) } else { arch_cfg.clone() };
    let feature_cfg = if feature_cfg.is_empty() { gen.type_cfg(def) } else { feature_cfg.clone() };

    let repr = if let Some(layout) = def.class_layout() {
        let packing = Literal::u32_unsuffixed(layout.packing_size());
        quote! { #[repr(C, packed(#packing))] }
    } else {
        quote! { #[repr(C)] }
    };

    let fields = def.fields().map(|f| {
        let name = gen_ident(f.name());
        let sig = gen_sig(&f.signature(Some(def)), gen);
        if f.is_literal() {
            quote! {}
        } else if !gen.sys && is_union && !f.is_blittable(Some(def)) {
            quote! { pub #name: ::core::mem::ManuallyDrop<#sig>, }
        } else {
            quote! { pub #name: #sig, }
        }
    });

    let struct_or_union = if is_union {
        quote! { union }
    } else {
        quote! { struct }
    };

    let mut tokens = quote! {
        #repr
        #arch_cfg
        #feature_cfg
        pub #struct_or_union #name {#(#fields)*}
    };

    tokens.combine(&gen_struct_constants(def, &name, &arch_cfg, &feature_cfg));
    tokens.combine(&gen_copy_clone(def, &name, gen, &arch_cfg, &feature_cfg));
    tokens.combine(&gen_windows_traits(def, &name, gen, &arch_cfg, &feature_cfg));

    if !gen.sys {
        tokens.combine(&quote! {
            #arch_cfg
            #feature_cfg
            impl ::core::default::Default for #name {
                fn default() -> Self {
                    unsafe { ::core::mem::zeroed() }
                }
            }
        });

        tokens.combine(&extensions::gen(def));
    }

    if let Some(nested_types) = def.nested_types() {
        for (index, (_, nested_type)) in nested_types.iter().enumerate() {
            let nested_name = format!("{}_{}", struct_name, index);
            tokens.combine(&gen_struct_with_name(nested_type, &nested_name, gen, &arch_cfg, &feature_cfg));
        }
    }

    tokens
}

fn gen_windows_traits(def: &TypeDef, name: &TokenStream, gen: &Gen, arch_cfg: &TokenStream, feature_cfg: &TokenStream) -> TokenStream {
    if gen.sys {
        quote! {}
    } else {
        let abi = if def.is_blittable() {
            quote! { Self }
        } else {
            quote! { ::core::mem::ManuallyDrop<Self> }
        };

        quote! {
            #arch_cfg
            #feature_cfg
            unsafe impl ::windows::core::Abi for #name {
                type Abi = #abi;
            }
        }
    }
}

fn gen_copy_clone(def: &TypeDef, name: &TokenStream, gen: &Gen, arch_cfg: &TokenStream, feature_cfg: &TokenStream) -> TokenStream {
    if gen.sys || def.is_blittable() {
        quote! {
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
        }
    } else if def.is_union() {
        quote! {
            #arch_cfg
            #feature_cfg
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    unsafe { ::core::mem::transmute_copy(self) }
                }
            }
        }
    } else if def.class_layout().is_some() {
        // Don't support copy/clone of packed structs: https://github.com/rust-lang/rust/issues/82523
        quote! {}
    } else {
        let fields = def.fields().map(|f| {
            let name = gen_ident(f.name());
            if f.is_literal() {
                quote! {}
            } else if f.is_blittable(Some(def)) {
                quote! { #name: self.#name }
            } else {
                quote! { #name: self.#name.clone() }
            }
        });

        quote! {
            #arch_cfg
            #feature_cfg
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self { #(#fields),* }
                }
            }
        }
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
