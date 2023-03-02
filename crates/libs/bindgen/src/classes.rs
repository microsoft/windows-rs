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
    let interfaces = gen
        .reader
        .type_interfaces(&Type::TypeDef((def, Vec::new())));
    let mut methods = quote! {};
    let mut method_names = MethodNames::new();

    let cfg = gen.reader.type_def_cfg(def, &[]);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    for interface in &interfaces {
        if let Type::TypeDef((def, generics)) = &interface.ty {
            let mut virtual_names = MethodNames::new();

            for method in gen.reader.type_def_methods(*def) {
                methods.combine(&winrt_methods::gen(
                    gen,
                    *def,
                    generics,
                    interface.kind,
                    method,
                    &mut method_names,
                    &mut virtual_names,
                ));
            }
        }
    }

    let factories = interfaces.iter().filter_map(|interface| match interface.kind {
        InterfaceKind::Static => {
            if let Type::TypeDef((def, generics)) = &interface.ty {
                if gen.reader.type_def_methods(*def).next().is_some() {
                    let interface_type = gen.type_name(&interface.ty);
                    let features = gen.cfg_features(&gen.reader.type_def_cfg(*def, generics));

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
                            static SHARED: ::windows::imp::FactoryCache<#name, #interface_type> =
                                ::windows::imp::FactoryCache::new();
                            SHARED.call(callback)
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
                fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(
                    callback: F,
                ) -> ::windows::core::Result<R> {
                    static SHARED: ::windows::imp::FactoryCache<#name, ::windows::imp::IGenericFactory> =
                        ::windows::imp::FactoryCache::new();
                    SHARED.call(callback)
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

        tokens.combine(&gen.interface_core_traits(
            def,
            &[],
            &name,
            &TokenStream::new(),
            &TokenStream::new(),
            &features,
        ));
        tokens.combine(&gen.interface_winrt_trait(
            def,
            &[],
            &name,
            &TokenStream::new(),
            &TokenStream::new(),
            &features,
        ));
        tokens.combine(&gen.interface_trait(def, &[], &name, &TokenStream::new(), &features, true));
        tokens.combine(&gen.runtime_name_trait(def, &[], &name, &TokenStream::new(), &features));
        tokens.combine(&gen.async_get(
            def,
            &[],
            &name,
            &TokenStream::new(),
            &TokenStream::new(),
            &features,
        ));
        tokens.combine(&iterators::gen(
            gen,
            def,
            &[],
            &name,
            &TokenStream::new(),
            &TokenStream::new(),
            &cfg,
        ));
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

fn gen_conversions(
    gen: &Gen,
    def: TypeDef,
    name: &TokenStream,
    interfaces: &[Interface],
    cfg: &Cfg,
) -> TokenStream {
    let features = gen.cfg_features(cfg);
    let mut tokens = quote! {
        #features
        ::windows::imp::interface_hierarchy!(#name, ::windows::core::IUnknown, ::windows::core::IInspectable);
    };

    for interface in interfaces {
        if gen.reader.type_is_exclusive(&interface.ty) {
            continue;
        }

        if interface.kind != InterfaceKind::Default
            && interface.kind != InterfaceKind::None
            && interface.kind != InterfaceKind::Base
        {
            continue;
        }

        let into = gen.type_name(&interface.ty);
        // TODO: simplify - maybe provide + operator?
        let features = gen.cfg_features(&cfg.union(&gen.reader.type_cfg(&interface.ty)));

        tokens.combine(&quote! {
            #features
            impl ::windows::core::CanTryInto<#into> for #name {}
        });
    }

    for def in gen.reader.type_def_bases(def) {
        let into = gen.type_def_name(def, &[]);
        let features = gen.cfg_features(&cfg.union(&gen.reader.type_def_cfg(def, &[])));

        tokens.combine(&quote! {
            #features
            impl ::windows::core::CanTryInto<#into> for #name {}
        });
    }

    tokens
}
