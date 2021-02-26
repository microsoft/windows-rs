use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Interface(pub GenericType);

impl Interface {
    pub fn type_signature(&self) -> String {
        let guid = Guid::from_type_def(&self.0.def);

        if self.0.generics.is_empty() {
            format!("{{{:#?}}}", guid)
        } else {
            let mut result = format!("pinterface({{{:#?}}}", guid);

            for generic in &self.0.generics {
                result.push(';');
                result.push_str(&generic.type_signature());
            }

            result.push(')');
            result
        }
    }

    pub fn interfaces(&self) -> Vec<InterfaceInfo> {
        fn add_interfaces(result: &mut Vec<InterfaceInfo>, parent: &GenericType, is_base: bool) {
            for child in parent.def.interface_impls() {
                if let Some(def) = child.generic_interface(&parent.generics) {
                    if !result.iter().any(|info| info.def == def) {
                        add_interfaces(result, &def, is_base);
                        let version = def.def.version();

                        result.push(InterfaceInfo {
                            def,
                            kind: InterfaceKind::NonDefault,
                            is_base: false,
                            version,
                        });
                    }
                }
            }
        }

        let mut result = Vec::new();

        result.push(InterfaceInfo {
            def: self.0.clone(),
            kind: InterfaceKind::Default,
            is_base: false,
            version: self.0.def.version(),
        });

        add_interfaces(&mut result, &self.0, false);
        InterfaceInfo::sort(&mut result);
        result
    }

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0
            .def
            .methods()
            .map(|m| m.dependencies(&self.0.generics))
            .flatten()
            .chain(self.0.interfaces().map(|i| i.def))
            .collect()
    }

    pub fn definition(&self) -> Vec<tables::TypeDef> {
        self.0.definition()
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let guid = self.0.gen_guid(gen);
        let abi_name = self.0.gen_abi_name(gen);
        let phantoms = self.0.gen_phantoms();
        let constraints = self.0.gen_constraints();

        let abi_signatures = self
            .0
            .def
            .methods()
            .map(|m| m.signature(&self.0.generics).gen_winrt_abi(gen));

        // The exclusive interface may be a factory interface and then we still need a type to use
        // with the factory cache. And we don't know at this stage whether the interface is for
        // the class or its factory.
        let public_type = if self.0.def.is_exclusive() {
            TokenStream::new()
        } else {
            let type_signature = self
                .0
                .gen_signature(&format!("{{{:#?}}}", &self.0.def.guid()));

            let interfaces = self.interfaces();
            let methods = InterfaceInfo::gen_methods(&interfaces, gen);
            let (async_get, future) = gen_async(&self.0, &interfaces, gen);
            let object = gen_object(&name, &constraints);
            let iterator = gen_iterator(&self.0, &interfaces, gen);

            let send_sync = if async_kind(&self.0) == AsyncKind::None {
                quote! {}
            } else {
                quote! {
                    unsafe impl<#constraints> ::std::marker::Send for #name {}
                    unsafe impl<#constraints> ::std::marker::Sync for #name {}
                }
            };

            let conversions = interfaces
                .iter()
                .filter(|interface| interface.kind != InterfaceKind::Default)
                .map(|interface| interface.gen_conversion(&name, &constraints, gen));

            quote! {
                impl<#constraints> #name {
                    #methods
                    #async_get
                }
                unsafe impl<#constraints> ::windows::RuntimeType for #name {
                    type DefaultType = ::std::option::Option<Self>;
                    const SIGNATURE: ::windows::ConstBuffer = #type_signature;
                }
                #future
                #object
                #(#conversions)*
                #send_sync
                #iterator
            }
        };

        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name(::windows::Object, #phantoms) where #constraints;
            unsafe impl<#constraints> ::windows::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::Guid = #guid;
            }
            #public_type
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, count: *mut u32, values: *mut *mut ::windows::Guid) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut i32) -> ::windows::ErrorCode,
                #(pub unsafe extern "system" fn #abi_signatures,)*
                #phantoms
            ) where #constraints;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        let i = TypeReader::get_interface("Windows.Foundation", "IStringable");
        assert_eq!(i.type_signature(), "{96369f54-8eb6-48f0-abce-c1b211e627c3}")
    }

    #[test]
    fn test_interfaces() {
        let i = TypeReader::get_interface("Windows.Foundation", "IAsyncOperation`1");
        let i = i.interfaces();
        assert_eq!(i.len(), 2);

        assert_eq!(
            i[0].def.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: IAsyncOperation :: < TResult >"
        );

        assert_eq!(
            i[1].def.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: IAsyncInfo"
        );
    }

    #[test]
    fn test_generic_interfaces() {
        let i = TypeReader::get_interface("Windows.Foundation.Collections", "IMap`2");
        let i = i.interfaces();
        assert_eq!(i.len(), 2);

        assert_eq!(
            i[0].def.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IMap :: < K , V >"
        );

        assert_eq!(
            i[1].def.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IIterable :: < windows :: foundation :: collections :: IKeyValuePair :: < K , V > >"
        );
    }
}
