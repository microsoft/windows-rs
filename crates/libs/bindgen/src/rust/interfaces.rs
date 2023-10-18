use super::*;

pub fn writer(writer: &Writer, def: TypeDef) -> TokenStream {
    if writer.sys {
        gen_sys_interface(def)
    } else {
        gen_win_interface(writer, def)
    }
}

fn gen_sys_interface(def: TypeDef) -> TokenStream {
    let name = def.name();
    let ident = to_ident(name);

    if type_def_is_exclusive(def) {
        quote! {}
    } else {
        quote! {
            pub type #ident = *mut ::core::ffi::c_void;
        }
    }
}

fn gen_win_interface(writer: &Writer, def: TypeDef) -> TokenStream {
    let generics = &type_def_generics(def);
    let ident = writer.type_def_name(def, generics);
    let is_exclusive = type_def_is_exclusive(def);
    let phantoms = writer.generic_phantoms(generics);
    let constraints = writer.generic_constraints(generics);
    let cfg = type_def_cfg(def, &[]);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);
    let interfaces = type_interfaces(&Type::TypeDef(def, generics.to_vec()));
    let vtables = type_def_vtables(def);
    let has_unknown_base = matches!(vtables.first(), Some(Type::IUnknown));

    let mut tokens = if is_exclusive {
        quote! { #[doc(hidden)] }
    } else {
        quote! { #doc }
    };

    if has_unknown_base {
        tokens.combine(&quote! {
            #features
            #[repr(transparent)]
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
            pub struct #ident(::windows_core::IUnknown, #phantoms) where #constraints;
        });
    } else {
        tokens.combine(&quote! {
            #features
            #[repr(transparent)]
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
            pub struct #ident(::std::ptr::NonNull<::std::ffi::c_void>);
        });
    }

    if !is_exclusive {
        let mut methods = quote! {};
        // We need to distinguish between public and virtual methods because some WinRT type hierarchies inherit colliding (overloaded)
        // methods that must be distinguishable.
        let method_names = &mut MethodNames::new();
        let virtual_names = &mut MethodNames::new();

        if def.flags().contains(TypeAttributes::WindowsRuntime) {
            for method in def.methods() {
                methods.combine(&winrt_methods::writer(writer, def, generics, InterfaceKind::Default, method, method_names, virtual_names));
            }
            for interface in &interfaces {
                if let Type::TypeDef(def, generics) = &interface.ty {
                    for method in def.methods() {
                        methods.combine(&winrt_methods::writer(writer, *def, generics, InterfaceKind::None, method, method_names, virtual_names));
                    }
                }
            }
        } else {
            let mut bases = vtables.len();
            for ty in &vtables {
                match ty {
                    Type::IUnknown | Type::IInspectable => {}
                    Type::TypeDef(def, _) => {
                        let kind = if def.type_name() == TypeName::IDispatch { InterfaceKind::None } else { InterfaceKind::Default };
                        for method in def.methods() {
                            methods.combine(&com_methods::writer(writer, *def, kind, method, method_names, virtual_names, bases));
                        }
                    }
                    rest => unimplemented!("{rest:?}"),
                }

                bases -= 1;
            }
            for method in def.methods() {
                methods.combine(&com_methods::writer(writer, def, InterfaceKind::Default, method, method_names, virtual_names, 0));
            }
        }

        tokens.combine(&quote! {
            #features
            impl<#constraints> #ident {
                #methods
            }
        });

        if !vtables.is_empty() && generics.is_empty() {
            let mut hierarchy = format!("::windows_core::imp::interface_hierarchy!({ident}");
            let mut hierarchy_cfg = cfg.clone();

            for ty in &vtables {
                let into = writer.type_name(ty);

                write!(&mut hierarchy, ", {into}").unwrap();
                hierarchy_cfg = hierarchy_cfg.union(&type_cfg(ty));
            }

            hierarchy.push_str(");");
            tokens.combine(&writer.cfg_features(&hierarchy_cfg));
            tokens.push_str(&hierarchy);
        } else {
            for ty in &vtables {
                let into = writer.type_name(ty);
                let cfg = writer.cfg_features(&cfg.union(&type_cfg(ty)));
                tokens.combine(&quote! {
                    #cfg
                    impl<#constraints> ::windows_core::CanInto<#into> for #ident {}
                });
            }
        }

        if def.flags().contains(TypeAttributes::WindowsRuntime) {
            for interface in &interfaces {
                let into = writer.type_name(&interface.ty);
                let cfg = writer.cfg_features(&cfg.union(&type_cfg(&interface.ty)));
                tokens.combine(&quote! {
                    #cfg
                    impl<#constraints> ::windows_core::CanTryInto<#into> for #ident {}
                });
            }
        }

        tokens.combine(&writer.interface_winrt_trait(def, generics, &ident, &constraints, &phantoms, &features));
        tokens.combine(&writer.async_get(def, generics, &ident, &constraints, &phantoms, &features));
        tokens.combine(&iterators::writer(writer, def, generics, &ident, &constraints, &phantoms, &cfg));
        tokens.combine(&writer.agile(def, &ident, &constraints, &features));
    }

    tokens.combine(&writer.interface_trait(def, generics, &ident, &constraints, &features, has_unknown_base));
    tokens.combine(&writer.interface_vtbl(def, generics, &ident, &constraints, &features));
    tokens
}
