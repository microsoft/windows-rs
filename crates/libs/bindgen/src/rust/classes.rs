use super::*;

pub fn writer(writer: &Writer, def: TypeDef) -> TokenStream {
    if writer.sys {
        if type_def_has_default_interface(writer.reader, def) {
            let name = to_ident(writer.reader.type_def_name(def));
            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    } else {
        gen_class(writer, def)
    }
}

fn gen_class(writer: &Writer, def: TypeDef) -> TokenStream {
    if writer.reader.type_def_extends(def) == Some(TypeName::Attribute) {
        return TokenStream::new();
    }

    let name = to_ident(writer.reader.type_def_name(def));
    let interfaces = type_interfaces(writer.reader, &Type::TypeDef(def, Vec::new()));
    let mut methods = quote! {};
    let mut method_names = MethodNames::new();

    let cfg = type_def_cfg(writer.reader, def, &[]);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);

    for interface in &interfaces {
        if let Type::TypeDef(def, generics) = &interface.ty {
            let mut virtual_names = MethodNames::new();

            for method in writer.reader.type_def_methods(*def) {
                methods.combine(&winrt_methods::writer(writer, *def, generics, interface.kind, method, &mut method_names, &mut virtual_names));
            }
        }
    }

    let factories = interfaces.iter().filter_map(|interface| match interface.kind {
        InterfaceKind::Static => {
            if let Type::TypeDef(def, generics) = &interface.ty {
                if writer.reader.type_def_methods(*def).next().is_some() {
                    let interface_type = writer.type_name(&interface.ty);
                    let features = writer.cfg_features(&type_def_cfg(writer.reader, *def, generics));

                    return Some(quote! {
                        #[doc(hidden)]
                        #features
                        pub fn #interface_type<R, F: FnOnce(&#interface_type) -> ::windows_core::Result<R>>(
                            callback: F,
                        ) -> ::windows_core::Result<R> {
                            static SHARED: ::windows_core::imp::FactoryCache<#name, #interface_type> =
                                ::windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                    });
                }
            }
            None
        }
        _ => None,
    });

    if type_def_has_default_interface(writer.reader, def) {
        let new = if type_def_has_default_constructor(writer.reader, def) {
            quote! {
                pub fn new() -> ::windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(
                    callback: F,
                ) -> ::windows_core::Result<R> {
                    static SHARED: ::windows_core::imp::FactoryCache<#name, ::windows_core::imp::IGenericFactory> =
                        ::windows_core::imp::FactoryCache::new();
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
            pub struct #name(::windows_core::IUnknown);
            #features
            impl #name {
                #new
                #methods
                #(#factories)*
            }
        };

        tokens.combine(&writer.interface_core_traits(def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&writer.interface_winrt_trait(def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&writer.interface_trait(def, &[], &name, &TokenStream::new(), &features, true));
        tokens.combine(&writer.runtime_name_trait(def, &[], &name, &TokenStream::new(), &features));
        tokens.combine(&writer.async_get(def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&iterators::writer(writer, def, &[], &name, &TokenStream::new(), &TokenStream::new(), &cfg));
        tokens.combine(&gen_conversions(writer, def, &name, &interfaces, &cfg));
        tokens.combine(&writer.agile(def, &name, &TokenStream::new(), &features));
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

        tokens.combine(&writer.runtime_name_trait(def, &[], &name, &TokenStream::new(), &features));
        tokens
    }
}

fn gen_conversions(writer: &Writer, def: TypeDef, name: &TokenStream, interfaces: &[Interface], cfg: &Cfg) -> TokenStream {
    let features = writer.cfg_features(cfg);
    let mut tokens = quote! {
        #features
        ::windows_core::imp::interface_hierarchy!(#name, ::windows_core::IUnknown, ::windows_core::IInspectable);
    };

    for interface in interfaces {
        if type_is_exclusive(writer.reader, &interface.ty) {
            continue;
        }

        if interface.kind != InterfaceKind::Default && interface.kind != InterfaceKind::None && interface.kind != InterfaceKind::Base {
            continue;
        }

        let into = writer.type_name(&interface.ty);
        let features = writer.cfg_features(&cfg.union(&type_cfg(writer.reader, &interface.ty)));

        tokens.combine(&quote! {
            #features
            impl ::windows_core::CanTryInto<#into> for #name {}
        });
    }

    for def in type_def_bases(writer.reader, def) {
        let into = writer.type_def_name(def, &[]);
        let features = writer.cfg_features(&cfg.union(&type_def_cfg(writer.reader, def, &[])));

        tokens.combine(&quote! {
            #features
            impl ::windows_core::CanTryInto<#into> for #name {}
        });
    }

    tokens
}
