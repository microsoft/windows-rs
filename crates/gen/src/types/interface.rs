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

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        if !self.0.generics.is_empty() {
            return Vec::new();
        }

        self.0
            .def
            .methods()
            .map(|m| m.dependencies(&self.0.generics))
            .flatten()
            .chain(self.0.interfaces().filter_map(|i| i.0.definition()))
            .collect()
    }

    pub fn definition(&self) -> Option<tables::TypeDef> {
        if self.0.generics.is_empty() {
            Some(self.0.def)
        } else {
            None
        }
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let abi_name = self.0.gen_abi_name(gen);
        let phantoms = self.0.gen_phantoms();
        let constraints = self.0.gen_constraints();
        let guid = self.0.gen_guid();
        let abi_signatures = self
            .0
            .def
            .methods()
            .map(|m| m.signature(&self.0.generics).gen_winrt_abi(gen));

        let type_signature = self
            .0
            .gen_signature(&format!("{{{:#?}}}", &self.0.def.guid()));

        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name(::windows::IUnknown, #phantoms) where #constraints;
            impl<#constraints> #name {

            }
            unsafe impl<#constraints> ::windows::RuntimeType for #name {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = #type_signature;
            }
            unsafe impl<#constraints> ::windows::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::Guid = #guid;
            }
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
        let i: Vec<(Interface, parser::InterfaceKind)> = i.0.interfaces().collect();
        assert_eq!(i.len(), 1);

        assert_eq!(
            i[0].0 .0.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: IAsyncInfo"
        );
    }

    #[test]
    fn test_generic_interfaces() {
        let i = TypeReader::get_interface("Windows.Foundation.Collections", "IMap`2");
        let i: Vec<(Interface, parser::InterfaceKind)> = i.0.interfaces().collect();
        assert_eq!(i.len(), 1);

        assert_eq!(
            i[0].0.0.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IIterable :: < windows :: foundation :: collections :: IKeyValuePair :: < K , V > >"
        );
    }
}
