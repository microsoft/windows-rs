use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Class(pub GenericType);

impl Class {
    pub fn type_signature(&self) -> String {
        let default = self
            .0
            .interfaces()
            .find(|(_, kind)| *kind == parser::InterfaceKind::Default)
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    format!(
                        "Class {}.{} does not have a default interface.",
                        self.0.def.namespace(),
                        self.0.def.name()
                    )
                )
            });

        format!(
            "rc({}.{};{})",
            self.0.def.namespace(),
            self.0.def.name(),
            default.0 .0.interface_signature()
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

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        let generics = self.0.generics.iter().filter_map(|g| g.definition());
        let interfaces = self.0.interfaces().filter_map(|i| i.0.definition());
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
        let mut i: Vec<(Interface, parser::InterfaceKind)> = c.0.interfaces().collect();
        assert_eq!(i.len(), 3);

        i.sort_by(|a, b| {
            a.0 .0
                .gen_name(&Gen::Absolute)
                .as_str()
                .cmp(b.0 .0.gen_name(&Gen::Absolute).as_str())
        });

        assert_eq!(
            i[0].0.0.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IIterable :: < windows :: foundation :: collections :: IKeyValuePair :: < windows :: HString , windows :: HString > >"
        );

        assert_eq!(i[0].1, InterfaceKind::NonDefault);

        assert_eq!(
            i[1].0.0.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IMap :: < windows :: HString , windows :: HString >"
        );

        assert_eq!(i[1].1, InterfaceKind::Default);

        assert_eq!(
            i[2].0.0.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IObservableMap :: < windows :: HString , windows :: HString >"
        );

        assert_eq!(i[2].1, InterfaceKind::NonDefault);
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
