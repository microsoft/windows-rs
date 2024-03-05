use super::*;
use metadata::HasAttributes;

pub fn writer(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    if !def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
        return quote! {};
    }

    if writer.sys {
        if def.interface_impls().next().is_some() {
            let name = to_ident(def.name());
            quote! {
                pub type #name = *mut core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    } else {
        gen_class(writer, def)
    }
}

fn gen_class(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    if def.extends() == Some(metadata::TypeName::Attribute) {
        return TokenStream::new();
    }

    let name = to_ident(def.name());
    let interfaces = metadata::type_interfaces(&metadata::Type::TypeDef(def, Vec::new()));
    let mut methods = quote! {};
    let mut method_names = MethodNames::new();

    let cfg = cfg::type_def_cfg(writer, def, &[]);
    let features = writer.cfg_features(&cfg);

    for interface in &interfaces {
        if let metadata::Type::TypeDef(def, generics) = &interface.ty {
            let mut virtual_names = MethodNames::new();

            for method in def.methods() {
                methods.combine(&winrt_methods::writer(writer, *def, generics, interface.kind, method, &mut method_names, &mut virtual_names));
            }
        }
    }

    let factories = interfaces.iter().filter_map(|interface| match interface.kind {
        metadata::InterfaceKind::Static => {
            if let metadata::Type::TypeDef(def, generics) = &interface.ty {
                if def.methods().next().is_some() {
                    let interface_type = writer.type_name(&interface.ty);
                    let features = writer.cfg_features(&cfg::type_def_cfg(writer, *def, generics));

                    return Some(quote! {
                        #[doc(hidden)]
                        #features
                        pub fn #interface_type<R, F: FnOnce(&#interface_type) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<#name, #interface_type> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                    });
                }
            }
            None
        }
        _ => None,
    });

    if def.interface_impls().next().is_some() {
        let new = if type_def_has_default_constructor(def) {
            quote! {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<#name, windows_core::imp::IGenericFactory> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
        } else {
            quote! {}
        };

        let mut tokens = quote! {
            #features
            #[repr(transparent)]
            #[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
            pub struct #name(windows_core::IUnknown);
        };

        tokens.combine(&gen_conversions(writer, def, &name, &interfaces, &cfg));

        tokens.combine(&quote! {
            #features
            impl #name {
                #new
                #methods
                #(#factories)*
            }
        });

        tokens.combine(&writer.interface_winrt_trait(def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&writer.interface_trait(def, &[], &name, &TokenStream::new(), &features, true));
        tokens.combine(&writer.runtime_name_trait(def, &[], &name, &TokenStream::new(), &features));
        tokens.combine(&writer.async_get(def, &[], &name, &TokenStream::new(), &TokenStream::new(), &features));
        tokens.combine(&iterators::writer(writer, def, &[], &name, &TokenStream::new(), &TokenStream::new(), &cfg));
        tokens.combine(&writer.agile(def, &name, &TokenStream::new(), &features));
        tokens
    } else {
        let mut tokens = quote! {
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

fn gen_conversions(writer: &Writer, def: metadata::TypeDef, ident: &TokenStream, interfaces: &[metadata::Interface], cfg: &cfg::Cfg) -> TokenStream {
    let features = writer.cfg_features(cfg);
    let mut tokens = quote! {
        #features
        windows_core::imp::interface_hierarchy!(#ident, windows_core::IUnknown, windows_core::IInspectable);
    };

    let mut hierarchy = format!("windows_core::imp::required_hierarchy!({ident}");
    let mut hierarchy_cfg = cfg.clone();
    let mut hierarchy_added = false;

    for interface in interfaces {
        if type_is_exclusive(&interface.ty) {
            continue;
        }

        if interface.kind != metadata::InterfaceKind::Default && interface.kind != metadata::InterfaceKind::None && interface.kind != metadata::InterfaceKind::Base {
            continue;
        }

        let into = writer.type_name(&interface.ty);
        write!(&mut hierarchy, ", {into}").unwrap();
        hierarchy_cfg = hierarchy_cfg.union(&cfg::type_cfg(writer, &interface.ty));
        hierarchy_added = true;
    }

    for def in metadata::type_def_bases(def) {
        let into = writer.type_def_name(def, &[]);
        write!(&mut hierarchy, ", {into}").unwrap();
        hierarchy_cfg = hierarchy_cfg.union(&cfg::type_def_cfg(writer, def, &[]));
        hierarchy_added = true;
    }

    if hierarchy_added {
        hierarchy.push_str(");");
        tokens.combine(&writer.cfg_features(&hierarchy_cfg));
        tokens.push_str(&hierarchy);
    }

    tokens
}

fn type_def_has_default_constructor(row: metadata::TypeDef) -> bool {
    for attribute in row.attributes() {
        if attribute.name() == "ActivatableAttribute" {
            if attribute.args().iter().any(|arg| matches!(arg.1, metadata::Value::TypeName(_))) {
                continue;
            } else {
                return true;
            }
        }
    }
    false
}

fn type_is_exclusive(ty: &metadata::Type) -> bool {
    match ty {
        metadata::Type::TypeDef(row, _) => metadata::type_def_is_exclusive(*row),
        _ => false,
    }
}
