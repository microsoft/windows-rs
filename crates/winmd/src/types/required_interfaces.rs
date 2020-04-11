use crate::tables::*;
use crate::types::*;
use crate::TypeReader;
use std::collections::*;

#[derive(Default, Debug)]
pub struct RequiredInterfaces(pub BTreeMap<TypeName, InterfaceKind>);

impl RequiredInterfaces {
    pub fn required(reader: &TypeReader, name: &TypeName) -> Vec<Interface> {
        let mut interfaces = Self::default();
        interfaces.insert_required(reader, name);
        interfaces
            .0
            .into_iter()
            .map(move |(name, kind)| Interface::from_type_name_and_kind(reader, name, kind))
            .collect()
    }

    fn kind(reader: &TypeReader, required: InterfaceImpl) -> InterfaceKind {
        for attribute in required.attributes(reader) {
            let name = attribute.name(reader);

            if name == ("Windows.Foundation.Metadata", "DefaultAttribute") {
                return InterfaceKind::Default;
            }

            if name == ("Windows.Foundation.Metadata", "OverridableAttribute") {
                return InterfaceKind::Default;
            }
        }

        InterfaceKind::NonDefault
    }

    fn insert_type_name(&mut self, reader: &TypeReader, name: TypeName, kind: InterfaceKind) {
        if !self.0.contains_key(&name) {
            self.insert_required(reader, &name);
            self.0.insert(name, kind);
        }
    }

    fn insert_required(&mut self, reader: &TypeReader, name: &TypeName) {
        for required in name.def.interfaces(reader) {
            let name =
                TypeName::from_type_def_or_ref(reader, required.interface(reader), &name.generics);
            let kind = Self::kind(reader, required);
            self.insert_type_name(reader, name, kind);
        }
    }
}
