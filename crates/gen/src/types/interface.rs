use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Interface(pub GenericType);

impl Interface {
    pub fn signature(&self) -> String {
        let guid = Guid::from_type_def(&self.0.def);

        if self.0.generics.is_empty() {
            format!("{{{:#?}}}", guid)
        } else {
            let mut result = format!("pinterface({{{:#?}}}", guid);

            for generic in &self.0.generics {
                result.push(';');
                result.push_str(&generic.signature());
            }

            result.push(')');
            result
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.0
            .def
            .methods()
            .map(|m| m.dependencies(&self.0.generics))
            .chain(self.0.interfaces().map(|i| i.dependencies()))
            .flatten()
            .collect()
    }

    pub fn gen(&self, _: Gen) -> TokenStream {
        quote! {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get(namespace: &str, name: &str) -> Interface {
        if let ElementType::Interface(value) = TypeReader::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }

    #[test]
    fn test_bool() {
        let i = get("Windows.Foundation", "IStringable");
        assert_eq!(i.signature(), "{96369f54-8eb6-48f0-abce-c1b211e627c3}")
    }

    #[test]
    fn test_interfaces() {
        let i = get("Windows.Foundation", "IAsyncOperation`1");
        let i: Vec<Interface> = i.0.interfaces().collect();
        assert_eq!(i.len(), 1);

        assert_eq!(
            i[0].0.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: IAsyncInfo"
        );
    }

    #[test]
    fn test_generic_interfaces() {
        let i = get("Windows.Foundation.Collections", "IMap`2");
        let i: Vec<Interface> = i.0.interfaces().collect();
        assert_eq!(i.len(), 1);

        assert_eq!(
            i[0].0.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IIterable :: < windows :: foundation :: collections :: IKeyValuePair :: < K , V > >"
        );
    }
}

// #[derive(Debug)]
// pub struct Interface {
//     pub name: TypeName,
//     pub guid: Guid,
//     pub interfaces: Vec<RequiredInterface>,
// }

// impl Interface {
//     pub fn from_type_name(name: TypeName) -> Self {
//         let guid = name.def.guid();
//         let mut interfaces = Vec::new();

//         add_type(
//             &mut interfaces,
//             &name.def,
//             &name.namespace,
//             InterfaceKind::Default,
//         );

//         add_dependencies(&mut interfaces, &name, &name.namespace, true);

//         rename_collisions(&mut interfaces);

//         Self {
//             name,
//             guid,
//             interfaces,
//         }
//     }

//     pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
//         let mut dependencies = Vec::new();

//         for interface in &self.interfaces {
//             dependencies.append(&mut interface.name.dependencies());

//             if interface.kind == InterfaceKind::Default {
//                 for method in &interface.methods {
//                     dependencies.append(&mut method.dependencies());
//                 }
//             }
//         }

//         dependencies
//     }

//     pub fn default_interface(&self) -> &RequiredInterface {
//         self.interfaces
//             .iter()
//             .find(|i| i.kind == InterfaceKind::Default)
//             .unwrap()
//     }

//     pub fn gen(&self) -> TokenStream {
//         let definition = self.name.gen_definition();
//         let abi_definition = self.name.gen_abi_definition();
//         let name = self.name.gen();
//         let phantoms = self.name.phantoms();
//         let constraints = self.name.gen_constraint();

//         let default_interface = self.default_interface();
//         let guid = self.name.gen_guid(&self.guid);
//         let signature = self.name.gen_signature(&format!("{{{:#?}}}", &self.guid));

//         let conversions = self
//             .interfaces
//             .iter()
//             .filter(|interface| interface.kind != InterfaceKind::Default)
//             .map(|interface| interface.gen_conversions(&name, &constraints));

//         let methods = gen_method(&self.interfaces);

//         let abi_methods = default_interface.methods.iter().map(|method| {
//             let signature = method.gen_abi();

//             quote! {
//                 pub unsafe extern "system" fn #signature
//             }
//         });

//         let iterator = gen_iterator(&self.name, &self.interfaces);
//         let (async_get, future) = gen_async(&self.name, &self.interfaces);

//         quote! {
//             #[repr(transparent)]
//             pub struct #definition(::windows::Object, #phantoms) where #constraints;
//             impl<#constraints> ::std::clone::Clone for #name {
//                 fn clone(&self) -> Self {
//                     Self(self.0.clone(), #phantoms)
//                 }
//             }
//             impl<#constraints> ::std::fmt::Debug for #name {
//                 fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//                     write!(f, "{:?}", self.0)
//                 }
//             }
//             impl<#constraints> ::std::cmp::PartialEq for #name {
//                 fn eq(&self, other: &Self) -> bool {
//                     self.0 == other.0
//                 }
//             }
//             impl<#constraints> ::std::cmp::Eq for #name {}
//             impl<#constraints> #name {
//                 #methods
//                 #async_get
//             }
//             unsafe impl<#constraints> ::windows::Interface for #name {
//                 type Vtable = #abi_definition;
//                 const IID: ::windows::Guid = #guid;
//             }
//             #[repr(C)]
//             #[doc(hidden)]
//             pub struct #abi_definition(
//                 pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
//                 pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
//                 pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
//                 pub unsafe extern "system" fn(this: ::windows::RawPtr, count: *mut u32, values: *mut *mut ::windows::Guid) -> ::windows::ErrorCode,
//                 pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
//                 pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut i32) -> ::windows::ErrorCode,
//                 #(#abi_methods,)*
//                 #phantoms
//             ) where #constraints;
//             unsafe impl<#constraints> ::windows::RuntimeType for #name {
//                 type DefaultType = ::std::option::Option<Self>;
//                 const SIGNATURE: ::windows::ConstBuffer = { #signature };
//             }
//             impl<#constraints> ::std::convert::From<#name> for ::windows::Object {
//                 fn from(value: #name) -> Self {
//                     value.0
//                 }
//             }
//             impl<#constraints> ::std::convert::From<&#name> for ::windows::Object {
//                 fn from(value: &#name) -> Self {
//                     ::std::convert::From::from(::std::clone::Clone::clone(value))
//                 }
//             }

//             impl<'a, #constraints> ::std::convert::Into<::windows::Param<'a, ::windows::Object>> for #name {
//                 fn into(self) -> ::windows::Param<'a, ::windows::Object> {
//                     ::windows::Param::Owned(::std::convert::Into::<::windows::Object>::into(self))
//                 }
//             }
//             impl<'a, #constraints> ::std::convert::Into<::windows::Param<'a, ::windows::Object>> for &'a #name {
//                 fn into(self) -> ::windows::Param<'a, ::windows::Object> {
//                     ::windows::Param::Owned(::std::convert::Into::<::windows::Object>::into(::std::clone::Clone::clone(self)))
//                 }
//             }
//             #(#conversions)*
//             #iterator
//             #future
//         }
//     }
// }
