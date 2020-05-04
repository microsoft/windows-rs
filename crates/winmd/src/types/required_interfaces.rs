use crate::tables::*;
use crate::types::*;
use crate::TypeReader;
use std::collections::*;

#[derive(Default, Debug)]
pub struct RequiredInterfaces(pub BTreeMap<TypeName, InterfaceKind>);

impl RequiredInterfaces {
    fn insert_type_name(&mut self, reader: &TypeReader, name: TypeName, kind: InterfaceKind) {
        if !self.0.contains_key(&name) {
            self.insert_required(reader, &name);
            self.0.insert(name, kind);
        } else if kind == InterfaceKind::Default {
            self.0.insert(name, kind);
        }
    }

    pub fn insert_required(&mut self, reader: &TypeReader, name: &TypeName) {
        for required in name.def.interfaces(reader) {
            let name =
                TypeName::from_type_def_or_ref(reader, required.interface(reader), &name.generics);
            let kind = kind(reader, required);
            self.insert_type_name(reader, name, kind);
        }
    }
}

fn kind(reader: &TypeReader, required: InterfaceImpl) -> InterfaceKind {
    for attribute in required.attributes(reader) {
        let name = attribute.name(reader);

        match name {
            ("Windows.Foundation.Metadata", "DefaultAttribute") => return InterfaceKind::Default,
            _ => {}
        }
    }

    InterfaceKind::NonDefault
}
