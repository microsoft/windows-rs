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
    // pub default: bool,
    // pub exclusive: bool,
    // pub constructors: bool,
    // pub statics: bool,
    // pub overrides: bool,
    pub interfaces: Vec<Interface>,
}

impl Interface {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef, _generics: &Vec<TypeKind>) -> Self {
        let name = TypeName::from_type_def(reader, def); // TODO: generics above needs to feed in here to resolve any generics
        let guid = TypeGuid::from_args(
            def.attribute(reader, ("Windows.Foundation.Metadata", "GuidAttribute"))
                .args(reader),
        );
        let methods = def
            .methods(reader)
            .map(|method| Method::from_method_def(reader, method, &name.generics))
            .collect();
        let interfaces = name.interfaces(reader);
        Self {
            name,
            guid,
            methods,
            interfaces,
        }
    }

    pub fn interfaces(
        reader: &TypeReader,
        def: TypeDef,
        generics: &Vec<TypeKind>,
    ) -> Vec<Interface> {
        def.interfaces(reader)
            .map(|interface| Interface::from_interface_impl(reader, interface, generics))
            .collect()
    }
    fn from_type_ref(reader: &TypeReader, type_ref: TypeRef, generics: &Vec<TypeKind>) -> Self {
        Self::from_type_def(reader, type_ref.resolve(reader), generics)
    }

    fn from_type_spec(reader: &TypeReader, spec: TypeSpec, generics: &Vec<TypeKind>) -> Self {
        let name = TypeName::from_type_spec(reader, spec, generics);
        let guid = TypeGuid::new(); // TODO: Generate generic guid specialization
        let methods = Vec::new();
        let interfaces = name.interfaces(reader);
        Self {
            name,
            guid,
            methods,
            interfaces,
        }
    }

    fn from_type_def_or_ref(
        reader: &TypeReader,
        code: TypeDefOrRef,
        generics: &Vec<TypeKind>,
    ) -> Self {
        match code {
            TypeDefOrRef::TypeDef(value) => Self::from_type_def(reader, value, generics),
            TypeDefOrRef::TypeRef(value) => Self::from_type_ref(reader, value, generics),
            TypeDefOrRef::TypeSpec(value) => Self::from_type_spec(reader, value, generics),
        }
    }

    pub fn from_interface_impl(
        reader: &TypeReader,
        key: InterfaceImpl,
        generics: &Vec<TypeKind>,
    ) -> Self {
        // TODO: flip default/exclusive/overridable bits as needed
        Self::from_type_def_or_ref(reader, key.interface(reader), generics)
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
        let def = reader.resolve((namespace, type_name));

        match def.into_type(reader) {
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

        assert!(t.interfaces.len() == 1);
        let map = &t.interfaces[0];

        assert!(map.name.namespace == "Windows.Foundation.Collections");
        assert!(map.name.name == "IMap`2");
        assert!(map.name.generics.len() == 2);
        assert!(map.name.generics[0] == TypeKind::Generic("K".to_string()));
        assert!(map.name.generics[1] == TypeKind::Generic("V".to_string()));

        assert!(map.interfaces.len() == 1);
        let iterable = &map.interfaces[0];

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
