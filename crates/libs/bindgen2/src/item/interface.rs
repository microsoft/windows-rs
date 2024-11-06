use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterfaceKind {
    None,
    Default,
    Static,
    Composable,
    Base,
}

#[derive(Clone, Debug)]
pub enum MethodOrName {
    Method(Method),
    Name(&'static str),
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub def: TypeDef,
    pub generics: Vec<Type>,
    pub methods: Vec<MethodOrName>,
    pub kind: InterfaceKind,
    pub required_interfaces: Vec<Interface>,
    // TODO: store dependencies here (in expand) to avoid repeated calls to self.required_interfaces()
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.def == other.def
    }
}

impl Eq for Interface {}

impl Ord for Interface {
    fn cmp(&self, other: &Self) -> Ordering {
        self.def.cmp(&other.def)
    }
}

impl PartialOrd for Interface {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Interface {
    pub fn expand(&mut self, filter: &NameTree) {
        self.methods = self
            .def
            .methods()
            .map(|def| {
                let method = Method::new(def, &self.generics);
                if method.dependencies.included(filter) {
                    MethodOrName::Method(method)
                } else {
                    MethodOrName::Name(method.def.name())
                }
            })
            .collect();

        self.required_interfaces = self.required_interfaces();
        self.required_interfaces.sort();
        for interface in self.required_interfaces.iter_mut() {
            interface.expand(filter);
        }
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = self.write_name(writer);

        let vtbl_name = self.write_vtbl_name(writer);
        let is_exclusive = self.def.has_attribute("ExclusiveToAttribute");
        let constraints = writer.write_generic_constraints(&self.generics);
        let phantoms = writer.write_generic_phantoms(&self.generics);
        // TODO: should be able to "quote" this from the above
        let named_phantoms = writer.write_generic_named_phantoms(&self.generics);
        let interfaces = self.required_interfaces();

        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies);
        }

        let cfg = writer.write_cfg(self.def, self.def.namespace(), &dependencies, false);

        // TODO: have same vtbl/sys representation for WinRT interfaces as for Cpp interfaces

        let mut result = if self.generics.is_empty() {
            let guid = writer.write_guid_u128(&self.def.guid_attribute().unwrap());

            quote! {
                #cfg
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                #cfg
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
            }
        } else {
            let guid = self.def.guid_attribute().unwrap();
            let pinterface = Literal::byte_string(&format!("pinterface({{{guid}}}"));

            let generics = self.generics.iter().map(|generic| {
                let name = generic.write(writer);

                quote! {
                    .push_slice(b";").push_other(#name::SIGNATURE)
                }
            });

            quote! {
                #[repr(transparent)]
                #[derive(PartialEq, Eq, Debug, Clone)]
                pub struct #name(windows_core::IUnknown, #phantoms) where #constraints;
                impl<#constraints> windows_core::imp::CanInto<windows_core::IUnknown> for #name {}
                impl<#constraints> windows_core::imp::CanInto<windows_core::IInspectable> for #name {}
                unsafe impl<#constraints> windows_core::Interface for #name {
                    type Vtable = #vtbl_name;
                    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
                }
                impl<#constraints> windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(#pinterface)#(#generics)*.push_slice(b")");
                }
            }
        };

        if !is_exclusive && self.generics.is_empty() {
            result.combine(quote! {
                #cfg
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
            });
        }

        if !is_exclusive && !interfaces.is_empty() {
            if self.generics.is_empty() {
                let interfaces = interfaces.iter().map(|ty| ty.write_name(writer));

                result.combine(quote! {
                    #cfg
                    windows_core::imp::required_hierarchy!(#name, #(#interfaces),*);
                });
            } else {
                let interfaces = interfaces.iter().map(|ty| {
                    let ty = ty.write_name(writer);
                    quote!{
                        impl<#constraints> windows_core::imp::CanInto<#ty> for #name { const QUERY: bool = true; }
                    }
                });

                result.combine(quote! {
                    #(#interfaces)*
                });
            }
        }

        if !is_exclusive {
            let method_names = &mut MethodNames::new();
            let virtual_names = &mut MethodNames::new();
            let mut methods = TokenStream::new();

            for method in self.methods.iter().filter_map(|method| match &method {
                MethodOrName::Method(method) => Some(method),
                _ => None,
            }) {
                let mut difference = Dependencies::new();

                if writer.config.package {
                    difference = method.dependencies.difference(&dependencies);
                }

                let cfg = writer.write_cfg(self.def, self.def.namespace(), &difference, false);

                let method = method.write(
                    writer,
                    self.write_name(writer),
                    InterfaceKind::Default,
                    method_names,
                    virtual_names,
                );

                methods.combine(quote! {
                    #cfg
                    #method
                });
            }

            for interface in &self.required_interfaces {
                let virtual_names = &mut MethodNames::new();

                for method in interface.methods.iter().filter_map(|method| match &method {
                    MethodOrName::Method(method) => Some(method),
                    _ => None,
                }) {
                    let mut difference = Dependencies::new();

                    if writer.config.package {
                        difference = method.dependencies.difference(&dependencies);
                    }

                    let cfg = writer.write_cfg(self.def, self.def.namespace(), &difference, false);

                    let method = method.write(
                        writer,
                        interface.write_name(writer),
                        interface.kind,
                        method_names,
                        virtual_names,
                    );

                    methods.combine(quote! {
                        #cfg
                        #method
                    });
                }
            }

            if !methods.is_empty() {
                result.combine(quote! {
                    impl<#constraints> #name {
                        #methods
                    }
                });
            }
        }

        {
            let virtual_names = &mut MethodNames::new();

            let vtbl_methods = self.methods.iter().map(|method| match method {
                MethodOrName::Method(method) => {
                    let mut difference = Dependencies::new();

                    if writer.config.package {
                        difference = method.dependencies.difference(&dependencies);
                    }

                    let name = virtual_names.add(method.def);
                    let vtbl = method.write_abi(writer, false);
                    let cfg = writer.write_cfg(self.def, self.def.namespace(), &difference, false);

                    if cfg.is_empty() {
                        quote! {
                            pub #name: unsafe extern "system" fn(#vtbl) -> windows_core::HRESULT,
                        }
                    } else {
                        let cfg_not =
                            writer.write_cfg(self.def, self.def.namespace(), &difference, true);

                        quote! {
                            #cfg
                            pub #name: unsafe extern "system" fn(#vtbl) -> windows_core::HRESULT,
                            #cfg_not
                            #name: usize,
                        }
                    }
                }
                MethodOrName::Name(name) => {
                    let name = to_ident(name);
                    quote! { #name: usize, }
                }
            });

            result.combine(quote! {
                #cfg
                #[repr(C)]
                pub struct #vtbl_name where #constraints {
                    pub base__: windows_core::IInspectable_Vtbl,
                    #(#vtbl_methods)*
                    #named_phantoms
                }
            });
        }

        if writer.config.implement || !is_exclusive {
            let impl_name: TokenStream = format!("{}_Impl", self.def.name()).into();
            let generics: Vec<_> = self.generics.iter().map(|ty| ty.write(writer)).collect();
            let runtime_name = format!("{}.{}", self.def.namespace(), self.def.name(),);

            if writer.config.package {
                fn collect(interface: &Interface, dependencies:&mut Dependencies) {
                    for method in interface.methods.iter() {
                        if let MethodOrName::Method(method) = method {
                            dependencies.combine(&method.dependencies);
                        }
                    }
                }

                collect(self, &mut dependencies);
                self.required_interfaces.iter().for_each(|interface| collect(interface, &mut dependencies));
            }

            let cfg = writer.write_cfg(self.def, self.def.namespace(), &dependencies, false);

            result.combine(quote! {
                #cfg
                impl<#constraints> windows_core::RuntimeName for #name {
                    const NAME: &'static str = #runtime_name;
                }
            });

            let mut names = MethodNames::new();

            let field_methods: Vec<_> = self
                .methods
                .iter()
                .map(|method| match method {
                    MethodOrName::Method(method) => {
                        let name = names.add(method.def);
                        quote! { #name: #name::<#(#generics,)* Identity, OFFSET>, }
                    }
                    MethodOrName::Name(name) => {
                        // TODO: test this condition - should cause an AV when method is called
                        // TODO: does this need to use `MethodNames` for overloading?
                        let name = to_ident(name);
                        quote! { #name: 0, }
                    }
                })
                .collect();

            let mut names = MethodNames::new();

            let impl_methods: Vec<_> = self.methods.iter().map(|method| match method {
                MethodOrName::Method(method) => {
                    let name = names.add(method.def);
                    let signature = method.write_abi(writer, true);
                    let call = quote! { #impl_name::#name };
                    let upcall = method.write_upcall(call, true);

                    quote! {
                        unsafe extern "system" fn #name<#constraints Identity: #impl_name <#(#generics,)*>, const OFFSET: isize> (#signature) -> windows_core::HRESULT {
                            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            #upcall
                        }
                    }
                }
                _ => quote! {},
            }).collect();

            let mut names = MethodNames::new();

            let trait_methods: Vec<_> = self
                .methods
                .iter()
                .map(|method| match method {
                    MethodOrName::Method(method) => {
                        let name = names.add(method.def);
                        let signature = method.write_impl_signature(writer, true, true);
                        quote! { fn #name #signature; }
                    }
                    _ => quote! {},
                })
                .collect();

            let requires = if interfaces.is_empty() {
                // TODO: doesn't IUnknownImpl require Sized?
                quote! { Sized + windows_core::IUnknownImpl }
            } else {
                let interfaces = interfaces.iter().map(|ty| ty.write_impl_name(writer));

                quote! {  #(#interfaces)+* }
            };

            result.combine(quote! {
                #cfg
                pub trait #impl_name <#(#generics),*> : #requires where #constraints {
                    #(#trait_methods)*
                }
                #cfg
                impl<#constraints> #vtbl_name {
                    pub const fn new<Identity: #impl_name <#(#generics,)*>, const OFFSET: isize>() -> Self {
                        #(#impl_methods)*
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<Identity, #name, OFFSET>(),
                            #(#field_methods)*
                            #named_phantoms
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<#name as windows_core::Interface>::IID
                    }
                }
            });
        }

        result
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());
        let namespace = writer.write_namespace(self.def.namespace());

        if self.generics.is_empty() {
            quote! { #namespace #name }
        } else {
            let generics = self.generics.iter().map(|ty| ty.write(writer));
            quote! { #namespace #name < #(#generics),* > }
        }
    }

    fn write_vtbl_name(&self, writer: &Writer) -> TokenStream {
        let name: TokenStream = format!("{}_Vtbl", self.def.name()).into();

        if self.generics.is_empty() {
            name
        } else {
            let generics = self.generics.iter().map(|ty| ty.write(writer));
            quote! { #name < #(#generics,)* > }
        }
    }

    pub fn write_impl_name(&self, writer: &Writer) -> TokenStream {
        let name: TokenStream = format!("{}_Impl", self.def.name()).into();
        let namespace = writer.write_namespace(self.def.namespace());

        if self.generics.is_empty() {
            quote! { #namespace #name }
        } else {
            let generics = self.generics.iter().map(|ty| ty.write(writer));
            quote! { #namespace #name < #(#generics),* > }
        }
    }

    pub fn runtime_signature(&self) -> String {
        interface_signature(self.def, &self.generics)
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for ty in &self.generics {
                ty.dependencies(dependencies);
            }
            for interface in self.required_interfaces() {
                interface.dependencies(dependencies);
            }
        }
    }

    // TODO: this is where we can use config.minimal to elide required interfaces that aren't included?
    pub fn required_interfaces(&self) -> Vec<Self> {
        fn walk(interface: &Interface, set: &mut Vec<Interface>) {
            for ty in interface
                .def
                .interface_impls()
                .map(|imp| imp.ty(&interface.generics))
            {
                let Type::Item(Item::Interface(interface)) = ty else {
                    panic!();
                };

                if !set.iter().any(|existing| existing.def == interface.def) {
                    walk(&interface, set);
                    set.push(interface);
                }
            }
        }
        let mut set = vec![];
        walk(self, &mut set);
        set
    }
}
