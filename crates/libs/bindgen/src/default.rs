use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    let kind = gen.reader.type_def_kind(def);

    match kind {
        TypeKind::Delegate if gen.reader.type_def_flags(def).winrt() => interface_traits(gen, def),
        TypeKind::Class if gen.reader.type_def_flags(def).winrt() && gen.reader.type_def_has_default_interface(def) => interface_traits(gen, def),
        TypeKind::Interface => interface_traits(gen, def),
        TypeKind::Enum => enum_traits(gen, def),
        TypeKind::Struct => struct_traits(gen, def),
        _ => quote! {},
    }
}

fn interface_traits(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = trim_tick(gen.reader.type_def_name(def));
    let generics: &Vec<Type> = &gen.reader.type_def_generics(def).collect();
    let constraints = gen.generic_constraints(generics);
    let ident = gen.type_def_name(def, generics);
    let cfg = gen.reader.type_def_cfg(def, generics);
    let features = gen.cfg_features(&cfg);

    let mut tokens = quote! {};

    if !gen.reader.type_def_is_exclusive(def) {
        tokens.combine(&quote! {
            #features
            impl<#constraints> ::core::cmp::PartialEq for #ident {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            #features
            impl<#constraints> ::core::cmp::Eq for #ident {}
            #features
            impl<#constraints> ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_tuple(#name).field(&self.0).finish()
                }
            }
        });
    }

    if !gen.reader.type_def_flags(def).winrt() {
        let vtables = gen.reader.type_def_vtables(def);
        let mut methods = quote! {};
        let mut bases = vtables.len();
        let method_names = &mut MethodNames::new();
        let virtual_names = &mut MethodNames::new();

        for ty in &vtables {
            match ty {
                Type::IUnknown | Type::IInspectable => {}
                Type::TypeDef((def, _)) => {
                    let kind = if gen.reader.type_def_type_name(*def) == TypeName::IDispatch { InterfaceKind::None } else { InterfaceKind::Default };
                    for method in gen.reader.type_def_methods(*def) {
                        methods.combine(&com_methods::gen(gen, *def, kind, method, method_names, virtual_names, bases));
                    }
                }
                _ => unimplemented!(),
            }

            bases -= 1;
        }

        if !methods.is_empty() {
            tokens.combine(&quote! {
                #features
                impl<#constraints> #ident {
                    #methods
                }
            });
        }
    }

    tokens
}

fn enum_traits(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = gen.reader.type_def_name(def);
    let ident = to_ident(name);
    let cfg = gen.reader.type_def_cfg(def, &[]);
    let features = gen.cfg_features(&cfg);

    let mut tokens = quote! {
        #features
        impl ::core::default::Default for #ident {
            fn default() -> Self {
                Self(0)
            }
        }
        #features
        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(#name).field(&self.0).finish()
            }
        }
    };

    if gen.reader.type_def_is_flags(def) {
        tokens.combine(&quote! {
            #features
            impl ::core::ops::BitOr for #ident {
                type Output = Self;

                fn bitor(self, other: Self) -> Self {
                    Self(self.0 | other.0)
                }
            }
            #features
            impl ::core::ops::BitAnd for #ident {
                type Output = Self;

                fn bitand(self, other: Self) -> Self {
                    Self(self.0 & other.0)
                }
            }
            #features
            impl ::core::ops::BitOrAssign for #ident {
                fn bitor_assign(&mut self, other: Self) {
                    self.0.bitor_assign(other.0)
                }
            }
            #features
            impl ::core::ops::BitAndAssign for #ident {
                fn bitand_assign(&mut self, other: Self) {
                    self.0.bitand_assign(other.0)
                }
            }
            #features
            impl ::core::ops::Not for #ident {
                type Output = Self;

                fn not(self) -> Self {
                    Self(self.0.not())
                }
            }
        });
    }

    tokens
}

fn struct_traits(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.reader.type_def_is_contract(def) || gen.reader.type_def_is_handle(def) {
        return quote! {};
    }

    struct_traits_with_name(gen, def, gen.reader.type_def_name(def), &Cfg::default())
}

fn struct_traits_with_name(gen: &Gen, def: TypeDef, name: &str, cfg: &Cfg) -> TokenStream {
    if gen.reader.type_def_fields(def).next().is_none() {
        return quote! {};
    }

    let ident = to_ident(name);
    let cfg = cfg.union(&gen.reader.type_def_cfg(def, &[]));
    let features = gen.cfg_features(&cfg);

    let mut tokens = quote! {
        #features
        impl ::core::default::Default for #ident {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
    };

    if !gen.reader.type_def_has_explicit_layout(def) && !gen.reader.type_def_has_packing(def) && !gen.reader.type_def_has_callback(def) {
        let fields = gen.reader.type_def_fields(def).filter_map(|f| {
            let name = to_ident(gen.reader.field_name(f));
            if gen.reader.field_flags(f).literal() {
                None
            } else {
                Some(quote! { self.#name == other.#name })
            }
        });

        tokens.combine(&quote! {
            #features
            impl ::core::cmp::PartialEq for #ident {
                fn eq(&self, other: &Self) -> bool {
                    #(#fields)&&*
                }
            }
            #features
            impl ::core::cmp::Eq for #ident {}
        });

        let fields = gen.reader.type_def_fields(def).filter_map(|f| {
            if gen.reader.field_flags(f).literal() {
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

        tokens.combine(&quote! {
            #features
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct(#name) #(#fields)* .finish()
                }
            }
        });

        for (index, nested_type) in gen.reader.nested_types(def).enumerate() {
            let nested_name = format!("{name}_{index}");
            tokens.combine(&struct_traits_with_name(gen, nested_type, &nested_name, &cfg));
        }
    }

    tokens
}
