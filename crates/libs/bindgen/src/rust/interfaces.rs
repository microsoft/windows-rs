use super::*;

pub fn writer(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    if writer.sys {
        gen_sys_interface(writer, def)
    } else {
        gen_win_interface(writer, def)
    }
}

fn gen_sys_interface(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    if !writer.vtbl {
        return quote! {};
    }

    let vtables = metadata::type_def_vtables(def);
    let has_unknown_base = matches!(vtables.first(), Some(metadata::Type::IUnknown));

    let mut tokens = quote! {};

    if has_unknown_base {
        let iid_ident: TokenStream = format!("IID_{}", def.name()).into();
        let iid = writer.guid_literal(metadata::type_def_guid(def));

        tokens.combine(&quote! {
            pub const #iid_ident: GUID = GUID::from_u128(#iid);
        });
    }

    tokens.combine(&writer.interface_vtbl(def, &[], &TokenStream::new(), &TokenStream::new()));
    tokens
}

fn gen_win_interface(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    let generics = &metadata::type_def_generics(def);
    let ident = writer.type_def_name(def, generics);
    let vtbl_ident = writer.type_def_vtbl_name(def, generics);
    let is_exclusive = metadata::type_def_is_exclusive(def);
    let phantoms = writer.generic_phantoms(generics);
    let constraints = writer.generic_constraints(generics);
    let cfg = cfg::type_def_cfg(writer, def, &[]);
    let features = writer.cfg_features(&cfg);
    let interfaces = metadata::type_interfaces(&metadata::Type::TypeDef(def, generics.to_vec()));
    let vtables = metadata::type_def_vtables(def);
    let has_unknown_base = matches!(vtables.first(), Some(metadata::Type::IUnknown));

    let mut tokens = quote! {};

    if has_unknown_base {
        if generics.is_empty() {
            let iid = writer.guid_literal(metadata::type_def_guid(def));
            tokens.combine(&quote! {
                #features
                windows_core::imp::define_interface!(#ident, #vtbl_ident, #iid);
            });
        } else {
            tokens.combine(&quote! {
                #features
                #[repr(transparent)]
                #[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
                pub struct #ident(windows_core::IUnknown, #phantoms) where #constraints;
            });
        }
    } else {
        tokens.combine(&quote! {
            #features
            windows_core::imp::define_interface!(#ident, #vtbl_ident);
        });
    }

    if !is_exclusive {
        let mut methods = quote! {};
        // We need to distinguish between public and virtual methods because some WinRT type hierarchies inherit colliding (overloaded)
        // methods that must be distinguishable.
        let method_names = &mut MethodNames::new();
        let virtual_names = &mut MethodNames::new();

        if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
            for method in def.methods() {
                methods.combine(&winrt_methods::writer(writer, def, generics, metadata::InterfaceKind::Default, method, method_names, virtual_names));
            }
            for interface in &interfaces {
                if let metadata::Type::TypeDef(def, generics) = &interface.ty {
                    for method in def.methods() {
                        methods.combine(&winrt_methods::writer(writer, *def, generics, metadata::InterfaceKind::None, method, method_names, virtual_names));
                    }
                }
            }
        } else {
            for method in def.methods() {
                methods.combine(&com_methods::writer(writer, def, metadata::InterfaceKind::Default, method, method_names, virtual_names));
            }
        }

        if let Some(base) = vtables.last() {
            let base = writer.type_name(base);

            tokens.combine(&quote! {
                #features
                impl<#constraints> std::ops::Deref for #ident {
                    type Target = #base;
                    fn deref(&self) -> &Self::Target {
                        unsafe { std::mem::transmute(self) }
                    }
                }
            });
        }

        if !vtables.is_empty() && generics.is_empty() {
            let mut hierarchy = format!("windows_core::imp::interface_hierarchy!({ident}");
            let mut hierarchy_cfg = cfg.clone();

            for ty in &vtables {
                let into = writer.type_name(ty);

                write!(&mut hierarchy, ", {into}").unwrap();
                hierarchy_cfg = hierarchy_cfg.union(cfg::type_cfg(writer, ty));
            }

            hierarchy.push_str(");");
            tokens.combine(&writer.cfg_features(&hierarchy_cfg));
            tokens.push_str(&hierarchy);
        } else {
            for ty in &vtables {
                let into = writer.type_name(ty);
                let cfg = writer.cfg_features(&cfg.union(cfg::type_cfg(writer, ty)));
                tokens.combine(&quote! {
                    #cfg
                    impl<#constraints> windows_core::CanInto<#into> for #ident {}
                });
            }
        }

        if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) && !interfaces.is_empty() {
            if generics.is_empty() {
                let mut hierarchy = format!("windows_core::imp::required_hierarchy!({ident}");
                let mut hierarchy_cfg = cfg.clone();

                for interface in &interfaces {
                    let into = writer.type_name(&interface.ty);

                    write!(&mut hierarchy, ", {into}").unwrap();
                    hierarchy_cfg = hierarchy_cfg.union(cfg::type_cfg(writer, &interface.ty));
                }

                hierarchy.push_str(");");
                tokens.combine(&writer.cfg_features(&hierarchy_cfg));
                tokens.push_str(&hierarchy);
            } else {
                for interface in &interfaces {
                    let into = writer.type_name(&interface.ty);
                    let cfg = writer.cfg_features(&cfg.union(cfg::type_cfg(writer, &interface.ty)));
                    tokens.combine(&quote! {
                        #cfg
                        impl<#constraints> windows_core::CanInto<#into> for #ident { const QUERY: bool = true; }
                    });
                }
            }
        }

        tokens.combine(&quote! {
            #features
            impl<#constraints> #ident {
                #methods
            }
        });

        tokens.combine(&writer.async_get(def, generics, &ident, &constraints, &phantoms, &features));
        tokens.combine(&iterators::writer(writer, def, generics, &ident, &constraints, &phantoms, &cfg));
        tokens.combine(&writer.agile(def, &ident, &constraints, &features));
    }

    tokens.combine(&writer.interface_winrt_trait(def, generics, &ident, &constraints, &phantoms, &features));
    tokens.combine(&writer.interface_trait(def, generics, &ident, &constraints, &features, has_unknown_base));
    tokens.combine(&writer.interface_vtbl(def, generics, &constraints, &features));
    tokens
}
