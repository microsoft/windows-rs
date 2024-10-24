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
        let guid = writer.write_guid_u128(&self.def.guid_attribute().unwrap());
        let non_exclusive = !self.def.has_attribute("ExclusiveToAttribute");
        let constraints = writer.write_generic_constraints(&self.generics);
        let phantoms = writer.write_generic_phantoms(&self.generics);
        // TODO: should be able to "quote" this from the above
        let named_phantoms = writer.write_generic_named_phantoms(&self.generics);
        let interfaces = self.required_interfaces();

        let methods = non_exclusive.then(|| {
            let method_names = &mut MethodNames::new();
            let virtual_names = &mut MethodNames::new();

            let methods = self
                .methods
                .iter()
                .filter_map(|method| match &method {
                    MethodOrName::Method(method) => Some(method),
                    _ => None,
                })
                .map(|method| {
                    method.write(
                        writer,
                        self.write_name(writer),
                        InterfaceKind::Default,
                        method_names,
                        virtual_names,
                    )
                });

            quote! {
                impl<#constraints> #name {
                    #(#methods)*
                }
            }
        });

        let virtual_names = &mut MethodNames::new();
        
        let vtbl_methods = self.methods.iter().map(|method| {
            match method {
            MethodOrName::Method(method) => 
                method.write_vtbl(writer, false, virtual_names),
            MethodOrName::Name(name) => {
                let name = to_ident(name);
                quote! { #name: usize, }
            }
            }
        });

        // TODO: rather than all this nesting/chaining - jsut have a result token stream
        // that we combine into in successive flat conditions

        let interfaces =non_exclusive.then(|| {
            if self.generics.is_empty() {
                    let hierarchy = quote! {
                        windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
                    };

                    if interfaces.is_empty() {
                        hierarchy
                    } else {
                        let interfaces = interfaces.iter().map(|ty| ty.write_name(writer));
                        quote! {
                            #hierarchy
                            windows_core::imp::required_hierarchy!(#name, #(#interfaces),*);
                        }
                    }
            } else {
                let interfaces = interfaces.iter().map(|ty| {
                    let ty = ty.write_name(writer);
                    quote!{
                        impl<#constraints> windows_core::imp::CanInto<#ty> for #name { const QUERY: bool = true; }
                    }
                });

                quote! {
                    #(#interfaces)*
                }
            }
        });

        // TODO: this disparity is a real pain to code gen
        let definition = if self.generics.is_empty() {
            quote! {
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                #interfaces
            }
        } else {
            quote! {
                #[repr(transparent)]
                #[derive(PartialEq, Eq, Debug, Clone)]
                pub struct #name(windows_core::IUnknown, #phantoms) where #constraints;
                impl<#constraints> windows_core::imp::CanInto<windows_core::IUnknown> for #name {}
                impl<#constraints> windows_core::imp::CanInto<windows_core::IInspectable> for #name {}
                #interfaces
                unsafe impl<#constraints> windows_core::Interface for #name {
                    type Vtable = #vtbl_name;
                    const IID: windows_core::GUID = windows_core::GUID::from_u128(#guid);
                }
            }
        };

        quote! {
            #definition
            #methods
            impl<#constraints> windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            pub struct #vtbl_name where #constraints {
                pub base__: windows_core::IInspectable_Vtbl,
                #(#vtbl_methods)*
                #named_phantoms
            }
        }
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());
        let namespace = writer.write_namespace(self.def.namespace());
        let generics = writer.write_generic_names(&self.generics);
        quote! { #namespace #name #generics }
    }

    pub fn write_vtbl_name(&self, writer: &Writer) -> TokenStream {
        let name: TokenStream = format!("{}_Vtbl", self.def.name()).into();
        let generics = writer.write_generic_names(&self.generics);
        quote! { #name #generics }

    }

    pub fn runtime_signature(&self) -> String {
        interface_signature(self.def, &self.generics)
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for interface in self.required_interfaces() {
                interface.dependencies(dependencies);
            }
        }
    }

    // TODO: this is where we can use config.minimal to elide required interfaces that aren't included?
    pub fn required_interfaces(&self) -> Vec<Interface> {
        fn walk(interface: &Interface, set: &mut Vec<Interface>) {
            for ty in interface
                .def
                .interface_impls()
                .map(|imp| imp.ty(&interface.generics))
            {
                let Type::Item(Item::Interface(interface)) = ty else {
                    panic!();
                };

                if set
                    .iter()
                    .position(|existing| existing.def == interface.def)
                    .is_none()
                {
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
