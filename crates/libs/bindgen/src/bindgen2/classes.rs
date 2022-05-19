use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.sys {
        if gen.reader.type_def_has_default_interface(def) {
            let name = to_ident(gen.reader.type_def_name(def));
            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    } else {
        gen_class(gen, def)
    }
}

fn gen_class(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.reader.type_def_extends(def) == TypeName::Attribute {
        return TokenStream::new();
    }

    let name = to_ident(gen.reader.type_def_name(def));
    let interfaces = gen.reader.type_interfaces(&Type::TypeDef((def, Vec::new())));
    // TODO: faster if this were a Vec?
    let mut methods = quote! {};
    let mut method_names = MethodNames::new();

    let cfg = gen.reader.type_def_cfg(def, &[]);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    for interface in &interfaces {
        if let Type::TypeDef((def, generics)) = &interface.ty {
            if gen.min_xaml && interface.kind == InterfaceKind::Base && gen.namespace.starts_with("Windows.UI.Xaml") && !gen.reader.type_def_namespace(*def).starts_with("Windows.Foundation") {
                continue;
            }

            let mut virtual_names = MethodNames::new();

            for method in gen.reader.type_def_methods(*def) {
                methods.combine(&winrt_methods::gen(gen, *def, generics, interface.kind, method, &mut method_names, &mut virtual_names));
            }
        }
    }

    let factories = interfaces.iter().filter_map(|interface| match interface.kind {
        InterfaceKind::Static | InterfaceKind::Composable => {
            if let Type::TypeDef((def, _)) = &interface.ty {
                if gen.reader.type_def_methods(*def).next().is_some() {
                    let interface_type = gen.type_name(&interface.ty);
                    let features = gen.cfg_features(&gen.reader.type_cfg(&interface.ty));

                    let hidden = if gen.doc {
                        quote! { #[doc(hidden)] }
                    } else {
                        quote! {}
                    };

                    return Some(quote! {
                        #hidden
                        #features
                        pub fn #interface_type<R, F: FnOnce(&#interface_type) -> ::windows::core::Result<R>>(
                            callback: F,
                        ) -> ::windows::core::Result<R> {
                            static mut SHARED: ::windows::core::FactoryCache<#name, #interface_type> =
                                ::windows::core::FactoryCache::new();
                            unsafe { SHARED.call(callback) }
                        }
                    });
                }
            }
            None
        }
        _ => None,
    });

    if gen.reader.type_def_has_default_interface(def) {
        let new = if gen.reader.type_def_has_default_constructor(def) {
            quote! {
                pub fn new() -> ::windows::core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(
                    callback: F,
                ) -> ::windows::core::Result<R> {
                    static mut SHARED: ::windows::core::FactoryCache<#name, ::windows::core::IGenericFactory> =
                        ::windows::core::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
        } else {
            quote! {}
        };

        let mut tokens = quote! {
            #doc
            #features
            #[repr(transparent)]
            pub struct #name(::windows::core::IUnknown);
            #features
            impl #name {
                #new
                #methods
                #(#factories)*
            }
        };

        tokens.combine(&gen.interface_core_traits(def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&gen.interface_winrt_trait(def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&gen.interface_trait(def, &[], &name, &TokenStream::new(), &features));
        tokens.combine(&gen.runtime_name_trait(def, &[], &name, &TokenStream::new(), &features));
        tokens.combine(&gen.async_get(def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&iterators::gen(gen, def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&gen_conversions(gen, def, &name, &interfaces, &cfg));
        tokens.combine(&gen.agile(def, &name, &TokenStream::new(), &features));
        tokens
    } else {
        let mut tokens = quote! {
            #doc
            #features
            pub struct #name;
            #features
            impl #name {
                #methods
                #(#factories)*
            }
        };

        tokens.combine(&gen.runtime_name_trait(def, &[], &name, &TokenStream::new(), &features));
        tokens
    }
}

fn gen_conversions(gen: &Gen, def: TypeDef, name: &TokenStream, interfaces : &[Interface], cfg: &Cfg) -> TokenStream {
    let mut tokens = quote! {};

    for def in &[Type::IUnknown, Type::IInspectable] {
        let into = gen.type_name(def);
        let features = gen.cfg_features(cfg);
        tokens.combine(&quote! {
            #features
            impl ::core::convert::From<#name> for #into {
                fn from(value: #name) -> Self {
                    unsafe { ::core::mem::transmute(value) }
                }
            }
            #features
            impl ::core::convert::From<&#name> for #into {
                fn from(value: &#name) -> Self {
                    ::core::convert::From::from(::core::clone::Clone::clone(value))
                }
            }
            #features
            impl<'a> ::windows::core::IntoParam<'a, #into> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
                }
            }
            #features
            impl<'a> ::windows::core::IntoParam<'a, #into> for &'a #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
                }
            }
        });
    }

    for interface in interfaces {
        if gen.reader.type_is_exclusive(&interface.ty) {
            continue;
        }

        if interface.kind != InterfaceKind::Default && interface.kind != InterfaceKind::None && interface.kind != InterfaceKind::Base {
            continue;
        }

        let into = gen.type_name(&interface.ty);
        // TODO: simplify - maybe provide + operator?
        let features = gen.cfg_features(&cfg.union(&gen.reader.type_cfg(&interface.ty)));

        tokens.combine(&quote! {
            #features
            impl ::core::convert::TryFrom<#name> for #into {
                type Error = ::windows::core::Error;
                fn try_from(value: #name) -> ::windows::core::Result<Self> {
                    ::core::convert::TryFrom::try_from(&value)
                }
            }
            #features
            impl ::core::convert::TryFrom<&#name> for #into {
                type Error = ::windows::core::Error;
                fn try_from(value: &#name) -> ::windows::core::Result<Self> {
                    ::windows::core::Interface::cast(value)
                }
            }
            #features
            impl<'a> ::windows::core::IntoParam<'a, #into> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::IntoParam::into_param(&self)
                }
            }
            #features
            impl<'a> ::windows::core::IntoParam<'a, #into> for &#name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::core::convert::TryInto::<#into>::try_into(self)
                        .map(::windows::core::Param::Owned)
                        .unwrap_or(::windows::core::Param::None)
                }
            }
        });
    }

    for def in gen.reader.type_def_bases(def) {
        let into = gen.type_def_name(def, &[]);
        let features = gen.cfg_features(&cfg.union(&gen.reader.type_def_cfg(def, &[])));

        tokens.combine(&quote! {
            #features
            impl ::core::convert::From<#name> for #into {
                fn from(value: #name) -> Self {
                    ::core::convert::From::from(&value)
                }
            }
            #features
            impl ::core::convert::From<&#name> for #into {
                fn from(value: &#name) -> Self {
                    // This unwrap is legitimate because conversion to base can never fail because
                    // the base can never change across versions.
                    ::windows::core::Interface::cast(value).unwrap()
                }
            }
            #features
            impl<'a> ::windows::core::IntoParam<'a, #into> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::IntoParam::into_param(&self)
                }
            }
            #features
            impl<'a> ::windows::core::IntoParam<'a, #into> for &#name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Owned(::core::convert::Into::<#into>::into(self))
                }
            }
        });
    }

    tokens
}
