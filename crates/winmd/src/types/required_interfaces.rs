use crate::tables::*;
use crate::types::*;
use crate::TypeReader;
use std::collections::*;

#[derive(Default, Debug)]
pub struct RequiredInterfaces(pub BTreeMap<TypeName, InterfaceKind>);

impl RequiredInterfaces {
    pub fn from_type_name(reader: &TypeReader, name: &TypeName) -> Self {
        let mut interfaces = Self::default();
        interfaces.insert_required(reader, name);
        interfaces
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

    pub fn into_interfaces(self, reader: &TypeReader) -> Vec<Interface> {
        self.0
            .into_iter()
            .map(move |(name, kind)| Interface::from_type_name_and_kind(reader, name, kind))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri() {
        let reader = &TypeReader::from_os();
        let t = reader.resolve_type(("Windows.Foundation", "Uri"));
        let required = RequiredInterfaces::from_type_name(reader, &t.name());

        assert!(required.0.len() == 3);
        let mut iter = required.0.iter();

        let (name, kind) = iter.next().unwrap();
        assert!(*kind == InterfaceKind::NonDefault);
        assert!(name.name == "IStringable");

        let (name, kind) = iter.next().unwrap();
        assert!(*kind == InterfaceKind::Default);
        assert!(name.name == "IUriRuntimeClass");

        let (name, kind) = iter.next().unwrap();
        assert!(*kind == InterfaceKind::NonDefault);
        assert!(name.name == "IUriRuntimeClassWithAbsoluteCanonicalUri");
    }

    #[test]
    fn test_uri_decoder() {
        let reader = &TypeReader::from_os();
        let decoder = reader.resolve_type(("Windows.Foundation", "WwwFormUrlDecoder"));
        let entry = reader.resolve_type(("Windows.Foundation", "IWwwFormUrlDecoderEntry"));
        let required = RequiredInterfaces::from_type_name(reader, &decoder.name()).0;
        assert!(required.len() == 3);

        let (name, kind) = required
            .iter()
            .find(|(k, _)| k.name == "IWwwFormUrlDecoderRuntimeClass")
            .unwrap();
        assert!(*kind == InterfaceKind::Default);
        assert!(name.generics.len() == 0);

        let (name, kind) = required
            .iter()
            .find(|(k, _)| k.name == "IIterable`1")
            .unwrap();
        assert!(*kind == InterfaceKind::NonDefault);
        assert!(name.generics.len() == 1);
        assert!(name.generics[0] == TypeKind::Interface(entry.name().clone()));

        let (name, kind) = required
            .iter()
            .find(|(k, _)| k.name == "IVectorView`1")
            .unwrap();
        assert!(*kind == InterfaceKind::NonDefault);
        assert!(name.generics.len() == 1);
        assert!(name.generics[0] == TypeKind::Interface(entry.name().clone()));
    }

    #[test]
    fn test_async_action() {
        let reader = &TypeReader::from_os();
        let t = reader.resolve_type(("Windows.Foundation", "IAsyncAction"));
        let required = RequiredInterfaces::from_type_name(reader, &t.name());

        assert!(required.0.len() == 1);
        let mut iter = required.0.iter();

        let (name, kind) = iter.next().unwrap();
        assert!(*kind == InterfaceKind::NonDefault);
        assert!(name.name == "IAsyncInfo");
    }

    #[test]
    fn test_observable_map() {
        let reader = &TypeReader::from_os();
        let t = reader.resolve_type(("Windows.Foundation.Collections", "IObservableMap`2"));
        let required = RequiredInterfaces::from_type_name(reader, &t.name()).0;
        assert!(required.len() == 2);

        let (name, kind) = required
            .iter()
            .find(|(k, _)| k.name == "IIterable`1")
            .unwrap();
        assert!(*kind == InterfaceKind::NonDefault);
        assert!(name.generics.len() == 1);
        let pair = match &name.generics[0] {
            TypeKind::Interface(pair) => pair,
            _ => panic!("Wrong type"),
        };

        assert!(pair.namespace == "Windows.Foundation.Collections");
        assert!(pair.name == "IKeyValuePair`2");
        assert!(pair.generics[0] == TypeKind::Generic("K".to_string()));
        assert!(pair.generics[1] == TypeKind::Generic("V".to_string()));

        let (name, kind) = required.iter().find(|(k, _)| k.name == "IMap`2").unwrap();
        assert!(*kind == InterfaceKind::NonDefault);
        assert!(name.generics.len() == 2);
        assert!(name.generics[0] == TypeKind::Generic("K".to_string()));
        assert!(name.generics[1] == TypeKind::Generic("V".to_string()));
    }
}
