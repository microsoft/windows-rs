use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        if def.default_interface().is_some() {
            let name = gen_type_ident(def, gen);
            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    } else {
        gen_class(def, gen)
    }
}

fn gen_class(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let has_default = def.default_interface().is_some();
    let interfaces = def.class_interfaces();
    let mut methods = quote! {};
    let mut method_names = BTreeMap::<String, u32>::new();

    for (def, kind) in &interfaces {
        let mut vtable_offset = 6;
        for method in def.methods() {
            methods.combine(&gen_winrt_method(&def, *kind, &method, vtable_offset, &mut method_names, gen));
            vtable_offset += 1;
        }
    }

    let factories = interfaces.iter().filter_map(|(def, kind)| match kind {
        InterfaceKind::Static | InterfaceKind::Composable => {
            if def.methods().next().is_some() {
                let interface_name = format_token!("{}", def.name());
                let interface_type = gen_type_name(&def, gen);

                Some(quote! {
                    pub fn #interface_name<R, F: FnOnce(&#interface_type) -> ::windows::core::Result<R>>(
                        callback: F,
                    ) -> ::windows::core::Result<R> {
                        static mut SHARED: ::windows::core::FactoryCache<#name, #interface_type> =
                            ::windows::core::FactoryCache::new();
                        unsafe { SHARED.call(callback) }
                    }
                })
            } else {
                None
            }
        }
        _ => None,
    });

    if has_default {
        let new = if def.has_default_constructor() {
            quote! {
                pub fn new() -> ::windows::core::Result<Self> {
                    Self::IActivationFactory(|f| f.activate_instance::<Self>())
                }
                fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(
                    callback: F,
                ) -> ::windows::core::Result<R> {
                    static mut SHARED: ::windows::core::FactoryCache<#name, ::windows::core::IActivationFactory> =
                        ::windows::core::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
        } else {
            quote! {}
        };

        let cfg = gen.type_cfg(&def);

        let mut tokens = quote! {
            #cfg
            #[repr(transparent)]
            pub struct #name(::windows::core::IUnknown);
            #cfg
            impl #name {
                #new
                #methods
                #(#factories)*
            }
        };

        tokens.combine(&gen_std_traits(def, &cfg, gen));
        tokens.combine(&gen_runtime_trait(def, &cfg, gen));    
        tokens.combine(&gen_interface_trait(def, &cfg, gen));
        tokens.combine(&gen_runtime_name(def, &cfg, gen));
        tokens.combine(&gen_async(def,&cfg,gen));
        tokens
    } else {
        let mut tokens = quote! {
            pub struct #name {}
            impl #name {
                #methods
                #(#factories)*
            }
        };

        tokens.combine(&gen_runtime_name(def, &quote!{}, gen));
        tokens
    }
}

fn gen_runtime_name(def: &TypeDef, cfg: &TokenStream, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let runtime_name = format!("{}", def.type_name());

    quote! {
        #cfg
        impl ::windows::core::RuntimeName for #name {
            const NAME: &'static str = #runtime_name;
        }
    }
}
