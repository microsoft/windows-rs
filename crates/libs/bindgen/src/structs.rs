use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.is_api_contract() {
        return quote! {};
    }

    if !gen.sys {
        if let Some(replacement) = replacements::gen(def) {
            return replacement;
        }
    }

    if def.is_handle() {
        return handles::gen(def, gen);
    }

    gen_struct_with_name(def, def.name(), &Cfg::new(), gen)
}

fn gen_struct_with_name(def: &TypeDef, struct_name: &str, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let name = gen_ident(struct_name);

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
    let cfg = cfg.union(gen.type_cfg(def));

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

    let features = cfg.gen(gen);
    let doc = cfg.gen_doc(gen);

    let mut tokens = quote! {
        #repr
        #doc
        #features
        pub #struct_or_union #name {#(#fields)*}
    };

    tokens.combine(&gen_struct_constants(def, &name, &cfg, gen));
    tokens.combine(&gen_copy_clone(def, &name, &cfg, gen));
    tokens.combine(&gen_debug(def, &name, &cfg, gen));
    tokens.combine(&gen_windows_traits(def, &name, &cfg, gen));
    tokens.combine(&gen_compare_traits(def, &name, &cfg, gen));

    if !gen.sys {
        tokens.combine(&quote! {
            #features
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
            tokens.combine(&gen_struct_with_name(nested_type, &nested_name, &cfg, gen));
        }
    }

    tokens
}

fn gen_windows_traits(def: &TypeDef, name: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    if gen.sys {
        quote! {}
    } else {
        let abi = if def.is_blittable() {
            quote! { Self }
        } else {
            quote! { ::core::mem::ManuallyDrop<Self> }
        };

        let cfg = cfg.gen(gen);

        let mut tokens = quote! {
            #cfg
            unsafe impl ::windows::core::Abi for #name {
                type Abi = #abi;
            }
        };

        if def.is_winrt() {
            let signature = Literal::byte_string(def.type_signature().as_bytes());

            tokens.combine(&quote! {
                #cfg
                unsafe impl ::windows::core::RuntimeType for #name {
                    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(#signature);
                }
                #cfg
                impl ::windows::core::DefaultType for #name {
                    type DefaultType = Self;
                }
            });
        }

        tokens
    }
}

fn gen_compare_traits(def: &TypeDef, name: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let cfg = cfg.gen(gen);

    if gen.sys {
        quote! {}
    } else if def.is_blittable() || def.is_union() || def.class_layout().is_some() {
        quote! {
            #cfg
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    unsafe {
                        ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<#name>()) == 0
                    }
                }
            }
            #cfg
            impl ::core::cmp::Eq for #name {}
        }
    } else {
        let fields = def.fields().map(|f| {
            let name = gen_ident(f.name());
            if f.is_literal() {
                quote! {}
            } else {
                let sig = f.signature(Some(def));
                if sig.kind.is_callback() {
                    quote! {
                        self.#name.map(|f| f as usize) == other.#name.map(|f| f as usize)
                    }
                } else {
                    quote! { self.#name == other.#name }
                }
            }
        });

        quote! {
            #cfg
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    #(#fields)&&*
                }
            }
            #cfg
            impl ::core::cmp::Eq for #name {}
        }
    }
}

fn gen_debug(def: &TypeDef, ident: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    if gen.sys || def.is_union() || def.has_explicit() || def.is_packed() {
        quote! {}
    } else {
        let name = ident.as_str();
        let cfg = cfg.gen(gen);

        let fields = def.fields().map(|f| {
            if f.is_literal() {
                quote! {}
            } else {
                let name = f.name();
                let ident = gen_ident(name);
                let signature = f.signature(Some(def));
                if signature.is_callback() {
                    quote! { .field(#name, &self.#ident.map(|f| f as usize)) }
                } else if signature.is_callback_array() {
                    quote! {}
                } else {
                    quote! { .field(#name, &self.#ident) }
                }
            }
        });

        quote! {
            #cfg
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct(#name) #(#fields)* .finish()
                }
            }
        }
    }
}

fn gen_copy_clone(def: &TypeDef, name: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let cfg = cfg.gen(gen);

    if gen.sys || def.is_blittable() {
        quote! {
            #cfg
            impl ::core::marker::Copy for #name {}
            #cfg
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    } else if def.is_union() {
        quote! {
            #cfg
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
            #cfg
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self { #(#fields),* }
                }
            }
        }
    }
}

fn gen_struct_constants(def: &TypeDef, struct_name: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let cfg = cfg.gen(gen);

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
            #cfg
            impl #struct_name {
                #tokens
            }
        };
    }

    tokens
}
