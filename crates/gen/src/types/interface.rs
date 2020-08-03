use crate::tables::*;
use crate::types::debug;
use crate::types::*;
use crate::*;
use squote::{quote, TokenStream};
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Interface {
    pub name: TypeName,
    pub interfaces: Vec<RequiredInterface>,
    pub signature: String,
}

impl Interface {
    pub fn from_type_name(reader: &TypeReader, name: TypeName) -> Self {
        let mut interfaces = Vec::new();

        add_type(
            &mut interfaces,
            reader,
            name.def,
            &name.namespace,
            InterfaceKind::Default,
        );

        add_dependencies(&mut interfaces, reader, &name, &name.namespace, true);
        let signature = name.base_interface_signature(reader);

        Self {
            name,
            interfaces,
            signature,
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        let mut dependencies = Vec::new();

        for interface in &self.interfaces {
            dependencies.append(&mut interface.name.dependencies());

            if interface.kind == InterfaceKind::Default {
                for method in &interface.methods {
                    dependencies.append(&mut method.dependencies());
                }
            }
        }

        dependencies
    }

    pub fn to_tokens(&self) -> TokenStream {
        let definition = self.name.to_definition_tokens();
        let abi_definition = self.name.to_abi_definition_tokens();
        let name = &self.name.tokens;
        let phantoms = self.name.phantoms();
        let constraints = &self.name.constraints;

        let default_interface = self
            .interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
            .unwrap();

        let guid = self.name.to_guid_tokens(&default_interface.guid);

        let conversions = TokenStream::from_iter(
            self.interfaces
                .iter()
                .filter(|interface| interface.kind != InterfaceKind::Default)
                .map(|interface| interface.to_conversions_tokens(&name, &constraints)),
        );

        let methods = to_method_tokens(&self.interfaces);
        let abi_methods = default_interface.to_abi_method_tokens();
        let iterator = iterator_tokens(&self.name, &self.interfaces);
        let signature = self.name.to_signature_tokens(&self.signature);
        let (async_get, future) = get_async_tokens(&self.name, &self.interfaces);
        let debug = debug::debug_tokens(&self.name, &self.interfaces);

        quote! {
            #[repr(transparent)]
            #[derive(::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq)]
            pub struct #definition where #constraints {
                ptr: ::winrt::ComPtr<#name>,
                #phantoms
            }
            impl<#constraints> #name {
                #methods
                #async_get
            }
            unsafe impl<#constraints> ::winrt::ComInterface for #name {
                type VTable = #abi_definition;
                const IID: ::winrt::Guid = #guid;
            }
            #[repr(C)]
            pub struct #abi_definition where #constraints {
                base__: [ usize; 6],
                #abi_methods
                #phantoms
            }
            unsafe impl<#constraints> ::winrt::RuntimeType for #name {
                const SIGNATURE: ::winrt::ConstBuffer = { #signature };
            }
            unsafe impl<#constraints> ::winrt::AbiTransferable for #name {
                type Abi = ::winrt::RawComPtr<Self>;
                fn get_abi(&self) -> Self::Abi {
                    <::winrt::ComPtr<#name> as ::winrt::AbiTransferable>::get_abi(&self.ptr)
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    <::winrt::ComPtr<#name> as ::winrt::AbiTransferable>::set_abi(&mut self.ptr)
                }
            }
            impl<#constraints> ::std::convert::From<#name> for ::winrt::Object {
                fn from(value: #name) -> Self {
                    unsafe { ::std::mem::transmute(value) }
                }
            }
            impl<#constraints> ::std::convert::From<&#name> for ::winrt::Object {
                fn from(value: &#name) -> Self {
                    ::std::convert::From::from(::std::clone::Clone::clone(value))
                }
            }
            impl<'a, #constraints> ::std::convert::Into<::winrt::Param<'a, ::winrt::Object>> for #name {
                fn into(self) -> ::winrt::Param<'a, ::winrt::Object> {
                    ::winrt::Param::Owned(::std::convert::Into::<::winrt::Object>::into(self))
                }
            }
            impl<'a, #constraints> ::std::convert::Into<::winrt::Param<'a, ::winrt::Object>> for &'a #name {
                fn into(self) -> ::winrt::Param<'a, ::winrt::Object> {
                    ::winrt::Param::Owned(::std::convert::Into::<::winrt::Object>::into(::std::clone::Clone::clone(self)))
                }
            }
            #debug
            #conversions
            #iterator
            #future
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interface((namespace, type_name): (&str, &str)) -> Interface {
        let reader = &TypeReader::from_os();
        let t = reader.resolve_type_def((namespace, type_name));
        let t = Type::from_type_def(reader, t);

        match t {
            Type::Interface(t) => t,
            _ => panic!("Type not an interface"),
        }
    }

    #[test]
    fn test_stringable() {
        let t = interface(("Windows.Foundation", "IStringable"));
        assert!(t.name.runtime_name() == "Windows.Foundation.IStringable");
        assert!(t.interfaces.len() == 1);

        let t = t
            .interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
            .unwrap();

        assert!(t.name.runtime_name() == "Windows.Foundation.IStringable");
        assert!(t.kind == InterfaceKind::Default);

        assert!(t.methods.len() == 1);
        let method = &t.methods[0];
        assert!(method.name == "to_string");
        assert!(method.kind == MethodKind::Normal);

        assert!(method.params.is_empty());
        let param = method.return_type.as_ref().unwrap();
        assert!(param.kind == TypeKind::String);

        assert!(format!("{:#?}", &t.guid) == "96369f54-8eb6-48f0-abce-c1b211e627c3");
    }

    #[test]
    fn test_async_action() {
        let t = interface(("Windows.Foundation", "IAsyncAction"));
        assert!(t.name.runtime_name() == "Windows.Foundation.IAsyncAction");

        assert!(t.interfaces.len() == 2);

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IAsyncInfo")
            .unwrap();

        assert!(interface.kind == InterfaceKind::NonDefault);
        assert!(interface.name.runtime_name() == "Windows.Foundation.IAsyncInfo");

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IAsyncAction")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Default);
        assert!(interface.name.runtime_name() == "Windows.Foundation.IAsyncAction");
    }

    #[test]
    fn test_observable_map() {
        let t = interface(("Windows.Foundation.Collections", "IObservableMap`2"));
        assert!(t.name.runtime_name() == "Windows.Foundation.Collections.IObservableMap`2<K, V>");
        assert!(t.interfaces.len() == 3);

        let default_interface = t
            .interfaces
            .iter()
            .find(|required| required.name.name == "IObservableMap`2")
            .unwrap();

        assert!(default_interface.kind == InterfaceKind::Default);
        assert!(default_interface.methods.len() == 2);
        assert!(default_interface.methods[0].name == "map_changed");
        assert!(default_interface.methods[1].name == "remove_map_changed");

        let map = t
            .interfaces
            .iter()
            .find(|required| required.name.name == "IMap`2")
            .unwrap();

        assert!(map.kind == InterfaceKind::NonDefault);
        assert!(map.name.runtime_name() == "Windows.Foundation.Collections.IMap`2<K, V>");

        let iterable = t
            .interfaces
            .iter()
            .find(|required| required.name.name == "IIterable`1")
            .unwrap();

        assert!(iterable.kind == InterfaceKind::NonDefault);
        assert!(iterable.name.runtime_name() == "Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<K, V>>");
    }
}
