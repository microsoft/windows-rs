use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Delegate(pub GenericType);

impl Delegate {
    pub fn type_signature(&self) -> String {
        if self.0.generics.is_empty() {
            format!("delegate({})", self.0.interface_signature())
        } else {
            self.0.interface_signature()
        }
    }

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        if !self.0.generics.is_empty() {
            return Vec::new();
        }

        self.method().dependencies(&[])
    }

    pub fn definition(&self) -> Option<tables::TypeDef> {
        if self.0.generics.is_empty() {
            Some(self.0.def)
        } else {
            None
        }
    }

    fn method(&self) -> tables::MethodDef {
        self.0
            .def
            .methods()
            .find(|m| m.name() == "Invoke")
            .expect("Callback")
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let abi_name = self.0.gen_abi_name(gen);
        let signature = self.method().signature(&self.0.generics);
        let abi_signature = signature.gen_winrt_abi(gen);
        let fn_constraint = signature.gen_constraint(gen);
        let guid = self.0.gen_guid();
        let phantoms = self.0.gen_phantoms();
        let constraints = self.0.gen_constraints();

        let method = MethodInfo {
            name: "invoke",
            vtable_offset: 3,
            overload: 0,
        };

        let interface = InterfaceInfo {
            def: self.0.clone(),
            kind: InterfaceKind::Default,
            is_base: false,
            version: (0,0)
        };

        let method = signature.gen_winrt_method(&method, &interface, gen);

        let type_signature = if self.0.generics.is_empty() {
            self.0
                .gen_signature(&format!("delegate({{{:#?}}})", &self.0.def.guid()))
        } else {
            self.0
                .gen_signature(&format!("{{{:#?}}}", &self.0.def.guid()))
        };

        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name(::windows::IUnknown, #phantoms) where #constraints;
            impl<#constraints> #name {
                #method
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
                pub unsafe extern "system" fn #abi_signature,
                #phantoms
            ) where #constraints;
        }
    }
}
