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
    Statics,
    Composable,
}

impl RequiredInterface {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef, calling_namespace: &str) -> Self {
        let name = TypeName::from_type_def(reader, def, calling_namespace);
        let guid = TypeGuid::from_type_def(reader, def);

        let mut methods = def
            .methods(reader)
            .map(|method| {
                Method::from_method_def(reader, method, &name.generics, calling_namespace)
            })
            .collect();

        rename_collisions(&mut methods);

        Self {
            name,
            guid,
            methods,
            kind: InterfaceKind::NonDefault,
        }
    }

    fn from_type_name_and_kind(
        reader: &TypeReader,
        name: TypeName,
        kind: InterfaceKind,
        generics: bool,
        calling_namespace: &str,
    ) -> Self {
        let guid = name.guid(reader, generics);

        let mut methods = name
            .def
            .methods(reader)
            .map(|method| {
                Method::from_method_def(reader, method, &name.generics, calling_namespace)
            })
            .collect();

        rename_collisions(&mut methods);

        Self {
            name,
            guid,
            methods,
            kind,
        }
    }

    pub fn append_default(
        reader: &TypeReader,
        name: &TypeName,
        interfaces: &mut Vec<RequiredInterface>,
    ) {
        let generics = !name.generics.is_empty();

        let mut map = RequiredInterfaces::default();
        map.insert_required(reader, name, &name.namespace);

        for (append_name, kind) in map.0 {
            let required = RequiredInterface::from_type_name_and_kind(
                reader,
                append_name,
                kind,
                generics,
                &name.namespace,
            );

            if kind == InterfaceKind::Default {
                interfaces.insert(0, required);
            } else {
                interfaces.push(required);
            }
        }
    }

    pub fn append_required(
        reader: &TypeReader,
        name: &TypeName,
        calling_namespace: &str,
        interfaces: &mut Vec<RequiredInterface>,
    ) {
        let generics = !name.generics.is_empty();

        let mut map = RequiredInterfaces::default();
        map.insert_required(reader, name, calling_namespace);

        for (append_name, kind) in map.0 {
            let mut kind = kind;

            if kind == InterfaceKind::Default {
                kind = InterfaceKind::NonDefault;
            }

            if interfaces.iter().any(|i| i.name == append_name) {
                continue;
            }

            interfaces.push(RequiredInterface::from_type_name_and_kind(
                reader,
                append_name,
                kind,
                generics,
                calling_namespace,
            ));
        }
    }

    pub fn to_abi_method_tokens(&self) -> TokenStream {
        TokenStream::from_iter(
            self.methods
                .iter()
                .map(|method| method.to_abi_tokens(&self.name)),
        )
    }

    pub fn to_conversions_tokens(
        &self,
        from: &TokenStream,
        constraints: &TokenStream,
    ) -> TokenStream {
        match self.kind {
            InterfaceKind::Default => {
                let into = &self.name.tokens;
                quote! {
                    impl<#constraints> ::std::convert::From<#from> for #into {
                        fn from(value: #from) -> Self {
                            unsafe { ::std::mem::transmute(value) }
                        }
                    }
                    impl<#constraints> ::std::convert::From<&#from> for #into {
                        fn from(value: &#from) -> Self {
                            ::std::convert::From::from(::std::clone::Clone::clone(value))
                        }
                    }
                    impl<'a, #constraints> ::std::convert::Into<::winrt::Param<'a, #into>> for #from {
                        fn into(self) -> ::winrt::Param<'a, #into> {
                            ::winrt::Param::Owned(::std::convert::Into::<#into>::into(self))
                        }
                    }
                    impl<'a, #constraints> ::std::convert::Into<::winrt::Param<'a, #into>> for &'a #from {
                        fn into(self) -> ::winrt::Param<'a, #into> {
                            ::winrt::Param::Owned(::std::convert::Into::<#into>::into(::std::clone::Clone::clone(self)))
                        }
                    }
                }
            }
            InterfaceKind::NonDefault => {
                let into = &self.name.tokens;
                quote! {
                    impl<#constraints> ::std::convert::From<#from> for #into {
                        fn from(value: #from) -> Self {
                            ::std::convert::From::from(&value)
                        }
                    }
                    impl<#constraints> ::std::convert::From<&#from> for #into {
                        fn from(value: &#from) -> Self {
                            <#from as ::winrt::ComInterface>::query(value)
                        }
                    }
                    impl<'a, #constraints> ::std::convert::Into<::winrt::Param<'a, #into>> for #from {
                        fn into(self) -> ::winrt::Param<'a, #into> {
                            ::winrt::Param::Owned(::std::convert::Into::<#into>::into(self))
                        }
                    }
                    impl<'a, #constraints> ::std::convert::Into<::winrt::Param<'a, #into>> for &'a #from {
                        fn into(self) -> ::winrt::Param<'a, #into> {
                            ::winrt::Param::Owned(::std::convert::Into::<#into>::into(::std::clone::Clone::clone(self)))
                        }
                    }
                }
            }
            _ => quote! {},
        }
    }
}

pub fn to_method_tokens(interfaces: &Vec<RequiredInterface>) -> TokenStream {
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
                InterfaceKind::Default => method.to_default_tokens(),
                InterfaceKind::NonDefault | InterfaceKind::Overrides => {
                    method.to_non_default_tokens(interface)
                }
                InterfaceKind::Statics => method.to_static_tokens(interface),
                InterfaceKind::Composable => method.to_composable_tokens(interface),
            });
        }
    }

    TokenStream::from_iter(tokens)
}

fn rename_collisions(methods: &mut Vec<Method>) {
    let mut names = BTreeSet::new();

    for method in methods {
        if names.contains(&method.name) {
            method.name = format!("{}2", method.name);
        } else {
            names.insert(&method.name);
        }
    }
}
