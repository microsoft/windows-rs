use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.sys {
        gen_sys_interface(gen, def)
    } else {
        gen_win_interface(gen, def)
    }
}

fn gen_sys_interface(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = gen.reader.type_def_name(def);
    let ident = to_ident(name);

    if gen.reader.type_def_is_exclusive(def) {
        quote! {}
    } else {
        quote! {
            pub type #ident = *mut ::core::ffi::c_void;
        }
    }
}

fn gen_win_interface(gen: &Gen, def: TypeDef) -> TokenStream {
    let generics: &Vec<Type> = &gen.reader.type_def_generics(def).collect();
    let ident = gen.type_def_name(def, generics);
    let is_exclusive = gen.reader.type_def_is_exclusive(def);
    let phantoms = gen.generic_phantoms(generics);
    let constraints = gen.generic_constraints(generics);
    let cfg = gen.reader.type_def_cfg(def, &[]);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);
    let interfaces = gen.reader.type_interfaces(&Type::TypeDef((def, generics.to_vec()))); // TODO: how to avoid copy?
    let vtables = gen.reader.type_def_vtables(def);
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
            pub struct #ident(::windows::core::IUnknown, #phantoms) where #constraints;
        });
    } else {
        tokens.combine(&quote! {
            #features
            #[repr(transparent)]
            pub struct #ident(::std::ptr::NonNull<::std::ffi::c_void>);
            #features
            unsafe impl ::windows::core::Abi for Option<#ident> {
                type Abi = *mut ::std::ffi::c_void;
            }
            #features
            unsafe impl ::windows::core::Abi for #ident {
                type Abi = *mut ::std::ffi::c_void;

                fn abi_is_possibly_valid(abi: &Self::Abi) -> bool {
                    !abi.is_null()
                }
            }
        });
    }

    if !is_exclusive {
        let mut methods = quote! {};
        // TODO: why do we need to distinguish between public and virtual methods?
        let method_names = &mut MethodNames::new();
        let virtual_names = &mut MethodNames::new();

        if gen.reader.type_def_flags(def).winrt() {
            for method in gen.reader.type_def_methods(def) {
                methods.combine(&winrt_methods::gen(gen, def, generics, InterfaceKind::Default, method, method_names, virtual_names));
            }
            for interface in &interfaces {
                if let Type::TypeDef((def, generics)) = &interface.ty {
                    for method in gen.reader.type_def_methods(*def) {
                        methods.combine(&winrt_methods::gen(gen, *def, generics, InterfaceKind::None, method, method_names, virtual_names));
                    }
                }
            }
        } else {
            let mut bases = vtables.len();
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
            for method in gen.reader.type_def_methods(def) {
                methods.combine(&com_methods::gen(gen, def, InterfaceKind::Default, method, method_names, virtual_names, 0));
            }
        }

        tokens.combine(&quote! {
            #features
            impl<#constraints> #ident {
                #methods
            }
        });

        if !vtables.is_empty() && generics.is_empty() {
            let mut hierarchy = format!("::windows::core::interface_hierarchy!({}", ident);
            let mut hierarchy_cfg = cfg.clone();

            for ty in &vtables {
                let into = gen.type_name(ty);

                write!(&mut hierarchy, ", {}", into).unwrap();
                hierarchy_cfg = hierarchy_cfg.union(&gen.reader.type_cfg(ty));
            }

            hierarchy.push_str(");");
            tokens.combine(&gen.cfg_features(&hierarchy_cfg));
            tokens.push_str(&hierarchy);
        } else {
            for ty in &vtables {
                let into = gen.type_name(ty);
                let cfg = gen.cfg_features(&cfg.union(&gen.reader.type_cfg(ty)));
                tokens.combine(&quote! {
                    #cfg
                    impl<#constraints> ::core::convert::From<#ident> for #into {
                        fn from(value: #ident) -> Self {
                            unsafe { ::core::mem::transmute(value) }
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::core::convert::From<&'a #ident> for &'a #into {
                        fn from(value: &'a #ident) -> Self {
                            unsafe { ::core::mem::transmute(value) }
                        }
                    }
                    #cfg
                    impl<#constraints> ::core::convert::From<&#ident> for #into {
                        fn from(value: &#ident) -> Self {
                            ::core::convert::From::from(::core::clone::Clone::clone(value))
                        }
                    }
                });
            }
        }

        if gen.reader.type_def_flags(def).winrt() {
            for interface in &interfaces {
                let into = gen.type_name(&interface.ty);
                let cfg = gen.cfg_features(&cfg.union(&gen.reader.type_cfg(&interface.ty)));
                tokens.combine(&quote! {
                    #cfg
                    impl<#constraints> ::core::convert::TryFrom<#ident> for #into {
                        type Error = ::windows::core::Error;
                        fn try_from(value: #ident) -> ::windows::core::Result<Self> {
                            ::core::convert::TryFrom::try_from(&value)
                        }
                    }
                    #cfg
                    impl<#constraints> ::core::convert::TryFrom<&#ident> for #into {
                        type Error = ::windows::core::Error;
                        fn try_from(value: &#ident) -> ::windows::core::Result<Self> {
                            ::windows::core::Interface::cast(value)
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::core::convert::TryFrom<&#ident> for ::windows::core::InParam<'a, #into> {
                        type Error = ::windows::core::Error;
                        fn try_from(value: &#ident) -> ::windows::core::Result<Self> {
                            let item = ::std::convert::TryInto::try_into(value)?;
                            Ok(::windows::core::InParam::owned(item))
                        }
                    }
                });
            }
        }

        tokens.combine(&gen.interface_core_traits(def, generics, &ident, &constraints, &phantoms, &features));
        tokens.combine(&gen.interface_winrt_trait(def, generics, &ident, &constraints, &phantoms, &features));
        tokens.combine(&gen.async_get(def, generics, &ident, &constraints, &phantoms, &features));
        tokens.combine(&iterators::gen(gen, def, generics, &ident, &constraints, &phantoms, &cfg));
        tokens.combine(&gen.agile(def, &ident, &constraints, &features));
    }

    tokens.combine(&gen.interface_trait(def, generics, &ident, &constraints, &features, has_unknown_base));
    tokens.combine(&gen.interface_vtbl(def, generics, &ident, &constraints, &features));
    tokens
}
