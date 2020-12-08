use crate::*;
use squote::{quote, TokenStream};
use std::collections::*;

#[derive(Debug)]
pub struct RequiredInterface {
    pub name: TypeName,
    pub methods: Vec<Method>,
    pub kind: InterfaceKind,
}

impl PartialEq for RequiredInterface {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for RequiredInterface {}

impl PartialOrd for RequiredInterface {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RequiredInterface {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl RequiredInterface {
    fn from_type_def(
        def: &winmd::TypeDef,
        calling_namespace: &'static str,
        kind: InterfaceKind,
    ) -> Self {
        let name = TypeName::from_type_def(def, calling_namespace);
        Self::from_type_name_and_kind(name, kind, calling_namespace)
    }

    fn from_type_name_and_kind(
        name: TypeName,
        kind: InterfaceKind,
        calling_namespace: &'static str,
    ) -> Self {
        let methods = name
            .def
            .methods()
            .enumerate()
            .map(|(count, method)| {
                Method::from_method_def(
                    &method,
                    (count + 6) as u32,
                    &name.generics,
                    calling_namespace,
                )
            })
            .collect();

        Self {
            name,
            methods,
            kind,
        }
    }

    pub fn gen_conversions(&self, from: &TokenStream, constraints: &TokenStream) -> TokenStream {
        match self.kind {
            InterfaceKind::Default => {
                let into = self.name.gen();
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
                let into = self.name.gen();
                quote! {
                    impl<#constraints> ::std::convert::From<#from> for #into {
                        fn from(value: #from) -> Self {
                            ::std::convert::From::from(&value)
                        }
                    }
                    impl<#constraints> ::std::convert::From<&#from> for #into {
                        fn from(value: &#from) -> Self {
                            ::winrt::Interface::cast(value).unwrap()
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
            _ => TokenStream::new(),
        }
    }
}

pub fn add_type(
    vec: &mut Vec<RequiredInterface>,
    def: &winmd::TypeDef,
    calling_namespace: &'static str,
    kind: InterfaceKind,
) {
    vec.push(RequiredInterface::from_type_def(
        def,
        calling_namespace,
        kind,
    ));
}

pub fn add_dependencies(
    vec: &mut Vec<RequiredInterface>,
    name: &TypeName,
    calling_namespace: &'static str,
    strip_default: bool,
) {
    for required in name.def.interfaces() {
        let is_default = required.is_default();
        let required = required.interface();

        let required_name =
            TypeName::from_type_def_or_ref(&required, &name.generics, calling_namespace);

        if let Some(index) = vec.iter().position(|i| i.name == required_name) {
            if !strip_default && vec[index].kind == InterfaceKind::NonDefault && is_default {
                vec[index].kind = InterfaceKind::Default;
            }
        } else {
            let kind = if !strip_default && is_default {
                InterfaceKind::Default
            } else {
                InterfaceKind::NonDefault
            };

            add_dependencies(vec, &required_name, calling_namespace, strip_default);

            vec.push(RequiredInterface::from_type_name_and_kind(
                required_name,
                kind,
                calling_namespace,
            ));
        }
    }
}

pub fn gen_method(interfaces: &Vec<RequiredInterface>) -> TokenStream {
    let mut tokens = TokenStream::new();

    for interface in interfaces {
        for method in &interface.methods {
            tokens.combine(&method.gen_method(&interface.name, interface.kind));
        }
    }

    tokens
}

pub fn rename_collisions(interfaces: &mut Vec<RequiredInterface>) {
    // First sort interfaces to ensure a stable method renaming across versions.
    // TODO: Once fast abi support is added, sorting here will be unnecessary.
    // https://github.com/microsoft/winrt-rs/issues/235
    interfaces.sort();

    let mut count = BTreeMap::new();

    for interface in interfaces {
        for method in &mut interface.methods {
            let count = count.entry(&method.name).or_insert(0);
            *count += 1;
            method.overload = *count;
        }
    }
}
