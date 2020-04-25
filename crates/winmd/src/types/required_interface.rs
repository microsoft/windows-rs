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
}

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

    fn from_type_name_and_kind(
        reader: &TypeReader,
        name: TypeName,
        kind: InterfaceKind,
        generics: bool,
    ) -> Self {
        let guid = name.guid(reader, generics);

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

    pub fn append_default(
        reader: &TypeReader,
        name: &TypeName,
        interfaces: &mut Vec<RequiredInterface>,
    ) {
        let generics = !name.generics.is_empty();

        let mut map = RequiredInterfaces::default();
        map.insert_required(reader, name);

        for (name, kind) in map.0 {
            let required = RequiredInterface::from_type_name_and_kind(reader, name, kind, generics);

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
        interfaces: &mut Vec<RequiredInterface>,
    ) {
        let generics = !name.generics.is_empty();

        let mut map = RequiredInterfaces::default();
        map.insert_required(reader, name);

        for (name, kind) in map.0 {
            let mut kind = kind;

            if kind == InterfaceKind::Default {
                kind = InterfaceKind::NonDefault;
            }

            interfaces.push(RequiredInterface::from_type_name_and_kind(
                reader, name, kind, generics,
            ));
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
                            #into::from(value.clone())
                        }
                    }
                }
            }
            InterfaceKind::NonDefault => {
                let into = self.name.to_tokens(calling_namespace);
                if self.name.generics.is_empty() {
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
                    }
                } else {
                    let guid = self.guid.to_tokens();

                    quote! {
                        impl<#constraints> From<#from> for #into {
                            fn from(value: #from) -> #into {
                                #into::from(&value)
                            }
                        }
                        impl<#constraints> From<&#from> for #into {
                            fn from(value: &#from) -> #into {
                                const GUID: ::winrt::Guid = ::winrt::Guid::from_values(#guid);
                                unsafe { ::winrt::unsafe_query(value, &GUID) }
                            }
                        }
                    }
                }
            }
            _ => quote! {},
        }
    }
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
                InterfaceKind::Statics => method.to_static_tokens(calling_namespace, interface),
            });
        }
    }

    TokenStream::from_iter(tokens)
}
