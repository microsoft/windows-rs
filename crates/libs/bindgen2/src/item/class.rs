use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Class {
    pub def: TypeDef,
    pub generics: Vec<Type>,
    pub required_interfaces: Vec<Interface>, // This is required interfaces that have been "expanded" to include methods
}

impl Class {
    pub fn expand(&mut self, filter: &NameTree) {
        // TODO: load interfaces, methods, bases?
        //  for interface in self.required_interfaces() {
        //     set.interfaces.insert(interface.expand(filter));
        //  }

        debug_assert!(self.required_interfaces.is_empty());

        self.required_interfaces = self.required_interfaces();
        self.required_interfaces.sort();
        for interface in self.required_interfaces.iter_mut() {
            interface.expand(filter);
        }
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());

        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies);
        }

        let cfg = writer.write_cfg(self.def, self.def.namespace(), &dependencies, false);

        let runtime_name = format!("{}.{}", self.def.namespace(), self.def.name(),);

        let runtime_name = quote! {
            #cfg
            impl windows_core::RuntimeName for #name {
                const NAME: &'static str = #runtime_name;
            }
        };

        let mut methods = quote! {};
        let mut method_names = MethodNames::new();

        for interface in &self.required_interfaces {
            let mut virtual_names = MethodNames::new();

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
                    &mut method_names,
                    &mut virtual_names,
                );

                methods.combine(quote! {
                    #cfg
                    #method
                });
            }
        }

        let new = self.has_default_constructor().then(||
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
        );

        let factories = self.required_interfaces.iter().filter_map(|interface| match interface.kind {
            InterfaceKind::Static | InterfaceKind::Composable => {
                if interface.methods.is_empty() {
                    None
                } else {
                        let interface_type = interface.write_name(writer);
                        let cfg = quote! {};

                        Some(quote! {
                            #cfg
                            fn #interface_type<R, F: FnOnce(&#interface_type) -> windows_core::Result<R>>(
                                callback: F,
                            ) -> windows_core::Result<R> {
                                static SHARED: windows_core::imp::FactoryCache<#name, #interface_type> =
                                    windows_core::imp::FactoryCache::new();
                                SHARED.call(callback)
                            }
                        })
                    }
                }
                _ => None,
            });

        if let Some(default_interface) = self.default_interface() {
            let default_interface = default_interface.write(writer);
            let interfaces = self
                .required_interfaces
                .iter()
                .filter(|ty|!ty.def.has_attribute("ExclusiveToAttribute"))
                .map(|ty| ty.write_name(writer));

            quote! {
                #cfg
                #[repr(transparent)]
                #[derive(PartialEq, Eq, Debug, Clone)]
                pub struct #name(windows_core::IUnknown);
                #cfg
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
                #cfg
                windows_core::imp::required_hierarchy!(#name, #(#interfaces),*);
                #cfg
                impl #name {
                    #new
                    #methods
                    #(#factories)*
                }
                #cfg
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, #default_interface>();
                }
                #cfg
                unsafe impl windows_core::Interface for #name {
                    type Vtable = <#default_interface as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <#default_interface as windows_core::Interface>::IID;
                }
                #runtime_name
            }
        } else {
            quote! {
                #cfg
                pub struct #name;
                #cfg
                impl #name {
                    #methods
                    #(#factories)*
                }
                #runtime_name
            }
        }
    }

    pub fn default_interface(&self) -> Option<Type> {
        self.def
            .interface_impls()
            .find(|imp| imp.has_attribute("DefaultAttribute"))
            .map(|imp| imp.ty(&self.generics))
    }

    pub fn runtime_signature(&self) -> String {
        format!(
            "rc({}.{};{})",
            self.def.namespace(),
            self.def.name(),
            self.default_interface().unwrap().runtime_signature()
        )
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        // TODO: this should succeed only if config is not excluding this item
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for interface in self.required_interfaces() {
                interface.dependencies(dependencies);
            }
        }
    }

    fn bases(&self) -> Vec<Self> {
        let mut bases = Vec::new();
        let mut def = self.def;
        let reader = def.reader();

        loop {
            let extends = def.extends().unwrap();
            let extends = TypeName(extends.namespace(), extends.name());

            if extends == TypeName::Object {
                break;
            }

            let Some(Item::Class(base)) = reader
                .with_full_name(extends.namespace(), extends.name())
                .next()
            else {
                panic!("Type not found");
            };

            def = base.def;
            bases.push(base);
        }

        bases
    }

    // TODO: this is where we can use config.minimal to elide required interfaces that aren't included?
    pub fn required_interfaces(&self) -> Vec<Interface> {
        fn walk(def: TypeDef, generics: &[Type], is_base: bool, set: &mut Vec<Interface>) {
            for imp in def.interface_impls() {
                let Type::Item(Item::Interface(mut interface)) = imp.ty(generics) else {
                    panic!();
                };

                interface.kind = if !is_base && imp.has_attribute("DefaultAttribute") {
                    InterfaceKind::Default
                } else if is_base {
                    InterfaceKind::Base
                } else {
                    InterfaceKind::None
                };

                if let Some(pos) = set
                    .iter()
                    .position(|existing| existing.def == interface.def)
                {
                    set[pos].kind = interface.kind;
                } else {
                    walk(interface.def, &interface.generics, is_base, set);
                    set.push(interface);
                }
            }
        }
        let mut set = vec![];
        walk(self.def, &self.generics, false, &mut set);

        for base in self.bases() {
            walk(base.def, &[], true, &mut set);
        }

        for attribute in self.def.attributes() {
            let kind = match attribute.name() {
                "StaticAttribute" | "ActivatableAttribute" => InterfaceKind::Static,
                "ComposableAttribute" => InterfaceKind::Composable,
                _ => continue,
            };

            for (_, arg) in attribute.args() {
                if let Value::TypeName(type_name) = arg {
                    let Some(Item::Interface(mut interface)) = self
                        .def
                        .reader()
                        .with_full_name(type_name.namespace(), type_name.name())
                        .next()
                    else {
                        panic!("Type not found");
                    };

                    interface.kind = kind;

                    set.push(interface);
                    break;
                }
            }
        }

        set
    }

    fn has_default_constructor(&self) -> bool {
        self.def
            .attributes()
            .filter(|attribute| attribute.name() == "ActivatableAttribute")
            .any(|attribute| {
                !attribute
                    .args()
                    .iter()
                    .any(|arg| matches!(arg.1, Value::TypeName(_)))
            })
    }
}
