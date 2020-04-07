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
        let mut interfaces: Vec<Interface> = def
            .interfaces(reader)
            .map(|interface| Interface::from_interface_impl(reader, interface))
            .collect();
        let mut bases = Vec::new();
        let mut base = def;

        loop {
            let (namespace, name) = base.extends(reader).name(reader);

            if (namespace, name) == ("System", "Object") {
                break;
            }

            base = reader.resolve((namespace, name));

            interfaces.extend(
                base.interfaces(reader)
                    .map(|interface| Interface::from_interface_impl(reader, interface)),
            );

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
fn can_read_class_with_generic_interface_from_reader() {
    let winmd_files = crate::load_winmd::from_os();
    let reader = &TypeReader::new(winmd_files);
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
