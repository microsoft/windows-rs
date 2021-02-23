use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Class(pub GenericType);

impl Class {
    pub fn type_signature(&self) -> String {
        let default = self.0.default_interface();

        format!(
            "rc({}.{};{})",
            self.0.def.namespace(),
            self.0.def.name(),
            default.interface_signature()
        )
    }

    // TODO: fold into GenericType::interfaces
    pub fn factories(&self) -> impl Iterator<Item = (Interface, parser::InterfaceKind)> + '_ {
        self.0.def.attributes().filter_map(|attribute| {
            match attribute.full_name() {
                ("Windows.Foundation.Metadata", "StaticAttribute")
                | ("Windows.Foundation.Metadata", "ActivatableAttribute") => {
                    for (_, arg) in attribute.args() {
                        if let parser::ConstantValue::TypeDef(def) = arg {
                            return Some((
                                Interface(GenericType::from_type_def(def, Vec::new())),
                                parser::InterfaceKind::Static,
                            ));
                        }
                    }

                    None
                }
                ("Windows.Foundation.Metadata", "ComposableAttribute") => {
                    // One of the arguments is a CompositionType enum and the Public variant
                    // has a value of 2 as a signed 32-bit integer.

                    let mut public = false;
                    let mut interface = None;

                    for (_, arg) in attribute.args() {
                        match arg {
                            parser::ConstantValue::I32(2) => {
                                public = true;
                            }
                            parser::ConstantValue::TypeDef(def) => {
                                interface = Some((
                                    Interface(GenericType::from_type_def(def, Vec::new())),
                                    parser::InterfaceKind::Composable,
                                ));
                            }
                            _ => {}
                        }
                    }

                    if public {
                        interface
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
    }

    pub fn interfaces(&self) -> Vec<InterfaceInfo> {
        fn add_interfaces(result: &mut Vec<InterfaceInfo>, parent: &GenericType, is_base: bool) {
            for child in parent.def.interface_impls() {
                if let Some(def) = child.generic_interface(&parent.generics) {
                    if !result.iter().any(|info| info.def == def) {
                        add_interfaces(result, &def, is_base);

                        let kind = if child.is_default() {
                            InterfaceKind::Default
                        } else {
                            InterfaceKind::NonDefault
                        };

                        let version = def.def.version();

                        result.push(InterfaceInfo {
                            def,
                            kind,
                            is_base,
                            version,
                        });
                    }
                }
            }
        }

        let mut result = Vec::new();
        add_interfaces(&mut result, &self.0, false);

        for base in self.0.bases() {
            add_interfaces(&mut result, &base, true);
        }

        InterfaceInfo::sort(&mut result);
        result
    }

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        let generics = self.0.generics.iter().filter_map(|g| g.definition());
        let interfaces = self.0.interfaces().map(|i| i.def);
        let bases = self.0.bases().map(|b| b.def);
        let factories = self.factories().filter_map(|(i, _)| i.definition());

        generics
            .chain(interfaces)
            .chain(bases)
            .chain(factories)
            .collect()
    }

    pub fn definition(&self) -> Option<tables::TypeDef> {
        Some(self.0.def)
    }

    pub fn gen(&self, _: &Gen) -> TokenStream {
        quote! {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let c = TypeReader::get_class("Windows.Foundation", "Uri");
        assert_eq!(
            c.type_signature(),
            "rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})"
        )
    }

    #[test]
    fn test_class() {
        let c = TypeReader::get_class("Windows.Foundation.Collections", "StringMap");
        let mut i = c.interfaces();
        assert_eq!(i.len(), 3);

        assert_eq!(
            i[0].def.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IMap :: < windows :: HString , windows :: HString >"
        );
        assert_eq!(i[0].kind, InterfaceKind::Default);

        assert_eq!(
            i[1].def.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IIterable :: < windows :: foundation :: collections :: IKeyValuePair :: < windows :: HString , windows :: HString > >"
        );
        assert_eq!(i[1].kind, InterfaceKind::NonDefault);

        assert_eq!(
            i[2].def.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IObservableMap :: < windows :: HString , windows :: HString >"
        );
        assert_eq!(i[2].kind, InterfaceKind::NonDefault);

    }

    #[test]
    fn test_bases() {
        let c = TypeReader::get_class("Windows.Foundation", "Uri");
        assert_eq!(c.0.bases().count(), 0);

        let c = TypeReader::get_class("Windows.UI.Composition", "CompositionObject");
        assert_eq!(c.0.bases().count(), 0);

        let c = TypeReader::get_class("Windows.UI.Composition", "Visual");
        let bases: Vec<GenericType> = c.0.bases().collect();
        assert_eq!(bases.len(), 1);
        assert_eq!(bases[0].def.name(), "CompositionObject");

        let c = TypeReader::get_class("Windows.UI.Composition", "SpriteVisual");
        let bases: Vec<GenericType> = c.0.bases().collect();
        assert_eq!(bases.len(), 3);
        assert_eq!(bases[0].def.name(), "ContainerVisual");
        assert_eq!(bases[1].def.name(), "Visual");
        assert_eq!(bases[2].def.name(), "CompositionObject");
    }
}
