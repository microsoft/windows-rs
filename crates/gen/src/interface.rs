use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Interface {
    pub name: TypeName,
    pub guid: TypeGuid,
    pub interfaces: Vec<RequiredInterface>,
}

impl Interface {
    pub fn from_type_name(name: TypeName) -> Self {
        let guid = TypeGuid::from_type_def(&name.def);
        let mut interfaces = Vec::new();

        add_type(
            &mut interfaces,
            &name.def,
            &name.namespace,
            InterfaceKind::Default,
        );

        add_dependencies(&mut interfaces, &name, &name.namespace, true);

        rename_collisions(&mut interfaces);

        Self {
            name,
            guid,
            interfaces,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
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

    pub fn default_interface(&self) -> &RequiredInterface {
        self.interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
            .unwrap()
    }

    pub fn gen(&self) -> TokenStream {
        let definition = self.name.gen_definition();
        let abi_definition = self.name.gen_abi_definition();
        let name = self.name.gen();
        let phantoms = self.name.phantoms();
        let constraints = self.name.gen_constraint();

        let default_interface = self.default_interface();
        let guid = self.name.gen_guid(&self.guid);
        let signature = self.name.gen_signature(&format!("{{{:#?}}}", &self.guid));

        let conversions = self
            .interfaces
            .iter()
            .filter(|interface| interface.kind != InterfaceKind::Default)
            .map(|interface| interface.gen_conversions(&name, &constraints));

        let methods = gen_method(&self.interfaces);

        let abi_methods = default_interface.methods.iter().map(|method| {
            let signature = method.gen_abi();

            quote! {
                pub unsafe extern "system" fn #signature
            }
        });

        let iterator = gen_iterator(&self.name, &self.interfaces);
        let (async_get, future) = gen_async(&self.name, &self.interfaces);

        quote! {
            #[repr(transparent)]
            pub struct #definition(::windows::Object, #phantoms) where #constraints;
            impl<#constraints> ::std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone(), #phantoms)
                }
            }
            impl<#constraints> ::std::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }
            impl<#constraints> ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl<#constraints> ::std::cmp::Eq for #name {}
            impl<#constraints> #name {
                #methods
                #async_get
            }
            unsafe impl<#constraints> ::windows::Interface for #name {
                type Vtable = #abi_definition;
                const IID: ::windows::Guid = #guid;
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_definition(
                pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, count: *mut u32, values: *mut *mut ::windows::Guid) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut i32) -> ::windows::ErrorCode,
                #(#abi_methods,)*
                #phantoms
            ) where #constraints;
            unsafe impl<#constraints> ::windows::RuntimeType for #name {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = { #signature };
            }
            impl<#constraints> ::std::convert::From<#name> for ::windows::Object {
                fn from(value: #name) -> Self {
                    value.0
                }
            }
            impl<#constraints> ::std::convert::From<&#name> for ::windows::Object {
                fn from(value: &#name) -> Self {
                    ::std::convert::From::from(::std::clone::Clone::clone(value))
                }
            }

            impl<'a, #constraints> ::std::convert::Into<::windows::Param<'a, ::windows::Object>> for #name {
                fn into(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::Object>::into(self))
                }
            }
            impl<'a, #constraints> ::std::convert::Into<::windows::Param<'a, ::windows::Object>> for &'a #name {
                fn into(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::Object>::into(::std::clone::Clone::clone(self)))
                }
            }
            #(#conversions)*
            #iterator
            #future
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn interface((namespace, type_name): (&str, &str)) -> Interface {
        let reader = &winmd::TypeReader::get();
        let t = reader.expect_type_def((namespace, type_name));
        let t = TypeDefinition::from_type_def(&t);

        match t {
            TypeDefinition::Interface(t) => t,
            _ => panic!("TypeDefinition not an interface"),
        }
    }

    #[test]
    fn test_stringable() {
        let t = interface(("Windows.Foundation", "IStringable"));
        assert!(t.name.runtime_name() == "Windows.Foundation.IStringable");
        assert!(t.interfaces.len() == 1);
        assert!(format!("{:#?}", &t.guid) == "96369f54-8eb6-48f0-abce-c1b211e627c3");

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

        assert!(method.signature.params.is_empty());
        let param = method.signature.return_type.as_ref().unwrap();
        assert!(param.kind == TypeKind::String);
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
