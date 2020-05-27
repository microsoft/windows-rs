use super::object::to_object_tokens;
use crate::tables::*;
use crate::types::debug;
use crate::types::*;
use crate::*;
use proc_macro2::TokenStream;
use quote::*;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Interface {
    pub name: TypeName,
    pub interfaces: Vec<RequiredInterface>,
    pub signature: String,
}

impl Interface {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let mut interfaces = Vec::new();

        // Ensures that the default interface is first in line.
        let mut default_interface = RequiredInterface::from_type_def(reader, def);
        default_interface.kind = InterfaceKind::Default;
        interfaces.push(default_interface);

        RequiredInterface::append_required(reader, &name, &mut interfaces);
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
        let definition = self.name.to_definition_tokens(&self.name.namespace);
        let abi_definition = self.name.to_abi_definition_tokens(&self.name.namespace);
        let name = &*self.name.to_tokens(&self.name.namespace);
        let phantoms = self.name.phantoms();
        let constraints = &*self.name.constraints();
        let default_interface = &self.interfaces[0];
        debug_assert!(default_interface.kind == InterfaceKind::Default);
        let guid = self.name.to_guid_tokens(&default_interface.guid);
        let conversions = TokenStream::from_iter(self.interfaces.iter().skip(1).map(|interface| {
            interface.to_conversions_tokens(&self.name.namespace, &name, &constraints)
        }));

        let object = to_object_tokens(&name, &constraints);
        let methods = to_method_tokens(&self.name.namespace, &self.interfaces);
        let abi_methods = default_interface.to_abi_method_tokens(&default_interface.name.namespace);
        let iterator = iterator_tokens(&self.name, &self.interfaces);
        let signature = self.name.to_signature_tokens(&self.signature);
        let async_get = async_get_tokens(&self.name, &self.interfaces);
        let debug = debug::debug_tokens(&self.name, &self.interfaces);

        quote! {
            #[repr(transparent)]
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
                fn iid() -> ::winrt::Guid {
                    #guid
                }
            }
            impl<#constraints> ::std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self {
                        ptr: self.ptr.clone(),
                        #phantoms
                    }
                }
            }
            #[repr(C)]
            pub struct #abi_definition where #constraints {
                pub unknown_query_interface: extern "system" fn(::winrt::RawComPtr<::winrt::IUnknown>, &::winrt::Guid, *mut ::winrt::RawPtr) -> ::winrt::ErrorCode,
                pub unknown_add_ref: extern "system" fn(::winrt::RawComPtr<::winrt::IUnknown>) -> u32,
                pub unknown_release: extern "system" fn(::winrt::RawComPtr<::winrt::IUnknown>) -> u32,
                pub inspectable_iids: extern "system" fn(::winrt::RawComPtr<::winrt::Object>, *mut u32, *mut *mut ::winrt::Guid) -> ::winrt::ErrorCode,
                pub inspectable_type_name: extern "system" fn(::winrt::RawComPtr<::winrt::Object>, *mut <::winrt::HString as ::winrt::RuntimeType>::Abi) -> ::winrt::ErrorCode,
                pub inspectable_trust_level: extern "system" fn(::winrt::RawComPtr<::winrt::Object>, *mut i32) -> ::winrt::ErrorCode,
                #abi_methods
                #phantoms
            }
            unsafe impl<#constraints> ::winrt::RuntimeType for #name {
                type Abi = ::winrt::RawComPtr<Self>;
                fn signature() -> String {
                    #signature
                }
                fn abi(&self) -> Self::Abi {
                    <::winrt::ComPtr<Self> as ::winrt::ComInterface>::as_raw(&self.ptr)
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    self.ptr.set_abi()
                }
            }
            #debug
            impl<#constraints> ::std::default::Default for #name {
                fn default() -> Self {
                    Self {
                        ptr: ::winrt::ComPtr::default(),
                        #phantoms
                    }
                 }
            }
            impl<#constraints> ::std::cmp::PartialEq<Self> for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.ptr == other.ptr
                }
            }
            #conversions
            #object
            #iterator
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
        assert!(t.name.runtime_name() == "Windows.Foundation.IStringable");
        assert!(t.interfaces.len() == 1);
        let t = &t.interfaces[0];
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
