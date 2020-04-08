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
}

impl Class {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let mut interfaces = Interface::interfaces(reader, def, &Vec::new());
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

        for attribute in def.attributes(reader) {
            match attribute.name(reader) {
                ("Windows.Foundation.Metadata", "StaticAttribute") => {}
                ("Windows.Foundation.Metadata", "ActivatableAttribute") => {}
                _ => {}
            }
        }

        Self {
            name,
            interfaces,
            bases,
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
            pub struct #name {

            }
        }
    }
}

#[test]
fn test_class_with_generic_interface() {
    let reader = &TypeReader::from_os();
    let def = reader.resolve(("Windows.Foundation", "WwwFormUrlDecoder"));
    let t = def.into_type(reader);

    let name = t.name();
    assert!(name.namespace == "Windows.Foundation");
    assert!(name.name == "WwwFormUrlDecoder");
    assert!(name.generics.is_empty());

    assert!(name.def == def);

    // let t = match t {
    //     Type::Class(t) => t,
    //     _ => panic!("Wrong type"),
    // };

    // TODO: Assert required interfaces...
    // defualt: IWwwFormUrlDecoderRuntimeClass
    // IIterable<IWwwFormUrlDecoderEntry>
    // IVectorView<IWwwFormUrlDecoderEntry>>
}

#[test]
fn test_class_with_bases() {
    let reader = &TypeReader::from_os();
    let def = reader.resolve(("Windows.UI.Composition", "SpriteVisual"));

    let t = match def.into_type(reader) {
        Type::Class(t) => t,
        _ => panic!("Wrong type"),
    };

    assert!(t.name.namespace == "Windows.UI.Composition");
    assert!(t.name.name == "SpriteVisual");
    assert!(t.name.generics.is_empty());

    assert!(t.name.def == def);
    assert!(t.bases.len() == 3);

    assert!(t.bases[0].namespace == "Windows.UI.Composition");
    assert!(t.bases[0].name == "ContainerVisual");
    assert!(t.bases[0].generics.is_empty());
    assert!(t.bases[0].def == reader.resolve(("Windows.UI.Composition", "ContainerVisual")));

    assert!(t.bases[1].namespace == "Windows.UI.Composition");
    assert!(t.bases[1].name == "Visual");
    assert!(t.bases[1].generics.is_empty());
    assert!(t.bases[1].def == reader.resolve(("Windows.UI.Composition", "Visual")));

    assert!(t.bases[2].namespace == "Windows.UI.Composition");
    assert!(t.bases[2].name == "CompositionObject");
    assert!(t.bases[2].generics.is_empty());
    assert!(t.bases[2].def == reader.resolve(("Windows.UI.Composition", "CompositionObject")));
}
