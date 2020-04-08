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
    pub fn from_type_def(reader: &TypeReader, def: TypeDef, generics: &Vec<TypeKind>) -> Self {
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
            #[repr(C)]
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

#[test]
fn test_stringable() {
    let reader = &TypeReader::from_os();
    let def = reader.resolve(("Windows.Foundation", "IStringable"));
    let t = def.into_type(reader);

    let name = t.name();
    assert!(name.namespace == "Windows.Foundation");
    assert!(name.name == "IStringable");
    assert!(name.generics.is_empty());

    assert!(name.def == def);

    let t = match t {
        Type::Interface(t) => t,
        _ => panic!("Wrong type"),
    };

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
    let reader = &TypeReader::from_os();
    let def = reader.resolve(("Windows.Foundation", "IAsyncAction"));

    let t = match def.into_type(reader) {
        Type::Interface(t) => t,
        _ => panic!("Wrong type"),
    };

    assert!(t.name.namespace == "Windows.Foundation");
    assert!(t.name.name == "IAsyncAction");
    assert!(t.name.generics.is_empty());

    assert!(t.name.def == def);
    assert!(t.interfaces.len() == 1);

    assert!(t.interfaces[0].name.namespace == "Windows.Foundation");
    assert!(t.interfaces[0].name.name == "IAsyncInfo");
    assert!(t.interfaces[0].name.generics.is_empty());
}

#[test]
fn test_observable_map() {
    // TODO: split into smaller focused tests
    let reader = &TypeReader::from_os();
    let def = reader.resolve(("Windows.Foundation.Collections", "IObservableMap`2"));
    let t = def.into_type(reader);
    let name = t.name();

    assert!(name.namespace == "Windows.Foundation.Collections");
    assert!(name.name == "IObservableMap`2");
    assert!(name.generics.len() == 2);
    assert!(name.generics[0] == TypeKind::Generic("K".to_string()));
    assert!(name.generics[1] == TypeKind::Generic("V".to_string()));

    assert!(name.def == def);

    let t = match t {
        Type::Interface(t) => t,
        _ => panic!("Wrong type"),
    };

    assert!(t.methods.len() == 2);

    let method = &t.methods[0];
    assert!(method.name == "map_changed");
    assert!(method.kind == MethodKind::Add);
    assert!(method.params.len() == 1);

    let handler = &method.params[0];
    assert!(handler.array == false);
    assert!(handler.input == true);
    assert!(handler.by_ref == false);

    let handler = match &handler.kind {
        TypeKind::Delegate(delegate) => delegate,
        _ => panic!("Wrong type"),
    };

    assert!(handler.namespace == "Windows.Foundation.Collections");
    assert!(handler.name == "MapChangedEventHandler`2");
    assert!(handler.generics.len() == 2);
    assert!(handler.generics[0] == TypeKind::Generic("K".to_string()));
    assert!(handler.generics[1] == TypeKind::Generic("V".to_string()));
    assert!(
        handler.def
            == reader.resolve(("Windows.Foundation.Collections", "MapChangedEventHandler`2"))
    );

    let token = method.return_type.as_ref().unwrap();
    assert!(token.array == false);
    assert!(token.input == false);
    assert!(token.by_ref == true);

    let token = match &token.kind {
        TypeKind::Struct(token) => token,
        _ => panic!("Wrong type"),
    };

    assert!(token.namespace == "Windows.Foundation");
    assert!(token.name == "EventRegistrationToken");
    assert!(token.generics.is_empty());
    assert!(token.def == reader.resolve(("Windows.Foundation", "EventRegistrationToken")));

    let method = &t.methods[1];
    assert!(method.name == "remove_map_changed");
    assert!(method.kind == MethodKind::Remove);
    assert!(method.params.len() == 1);

    let token = &method.params[0];
    assert!(token.array == false);
    assert!(token.input == true);
    assert!(token.by_ref == false);

    let token = match &token.kind {
        TypeKind::Struct(token) => token,
        _ => panic!("Wrong type"),
    };

    assert!(token.namespace == "Windows.Foundation");
    assert!(token.name == "EventRegistrationToken");
    assert!(token.generics.is_empty());
    assert!(token.def == reader.resolve(("Windows.Foundation", "EventRegistrationToken")));

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
