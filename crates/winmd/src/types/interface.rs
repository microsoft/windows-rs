use crate::codes::*;
use crate::tables::*;
use crate::types::*;
use crate::TypeReader;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Interface {
    pub name: TypeName,
    pub guid: TypeGuid,
    pub methods: Vec<Method>,
    pub kind: InterfaceKind,
    // pub exclusive: bool,
    pub interfaces: Vec<Interface>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InterfaceKind {
    Default,
    NonDefault,
    Overrides,
    Constructors,
    Statics,
}

impl Interface {
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

        // TODO: too wordy - this should be simpler
        let interfaces = RequiredInterfaces::from_type_name(reader, &name).into_interfaces(reader);

        Self {
            name,
            guid,
            methods,
            interfaces,
            kind: InterfaceKind::NonDefault,
        }
    }

    pub fn from_type_name_and_kind(
        reader: &TypeReader,
        name: TypeName,
        kind: InterfaceKind,
    ) -> Self {
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
            interfaces: Vec::new(),
            kind,
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.interfaces
            .iter()
            .flat_map(|i| i.name.dependencies())
            .chain(self.methods.iter().flat_map(|m| m.dependencies()))
            .collect()
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = self.name.ident();
        let phantoms = self.name.phantoms();
        let constraints = self.name.constraints();
        let projected_methods = TokenStream::new();

        quote! {
            #[repr(transparent)]
            #[derive(Default, Clone)]
            pub struct #name where #constraints {
                ptr: winrt::IUnknown,
                #phantoms
            }
            impl<#constraints> #name {
                #projected_methods
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interface((namespace, type_name): (&str, &str)) -> Interface {
        let reader = &TypeReader::from_os();
        let t = reader.resolve_type((namespace, type_name));

        match t {
            Type::Interface(t) => t,
            _ => panic!("Type not an interface"),
        }
    }

    #[test]
    fn test_stringable() {
        let t = interface(("Windows.Foundation", "IStringable"));

        assert!(t.name.namespace == "Windows.Foundation");
        assert!(t.name.name == "IStringable");
        assert!(t.name.generics.is_empty());

        assert!(t.methods.len() == 1);
        let method = &t.methods[0];
        assert!(method.name == "to_string");
        assert!(method.kind == MethodKind::Normal);

        assert!(method.params.is_empty());
        let param = method.return_type.as_ref().unwrap();
        assert!(param.kind == TypeKind::String);

        assert!(format!("{:#?}", &t.guid) == "96369F54-8EB6-48F0-ABCE-C1B211E627C3");
    }

    #[test]
    fn test_async_action() {
        let t = interface(("Windows.Foundation", "IAsyncAction"));

        assert!(t.name.namespace == "Windows.Foundation");
        assert!(t.name.name == "IAsyncAction");
        assert!(t.name.generics.is_empty());

        assert!(t.interfaces.len() == 1);

        assert!(t.interfaces[0].name.namespace == "Windows.Foundation");
        assert!(t.interfaces[0].name.name == "IAsyncInfo");
        assert!(t.interfaces[0].name.generics.is_empty());
    }

    #[test]
    fn test_observable_map() {
        let t = interface(("Windows.Foundation.Collections", "IObservableMap`2"));

        assert!(t.name.namespace == "Windows.Foundation.Collections");
        assert!(t.name.name == "IObservableMap`2");
        assert!(t.name.generics.len() == 2);
        assert!(t.name.generics[0] == TypeKind::Generic("K".to_string()));
        assert!(t.name.generics[1] == TypeKind::Generic("V".to_string()));

        assert!(t.methods.len() == 2);
        assert!(t.methods[0].name == "map_changed");
        assert!(t.methods[1].name == "remove_map_changed");

        assert!(t.interfaces.len() == 2);

        let map = t
            .interfaces
            .iter()
            .find(|required| required.name.name == "IMap`2")
            .unwrap();

        assert!(map.name.namespace == "Windows.Foundation.Collections");
        assert!(map.name.name == "IMap`2");
        assert!(map.name.generics.len() == 2);
        assert!(map.name.generics[0] == TypeKind::Generic("K".to_string()));
        assert!(map.name.generics[1] == TypeKind::Generic("V".to_string()));

        assert!(map.interfaces.len() == 0);

        let iterable = t
            .interfaces
            .iter()
            .find(|required| required.name.name == "IIterable`1")
            .unwrap();

        assert!(iterable.interfaces.len() == 0);
        assert!(iterable.name.namespace == "Windows.Foundation.Collections");
        assert!(iterable.name.name == "IIterable`1");
        assert!(iterable.name.generics.len() == 1);

        let pair = match &iterable.name.generics[0] {
            TypeKind::Interface(pair) => pair,
            _ => panic!("Wrong type"),
        };

        assert!(pair.namespace == "Windows.Foundation.Collections");
        assert!(pair.name == "IKeyValuePair`2");
        assert!(pair.generics[0] == TypeKind::Generic("K".to_string()));
        assert!(pair.generics[1] == TypeKind::Generic("V".to_string()));
    }
}
