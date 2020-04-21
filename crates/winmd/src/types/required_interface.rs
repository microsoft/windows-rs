use crate::tables::*;
use crate::types::*;
use crate::TypeReader;
use proc_macro2::TokenStream;
use quote::quote;
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

    pub fn to_conversions_tokens(
        &self,
        calling_namespace: &str,
        from: &TokenStream,
        constraints: &TokenStream,
    ) -> TokenStream {
        match self.kind {
            InterfaceKind::Default => {
                let into = self.name.to_tokens(calling_namespace);
                quote! {
                    impl<#constraints> From<#from> for #into {
                        fn from(value: #from) -> #into {
                            unsafe { std::mem::transmute(value) }
                        }
                    }
                    impl<#constraints> From<&#from> for #into {
                        fn from(value: &#from) -> #into {
                            unsafe { std::mem::transmute(value.clone()) }
                        }
                    }
                    // impl<'a, #constraints> Into<::winrt::Param<'a, #into>> for #from {
                    //     fn into(self) -> ::winrt::Param<'a, #into> {
                    //         ::winrt::Param::Owned(self.into())
                    //     }
                    // }
                    // impl<'a, #constraints> Into<::winrt::Param<'a, #into>> for &'a #from {
                    //     fn into(self) -> ::winrt::Param<'a, #into> {
                    //         ::winrt::Param::Owned(self.into())
                    //     }
                    // }
                }
            }
            InterfaceKind::NonDefault => {
                let into = self.name.to_tokens(calling_namespace);
                quote! {
                    impl<#constraints> From<#from> for #into {
                        fn from(value: #from) -> #into {
                            #into::from(&value)
                        }
                    }
                    impl<#constraints> From<&#from> for #into {
                        fn from(value: &#from) -> #into {
                            ::winrt::safe_query(value)
                        }
                    }
                    // impl<'a, #constraints> Into<::winrt::Param<'a, #into>> for #from {
                    //     fn into(self) -> ::winrt::Param<'a, #into> {
                    //         ::winrt::Param::Owned(self.into())
                    //     }
                    // }
                    // impl<'a, #constraints> Into<::winrt::Param<'a, #into>> for &'a #from {
                    //     fn into(self) -> ::winrt::Param<'a, #into> {
                    //         ::winrt::Param::Owned(self.into())
                    //     }
                    // }
                }
            }
            _ => quote! {},
        }
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
                    method.to_non_default_tokens(calling_namespace, interface)
                }
                // TODO: do we need these two categories if we're only going to fold them anyway?
                InterfaceKind::Constructors => {
                    method.to_static_tokens(calling_namespace, interface)
                }
                InterfaceKind::Statics => method.to_static_tokens(calling_namespace, interface),
            });
        }
    }

    TokenStream::from_iter(tokens)
}
