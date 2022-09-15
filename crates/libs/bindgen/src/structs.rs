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

    let repr = if let Some(layout) = gen.reader.type_def_class_layout(def) {
        let packing = Literal::usize_unsuffixed(gen.reader.class_layout_packing_size(layout));
        quote! { #[repr(C, packed(#packing))] }
    } else {
        quote! { #[repr(C)] }
    };

    let fields = gen.reader.type_def_fields(def).map(|f| {
        let name = to_ident(gen.reader.field_name(f));
        let ty = gen.reader.field_type(f, Some(def));
        let ty = gen.type_default_name(&ty);

        if gen.reader.field_flags(f).literal() {
            quote! {}
        } else if !gen.sys && flags.explicit_layout() && !gen.reader.field_is_blittable(f, def) {
            quote! { pub #name: ::core::mem::ManuallyDrop<#ty>, }
        } else {
            quote! { pub #name: #ty, }
        }
    });

    let struct_or_union = if flags.explicit_layout() {
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

        tokens.combine(&extensions::gen(gen.reader.type_def_type_name(def)));
    }

    for (index, nested_type) in gen.reader.nested_types(def).enumerate() {
        let nested_name = format!("{}_{}", struct_name, index);
        tokens.combine(&gen_struct_with_name(gen, nested_type, &nested_name, &cfg));
    }

    tokens
}

fn gen_windows_traits(gen: &Gen, def: TypeDef, name: &TokenStream, cfg: &Cfg) -> TokenStream {
    if gen.sys {
        quote! {}
    } else {
        let abi = if gen.reader.type_def_is_blittable(def) {
            quote! { Self }
        } else {
            quote! { ::core::mem::ManuallyDrop<Self> }
        };

        let features = gen.cfg_features(cfg);

        let mut tokens = quote! {
            #features
            unsafe impl ::windows::core::Abi for #name {
                type Abi = #abi;
            }
        };

        if gen.reader.type_def_flags(def).winrt() {
            let signature = Literal::byte_string(gen.reader.type_def_signature(def, &[]).as_bytes());

            let clone = if gen.reader.type_def_is_blittable(def) {
                quote! { *from }
            } else {
                quote! { from.clone() }
            };

            tokens.combine(&quote! {
                #features
                unsafe impl ::windows::core::RuntimeType for #name {
                    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(#signature);
                    type DefaultType = Self;
                    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
                        Ok(#clone)
                    }
                }
            });
        }

        tokens
    }
}

fn gen_compare_traits(gen: &Gen, def: TypeDef, name: &TokenStream, cfg: &Cfg) -> TokenStream {
    let features = gen.cfg_features(cfg);

    if gen.sys {
        quote! {}
    } else if gen.reader.type_def_is_blittable(def) || gen.reader.type_def_flags(def).explicit_layout() || gen.reader.type_def_class_layout(def).is_some() {
        quote! {
            #features
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    unsafe {
                        ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<#name>()) == 0
                    }
                }
            }
            #features
            impl ::core::cmp::Eq for #name {}
        }
    } else {
        let fields = gen.reader.type_def_fields(def).map(|f| {
            let name = to_ident(gen.reader.field_name(f));
            if gen.reader.field_flags(f).literal() {
                quote! {}
            } else {
                let ty = gen.reader.field_type(f, Some(def));
                if gen.reader.type_is_callback(&ty) {
                    quote! {
                        self.#name.map(|f| f as usize) == other.#name.map(|f| f as usize)
                    }
                } else {
                    quote! { self.#name == other.#name }
                }
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
    if gen.sys || gen.reader.type_def_has_explicit_layout(def) || gen.reader.type_def_has_packing(def) {
        quote! {}
    } else {
        let name = ident.as_str();
        let features = gen.cfg_features(cfg);

        let fields = gen.reader.type_def_fields(def).map(|f| {
            if gen.reader.field_flags(f).literal() {
                quote! {}
            } else {
                let name = gen.reader.field_name(f);
                let ident = to_ident(name);
                let ty = gen.reader.field_type(f, Some(def));
                if !ty.is_pointer() && gen.reader.type_is_callback(&ty) {
                    quote! { .field(#name, &self.#ident.map(|f| f as usize)) }
                } else if gen.reader.type_is_callback_array(&ty) {
                    quote! {}
                } else {
                    quote! { .field(#name, &self.#ident) }
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

    if gen.sys || gen.reader.type_def_is_blittable(def) {
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
    } else if gen.reader.type_def_flags(def).explicit_layout() {
        quote! {
            #features
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    unsafe { ::core::mem::transmute_copy(self) }
                }
            }
        }
    } else if gen.reader.type_def_class_layout(def).is_some() {
        // Don't support copy/clone of packed structs: https://github.com/rust-lang/rust/issues/82523
        quote! {}
    } else {
        let fields = gen.reader.type_def_fields(def).map(|f| {
            let name = to_ident(gen.reader.field_name(f));
            if gen.reader.field_flags(f).literal() {
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

fn gen_struct_constants(gen: &Gen, def: TypeDef, struct_name: &TokenStream, cfg: &Cfg) -> TokenStream {
    let features = gen.cfg_features(cfg);

    let constants = gen.reader.type_def_fields(def).filter_map(|f| {
        if gen.reader.field_flags(f).literal() {
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
