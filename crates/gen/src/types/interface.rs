use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Interface(pub tables::TypeDef);

impl Interface {
    fn interfaces(&self) -> Vec<InterfaceInfo> {
        fn add_interfaces(
            result: &mut Vec<InterfaceInfo>,
            parent: &tables::TypeDef,
            is_base: bool,
        ) {
            for child in parent.interface_impls() {
                if let ElementType::TypeDef(def) = child.generic_interface(&parent.generics) {
                    if !result.iter().any(|info| info.def == def) {
                        add_interfaces(result, &def, is_base);
                        let version = def.version();

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

        let mut result = vec![InterfaceInfo {
            def: self.0.clone(),
            kind: InterfaceKind::Default,
            is_base: false,
            version: self.0.version(),
        }];

        add_interfaces(&mut result, &self.0, false);
        InterfaceInfo::sort(&mut result);
        result
    }

    pub fn gen(&self, gen: &Gen, include: TypeInclude) -> TokenStream {
        let name = self.0.gen_name(gen);
        let struct_phantoms = self.0.gen_phantoms();
        let constraints = self.0.gen_constraints();
        let type_signature = self.0.gen_signature(&format!("{{{:#?}}}", &self.0.guid()));
        let guid = self.0.gen_guid(gen);

        if include == TypeInclude::Full {
            let abi_name = self.0.gen_abi_name(gen);
            let abi_phantoms = self.0.gen_phantoms();

            let abi_signatures = self
                .0
                .methods()
                .map(|m| m.signature(&self.0.generics).gen_winrt_abi(gen));

            let is_exclusive = self.0.is_exclusive();

            let hidden = if is_exclusive {
                quote! { #[doc(hidden)] }
            } else {
                quote! {}
            };

            // The exclusive interface may be a factory interface and then we still need a type to use
            // with the factory cache. And we don't know at this stage whether the interface is for
            // the class or its factory.
            let public_type = if is_exclusive {
                TokenStream::new()
            } else {
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
                #hidden
                pub struct #name(::windows::IInspectable, #(#struct_phantoms,)*) where #constraints;
                unsafe impl<#constraints> ::windows::Interface for #name {
                    type Vtable = #abi_name;
                    const IID: ::windows::Guid = #guid;
                }
                #public_type
                #[repr(C)]
                #[doc(hidden)]
                pub struct #abi_name(
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, count: *mut u32, values: *mut *mut ::windows::Guid) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut ::windows::RawPtr) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut i32) -> ::windows::HRESULT,
                    #(pub unsafe extern "system" fn #abi_signatures,)*
                    #(pub #abi_phantoms,)*
                ) where #constraints;
            }
        } else {
            quote! {
                #[repr(transparent)]
                #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
                #[doc(hidden)]
                pub struct #name(::windows::IInspectable, #(#struct_phantoms,)*) where #constraints;
                unsafe impl<#constraints> ::windows::Interface for #name {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = #guid;
                }
                unsafe impl<#constraints> ::windows::RuntimeType for #name {
                    type DefaultType = ::std::option::Option<Self>;
                    const SIGNATURE: ::windows::ConstBuffer = #type_signature;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        let i =
            TypeReader::get().expect_type_def(TypeName::new("Windows.Foundation", "IStringable"));
        assert_eq!(i.type_signature(), "{96369f54-8eb6-48f0-abce-c1b211e627c3}")
    }

    #[test]
    fn test_interfaces() {
        let i = TypeReader::get()
            .expect_type_def(TypeName::new("Windows.Foundation", "IAsyncOperation`1"));
        let i = Interface(i.with_generics());
        let i = i.interfaces();
        assert_eq!(i.len(), 2);

        assert_eq!(
            i[0].def.gen_name(&Gen::Absolute).as_str(),
            "Windows :: Foundation :: IAsyncOperation :: < TResult >"
        );

        assert_eq!(
            i[1].def.gen_name(&Gen::Absolute).as_str(),
            "Windows :: Foundation :: IAsyncInfo"
        );
    }

    #[test]
    fn test_generic_interfaces() {
        let i = TypeReader::get()
            .expect_type_def(TypeName::new("Windows.Foundation.Collections", "IMap`2"));
        let i = Interface(i.with_generics());
        let i = i.interfaces();
        assert_eq!(i.len(), 2);

        assert_eq!(
            i[0].def.gen_name(&Gen::Absolute).as_str(),
            "Windows :: Foundation :: Collections :: IMap :: < K , V >"
        );

        assert_eq!(
            i[1].def.gen_name(&Gen::Absolute).as_str(),
            "Windows :: Foundation :: Collections :: IIterable :: < Windows :: Foundation :: Collections :: IKeyValuePair :: < K , V > >"
        );
    }
}
