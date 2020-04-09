use crate::tables::*;
use crate::types::*;
use crate::TypeReader;

use proc_macro2::TokenStream;
use quote::quote;

/// A WinRT Class
#[derive(Debug)]
pub struct Class {
    pub name: TypeName,
    pub interfaces: Vec<Interface>,
    pub bases: Vec<TypeName>,
    pub default: bool,
}

impl Class {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let mut interfaces = name.interfaces(reader);
        let mut bases = Vec::new();
        let mut base = def;

        loop {
            let (namespace, name) = base.extends(reader).name(reader);

            if (namespace, name) == ("System", "Object") {
                break;
            }

            base = reader.resolve((namespace, name));

            interfaces.extend(Interface::interfaces(reader, base, &Vec::new()));

            let namespace = namespace.to_string();
            let name = name.to_string();
            let generics = Vec::new();

            bases.push(TypeName {
                namespace,
                name,
                generics,
                def: base,
            });
        }

        let mut default = false;

        for attribute in def.attributes(reader) {
            match attribute.name(reader) {
                ("Windows.Foundation.Metadata", "StaticAttribute") => {
                    let mut interface = Interface::from_type_def(
                        reader,
                        attribute_factory(reader, attribute).unwrap(),
                        &Vec::new(),
                    );
                    interface.kind = InterfaceKind::Statics;
                    interfaces.push(interface);
                }
                ("Windows.Foundation.Metadata", "ActivatableAttribute") => {
                    match attribute_factory(reader, attribute) {
                        Some(def) => {
                            let mut interface = Interface::from_type_def(reader, def, &Vec::new());
                            interface.kind = InterfaceKind::Constructors;
                            interfaces.push(interface);
                        }
                        None => default = true,
                    }
                }
                _ => {}
            }
        }

        Self {
            name,
            interfaces,
            bases,
            default,
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.interfaces
            .iter()
            .flat_map(|i| i.name.dependencies())
            .chain(self.bases.iter().map(|i| i.def))
            .collect()
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = self.name.ident();

        quote! {
            #[repr(transparent)]
            #[derive(Default, Clone)]
            pub struct #name { ptr: winrt::IUnknown }
        }
    }
}

fn attribute_factory(reader: &TypeReader, attribute: Attribute) -> Option<TypeDef> {
    for (_, arg) in attribute.args(reader) {
        if let AttributeArg::TypeDef(def) = arg {
            return Some(def);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class((namespace, type_name): (&str, &str)) -> Class {
        let reader = &TypeReader::from_os();
        let def = reader.resolve((namespace, type_name));

        match def.into_type(reader) {
            Type::Class(t) => t,
            _ => panic!("Type not an interface"),
        }
    }

    #[test]
    fn test_url_decoder() {
        let t = class(("Windows.Foundation", "WwwFormUrlDecoder"));
        assert!(t.default == false);

        assert!(t.name.namespace == "Windows.Foundation");
        assert!(t.name.name == "WwwFormUrlDecoder");
        assert!(t.name.generics.is_empty());

        assert!(t.interfaces.len() == 4);

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IWwwFormUrlDecoderRuntimeClassFactory")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Constructors);
        assert!(interface.name.namespace == "Windows.Foundation");
        assert!(interface.name.name == "IWwwFormUrlDecoderRuntimeClassFactory");
        assert!(interface.name.generics.is_empty());

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IWwwFormUrlDecoderRuntimeClass")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Default);
        assert!(interface.name.namespace == "Windows.Foundation");
        assert!(interface.name.name == "IWwwFormUrlDecoderRuntimeClass");
        assert!(interface.name.generics.is_empty());

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IIterable`1")
            .unwrap();

        assert!(interface.kind == InterfaceKind::NonDefault);
        assert!(interface.name.namespace == "Windows.Foundation.Collections");
        assert!(interface.name.name == "IIterable`1");
        assert!(interface.name.generics.len() == 1);

        let entry = match &interface.name.generics[0] {
            TypeKind::Interface(entry) => entry,
            _ => panic!("Wrong type"),
        };

        assert!(entry.namespace == "Windows.Foundation");
        assert!(entry.name == "IWwwFormUrlDecoderEntry");

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IVectorView`1")
            .unwrap();

        assert!(interface.kind == InterfaceKind::NonDefault);
        assert!(interface.name.namespace == "Windows.Foundation.Collections");
        assert!(interface.name.name == "IVectorView`1");
        assert!(interface.name.generics.len() == 1);

        let entry = match &interface.name.generics[0] {
            TypeKind::Interface(entry) => entry,
            _ => panic!("Wrong type"),
        };

        assert!(entry.namespace == "Windows.Foundation");
        assert!(entry.name == "IWwwFormUrlDecoderEntry");
    }

    #[test]
    fn test_class_with_bases() {
        let t = class(("Windows.UI.Composition", "SpriteVisual"));

        assert!(t.name.namespace == "Windows.UI.Composition");
        assert!(t.name.name == "SpriteVisual");
        assert!(t.name.generics.is_empty());

        assert!(t.bases.len() == 3);

        assert!(t.bases[0].namespace == "Windows.UI.Composition");
        assert!(t.bases[0].name == "ContainerVisual");
        assert!(t.bases[0].generics.is_empty());

        assert!(t.bases[1].namespace == "Windows.UI.Composition");
        assert!(t.bases[1].name == "Visual");
        assert!(t.bases[1].generics.is_empty());

        assert!(t.bases[2].namespace == "Windows.UI.Composition");
        assert!(t.bases[2].name == "CompositionObject");
        assert!(t.bases[2].generics.is_empty());
    }
}
