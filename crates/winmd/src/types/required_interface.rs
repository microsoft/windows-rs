use crate::tables::*;
use crate::types::*;
use crate::TypeReader;
use proc_macro2::TokenStream;
use std::collections::*;
use std::iter::FromIterator;

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
        let guid = TypeGuid::from_type_def(reader, def);

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
        let guid = TypeGuid::from_type_def(reader, name.def);

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

    pub fn append(reader: &TypeReader, name: &TypeName, interfaces: &mut Vec<RequiredInterface>) {
        let mut map = RequiredInterfaces::default();
        map.insert_required(reader, name);

        // Ensures that the default interface (if any) is first in line.
        for (name, kind) in map.0 {
            let required = RequiredInterface::from_type_name_and_kind(reader, name, kind);

            if kind == InterfaceKind::Default {
                interfaces.insert(0, required);
            } else {
                interfaces.push(required);
            }
        }
    }

    pub fn to_abi_method_tokens(&self, calling_namespace: &str) -> TokenStream {
        TokenStream::from_iter(
            self.methods
                .iter()
                .map(|method| method.to_abi_tokens(calling_namespace)),
        )
    }
}

impl RequiredInterfaces {
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
            let kind = kind(reader, required);
            self.insert_type_name(reader, name, kind);
        }
    }
}

fn kind(reader: &TypeReader, required: InterfaceImpl) -> InterfaceKind {
    for attribute in required.attributes(reader) {
        let name = attribute.name(reader);

        if matches!(
            name,
            ("Windows.Foundation.Metadata", "DefaultAttribute")
                | ("Windows.Foundation.Metadata", "OverridableAttribute")
        ) {
            return InterfaceKind::Default;
        }
    }

    InterfaceKind::NonDefault
}

pub fn to_method_tokens(
    calling_namespace: &str,
    interfaces: &Vec<RequiredInterface>,
) -> TokenStream {
    let mut tokens = Vec::new();
    let mut names = BTreeSet::new();

    for interface in interfaces {
        for method in &interface.methods {
            // If there are any collisions just drop and caller can QI for the actual interface.
            if names.contains(&method.name) {
                continue;
            }

            names.insert(&method.name);

            tokens.push(match interface.kind {
                InterfaceKind::Default => method.to_default_tokens(calling_namespace, interface),
                InterfaceKind::NonDefault | InterfaceKind::Overrides => {
                    method.to_non_default_tokens(calling_namespace)
                }
                InterfaceKind::Constructors => method.to_constructor_tokens(calling_namespace),
                InterfaceKind::Statics => method.to_static_tokens(calling_namespace),
            });
        }
    }

    TokenStream::from_iter(tokens)
}
