use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Class {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}

impl Class {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());

        let runtime_name = format!("{}.{}", self.def.namespace(), self.def.name(),);

        let runtime_name = quote! {
            impl windows_core::RuntimeName for #name {
                const NAME: &'static str = #runtime_name;
            }
        };

        if let Some(default_interface) = self.default_interface(&self.generics) {
            let default_interface = default_interface.write(writer);

            quote! {
                #[repr(transparent)]
                #[derive(PartialEq, Eq, Debug, Clone)]
                pub struct #name(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
                //windows_core::imp::required_hierarchy!(#name, IClosable);
                impl #name {
                    //#(#methods)*
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

    pub fn default_interface(&self, generics: &[Type]) -> Option<Type> {
        self.def
            .interface_impls()
            .find(|imp| imp.has_attribute("DefaultAttribute"))
            .map(|imp| imp.ty(generics))
    }

    pub fn runtime_signature(&self) -> String {
        format!(
            "rc({}.{};{})",
            self.def.namespace(),
            self.def.name(),
            self.default_interface(&self.generics)
                .unwrap()
                .runtime_signature()
        )
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for (interface, _) in self.required_interfaces() {
                interface.dependencies(dependencies, config);
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
    pub fn required_interfaces(&self) -> BTreeMap<Interface, InterfaceKind> {
        fn walk(
            def: TypeDef,
            generics: &[Type],
            is_base: bool,
            set: &mut BTreeMap<Interface, InterfaceKind>,
        ) {
            for imp in def.interface_impls() {
                let Type::Item(Item::Interface(interface)) = imp.ty(generics) else {
                    panic!();
                };

                let kind = if !is_base && imp.has_attribute("DefaultAttribute") {
                    InterfaceKind::Default
                } else if is_base {
                    InterfaceKind::Base
                } else {
                    InterfaceKind::None
                };

                if let Some(existing) = set.get_mut(&interface) {
                    if kind == InterfaceKind::Default {
                        *existing = kind;
                    }
                } else {
                    walk(interface.def, &interface.generics, is_base, set);
                    set.insert(interface, kind);
                }
            }
        }
        let mut set = BTreeMap::new();
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
                    let Some(Item::Interface(interface)) = self
                        .def
                        .reader()
                        .with_full_name(type_name.namespace(), type_name.name())
                        .next()
                    else {
                        panic!("Type not found");
                    };

                    set.insert(interface, kind);
                    break;
                }
            }
        }

        set
    }
}
