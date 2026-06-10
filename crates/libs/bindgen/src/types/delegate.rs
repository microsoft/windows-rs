use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Delegate {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}

impl Delegate {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn write_cfg(&self, config: &Config) -> TokenStream {
        write_simple_cfg(self, config)
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        let name = self.write_name(config);
        let vtbl_name: TokenStream = format!("{}_Vtbl", trim_tick(self.def.name()))
            .parse()
            .unwrap();
        let boxed: TokenStream = format!("{}Box", trim_tick(self.def.name()))
            .parse()
            .unwrap();
        let generic_names = self.generics.iter().map(|ty| ty.write_name(config));
        let generic_names = quote! { #(#generic_names,)* };

        let constraints = config.write_generic_constraints(&self.generics);
        let named_phantoms = config.write_generic_named_phantoms(&self.generics);
        let method = self.method(config.reader);
        let cfg = self.write_cfg(config);

        let invoke = method.write(
            config,
            None,
            InterfaceKind::Default,
            &mut MethodNames::for_style(&config.bindgen.style),
            &mut MethodNames::for_style(&config.bindgen.style),
        );

        // In minimal mode, delegates are invoked by the API, not by user code.
        // Suppress the public Invoke() method for all minimal delegates.
        let is_event_only = config.bindgen.style.is_minimal()
            && config.event_only_delegates.contains(&self.type_name());
        let invoke_method = if config.bindgen.style.is_minimal() {
            quote! {}
        } else {
            invoke
        };

        let invoke_vtbl = method.write_abi(config, true);

        let definition = if self.generics.is_empty() {
            let guid = config.write_guid_u128(&self.def.guid_attribute().unwrap());

            quote! {
                #cfg
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                #cfg
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
            }
        } else {
            let phantoms = config.write_generic_phantoms(&self.generics);

            let guid = self.def.guid_attribute().unwrap();
            let pinterface = Literal::byte_string(format!("pinterface({{{guid}}}").as_bytes());

            let generics = self.generics.iter().map(|generic| {
                let name = generic.write_name(config);
                quote! {
                    .push_slice(b";").push_other(#name::SIGNATURE)
                }
            });

            quote! {
                #cfg
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct #name(windows_core::IUnknown, #phantoms) where #constraints;
                #cfg
                unsafe impl<#constraints> windows_core::Interface for #name {
                    type Vtable = #vtbl_name<#generic_names>;
                    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
                }
                #cfg
                impl<#constraints> windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(#pinterface)#(#generics)*.push_slice(b")");
                }
            }
        };

        // Under `--minimal`, the delegate's `new` accepts a closure that
        // returns nothing; the boxed `Invoke` calls it and returns `S_OK`
        // directly, avoiding a `Result` round trip. This also lets the
        // generated event-add wrappers reuse the same closure signature.
        let fn_constraint = {
            let signature = if config.bindgen.style.is_minimal() {
                method.write_impl_signature_no_return(config)
            } else {
                method.write_impl_signature(config, false, false)
            };

            if config.bindgen.style.is_minimal() {
                quote! { F: Fn #signature + 'static }
            } else {
                quote! { F: Fn #signature + Send + 'static }
            }
        };

        // Event-only delegates never need `new()` — the event-add wrapper
        // inlines the DelegateBox construction directly.
        let new_method = if is_event_only {
            quote! {}
        } else {
            quote! {
                pub fn new<#fn_constraint>(invoke: F) -> Self {
                    let com = windows_core::imp::DelegateBox::<#name, F>::new(&#boxed::<#generic_names F>::VTABLE, invoke);
                    unsafe {
                        core::mem::transmute(windows_core::imp::Box::new(com))
                    }
                }
            }
        };

        let invoke_upcall = if config.bindgen.style.is_minimal() {
            method.write_upcall_no_return(quote! { (this.invoke) }, false, config)
        } else {
            method.write_upcall(quote! { (this.invoke) }, false, config)
        };

        let impl_block = if is_event_only {
            quote! {}
        } else {
            quote! {
                #cfg
                impl<#constraints> #name {
                    #new_method
                    #invoke_method
                }
            }
        };

        let hide_vtbl = if config.bindgen.style.is_minimal() {
            quote! {}
        } else {
            quote! { #[doc(hidden)] }
        };

        quote! {
            #definition
            #impl_block
            #cfg
            #[repr(C)]
            #hide_vtbl
            pub struct #vtbl_name<#generic_names> where #constraints {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(#invoke_vtbl) -> windows_core::HRESULT,
                #named_phantoms
            }
            #cfg
            struct #boxed<#generic_names #fn_constraint>(core::marker::PhantomData<(#generic_names fn() -> F,)>) where #constraints;
            #cfg
            impl<#constraints #fn_constraint> #boxed<#generic_names F> {
                const VTABLE: #vtbl_name<#generic_names> = #vtbl_name::<#generic_names>{
                    base__: windows_core::IUnknown_Vtbl{
                        QueryInterface: windows_core::imp::DelegateBox::<#name, F>::QueryInterface,
                        AddRef: windows_core::imp::DelegateBox::<#name, F>::AddRef,
                        Release: windows_core::imp::DelegateBox::<#name, F>::Release,
                    },
                    Invoke: Self::Invoke,
                    #named_phantoms
                };
                unsafe extern "system" fn Invoke(#invoke_vtbl) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox::<#name, F>);
                        #invoke_upcall
                    }
                }
            }
        }
    }

    pub fn method(&self, reader: &Reader) -> Method {
        Method::new(
            self.def
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap(),
            &self.generics,
            reader,
        )
    }

    pub fn runtime_signature(&self, reader: &Reader) -> String {
        if self.generics.is_empty() {
            let guid = self.def.guid_attribute().unwrap();
            format!("delegate({{{guid}}})")
        } else {
            interface_signature(self.def, &self.generics, reader)
        }
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &self.generics)
    }
}

impl Dependencies for Delegate {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        dependencies.combine(&self.method(reader).dependencies);

        for ty in &self.generics {
            ty.combine(dependencies, reader);
        }
    }
}
