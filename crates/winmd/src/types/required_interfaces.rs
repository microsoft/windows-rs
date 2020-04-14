use crate::tables::*;
use crate::types::*;
use crate::TypeReader;
use std::collections::*;

#[derive(Debug)]
pub struct RequiredInterface {
    pub name: TypeName,
    pub guid: TypeGuid,
    pub methods: Vec<Method>,
    pub kind: InterfaceKind,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InterfaceKind {
    Default,
    NonDefault,
    Overrides,
    Constructors,
    Statics,
}

#[derive(Default, Debug)]
struct RequiredInterfaces(pub BTreeMap<TypeName, InterfaceKind>);

impl RequiredInterface {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);

        let guid = TypeGuid::from_args(
            def.attribute(reader, ("Windows.Foundation.Metadata", "GuidAttribute"))
                .args(reader),
        );

        let methods = def
            .methods(reader)
            .map(|method| Method::from_method_def(reader, method, &name.generics))
            .collect();

        Self {
            name,
            guid,
            methods,
            kind: InterfaceKind::NonDefault,
        }
    }

    fn from_type_name_and_kind(reader: &TypeReader, name: TypeName, kind: InterfaceKind) -> Self {
        let guid = TypeGuid::from_args(
            name.def
                .attribute(reader, ("Windows.Foundation.Metadata", "GuidAttribute"))
                .args(reader),
        );

        let methods = name
            .def
            .methods(reader)
            .map(|method| Method::from_method_def(reader, method, &name.generics))
            .collect();

        Self {
            name,
            guid,
            methods,
            kind,
        }
    }

    pub fn all(reader: &TypeReader, name: &TypeName) -> Vec<RequiredInterface> {
        let mut interfaces = RequiredInterfaces::default();
        interfaces.insert_required(reader, name);
        interfaces
            .0
            .into_iter()
            .map(move |(name, kind)| RequiredInterface::from_type_name_and_kind(reader, name, kind))
            .collect()
    }
}

impl RequiredInterfaces {
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
