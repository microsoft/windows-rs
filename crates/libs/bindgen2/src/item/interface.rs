use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Interface {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}

impl Interface {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = self.write_name(writer);
        let vtbl_name = self.write_vtbl_name();
        let guid = writer.write_guid_u128(&self.def.guid_attribute().unwrap());
        let non_exclusive = !self.def.has_attribute("ExclusiveToAttribute");
        let constraints = writer.write_generic_constraints(&self.generics);
        let required_hierarchy = self.required_interfaces();

        let methods = non_exclusive.then(|| {
            let method_names = &mut MethodNames::new();
            let virtual_names = &mut MethodNames::new();

            let methods = self.def.methods().map(|method| {
                writer.write_method(method, &self.generics, InterfaceKind::Default, method_names, virtual_names)
            });

            quote! {
                impl<#constraints> #name {
                    #(#methods)*
                }
            }
        });

        let required_hierarchy =non_exclusive.then(|| {
            if self.generics.is_empty() {
                    if required_hierarchy.is_empty() {
                        quote! {}
                    } else {
                        let required_hierarchy = required_hierarchy.iter().map(|ty| ty.write_name(writer));
                        quote! {
                            windows_core::imp::required_hierarchy!(#name, #(#required_hierarchy),*);
                        }
                    }
            } else {
                let required_hierarchy = required_hierarchy.iter().map(|ty| {
                    let ty = ty.write_name(writer);
                    quote!{
                        impl<#constraints> windows_core::imp::CanInto<#ty> for #name { const QUERY: bool = true; }
                    }
                });

                quote! {
                    #(#required_hierarchy)*
                }
            }
        });

        // TODO: this disparity is a real pain to code gen
        let definition = if self.generics.is_empty() {
            quote! {
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
                #required_hierarchy
            }
        } else {
            let phantoms = writer.write_generic_phantoms(&self.generics);

            quote! {
                #[repr(transparent)]
                #[derive(PartialEq, Eq, Debug, Clone)]
                pub struct #name(windows_core::IUnknown, #phantoms) where #constraints;
                impl<#constraints> windows_core::imp::CanInto<windows_core::IUnknown> for #name {}
                impl<#constraints> windows_core::imp::CanInto<windows_core::IInspectable> for #name {}
                #required_hierarchy
                unsafe impl<#constraints> windows_core::Interface for #name {
                    type Vtable = #vtbl_name;
                    const IID: windows_core::GUID = windows_core::GUID::from_u128(#guid);
                }
            }
        };

        quote! {
            #definition
            impl<#constraints> core::ops::Deref for #name {
                type Target = windows_core::IInspectable;
                fn deref(&self) -> &Self::Target {
                    unsafe { core::mem::transmute(self) }
                }
            }
            #methods
            impl<#constraints> windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            pub struct #vtbl_name {
                pub base__: windows_core::IInspectable_Vtbl,
            }
        }
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());
        let namespace = writer.write_namespace(self.def.namespace());
        let generics = writer.write_generic_names(&self.generics);
        quote! { #namespace #name #generics }
    }

    pub fn write_vtbl_name(&self) -> TokenStream {
        format!("{}_Vtbl", self.def.name()).into()
    }

    pub fn runtime_signature(&self) -> String {
        interface_signature(self.def, &self.generics)
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for interface in self.required_interfaces() {
                interface.dependencies(dependencies, config);
            }
        }
    }

    // TODO: this is where we can use config.minimal to elide required interfaces that aren't included?
    pub fn required_interfaces(&self) -> BTreeSet<Interface> {
        fn walk(interface: &Interface, set: &mut BTreeSet<Interface>) {
            for ty in interface
                .def
                .interface_impls()
                .map(|imp| imp.ty(&interface.generics))
            {
                let Type::Item(Item::Interface(interface)) = ty else {
                    panic!();
                };

                if !set.contains(&interface) {
                    walk(&interface, set);
                    set.insert(interface);
                }
            }
        }
        let mut set = BTreeSet::new();
        walk(self, &mut set);
        set
    }
}
