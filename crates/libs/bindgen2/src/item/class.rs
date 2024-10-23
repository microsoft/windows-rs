use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Class {
    pub def: TypeDef,
    pub generics: Vec<Type>,
    pub required_interfaces: Vec<Interface>, // This is required interfaces that have been "expanded" to include methods
}

impl Class {
     pub fn expand(&mut self, filter:&NameTree)  {
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

        let runtime_name = format!("{}.{}", self.def.namespace(), self.def.name(),);

        let runtime_name = quote! {
            impl windows_core::RuntimeName for #name {
                const NAME: &'static str = #runtime_name;
            }
        };

        let mut methods = quote! {};
        let mut method_names = MethodNames::new();

        for interface in &self.required_interfaces {
            let mut virtual_names = MethodNames::new();

            for method in interface.methods() {
                if !method.included(&writer.config.tree) {
                    continue;
                }

                methods.combine(method.write(
                    writer,
                    interface.write_name(writer),
                    interface.kind,
                    &mut method_names,
                    &mut virtual_names,
                ));
            }
        }

        if let Some(default_interface) = self.default_interface() {
            let default_interface = default_interface.write(writer);

            quote! {
                #[repr(transparent)]
                #[derive(PartialEq, Eq, Debug, Clone)]
                pub struct #name(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
                //windows_core::imp::required_hierarchy!(#name, IClosable);
                impl #name {
                    #methods
                }
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, #default_interface>();
                }
                unsafe impl windows_core::Interface for #name {
                    type Vtable = <#default_interface as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <#default_interface as windows_core::Interface>::IID;
                }
                #runtime_name
            }
        } else {
            quote! {
                pub struct #name;
                impl #name {
                    //#(#methods)*
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
        fn walk(
            def: TypeDef,
            generics: &[Type],
            is_base: bool,
            set: &mut Vec<Interface>,
        ) {
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

                // TODO: replace for loop with iterator -> is_some()

                let mut found = false;
                for existing in set.iter_mut() {
                    if existing.def == interface.def {
                        found = true;
                        existing.kind = interface.kind;
                    }
                }
                    if found {
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
}
