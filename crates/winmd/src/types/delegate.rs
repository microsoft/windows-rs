use crate::tables::*;
use crate::types::*;
use crate::TypeReader;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Delegate {
    pub name: TypeName,
    pub method: Method,
    pub guid: TypeGuid,
}

impl Delegate {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let method = def
            .methods(reader)
            .find(|method| method.name(reader) == "Invoke")
            .unwrap();
        let method = Method::from_method_def(reader, method, &name.generics);
        let guid = TypeGuid::from_type_def(reader, def);
        Self { name, method, guid }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.method.dependencies()
    }

    pub fn to_tokens(&self) -> TokenStream {
        let definition = self.name.to_definition_tokens(&self.name.namespace);
        let abi_definition = self.name.to_abi_definition_tokens(&self.name.namespace);
        let name = self.name.to_tokens(&self.name.namespace);
        let phantoms = self.name.phantoms();
        let constraints = self.name.constraints();
        let abi_method = self.method.to_abi_tokens(&self.name, &self.name.namespace);
        let guid = self.guid.to_tokens();

        quote! {
            #[repr(transparent)]
            #[derive(Default)]
            pub struct #definition where #constraints {
                ptr: ::winrt::IUnknown,
                #phantoms
            }
            unsafe impl<#constraints> ::winrt::ComInterface for #name {
                type VTable = #abi_definition;
                const GUID: ::winrt::Guid = ::winrt::Guid::from_values(#guid);
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
                __base: [usize; 6],
                #abi_method
                #phantoms
            }
            unsafe impl<#constraints> ::winrt::RuntimeType for #name {
                type Abi = ::winrt::RawPtr;
                fn abi(&self) -> Self::Abi {
                     <::winrt::IUnknown as ::winrt::RuntimeType>::abi(&self.ptr) as Self::Abi
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    <::winrt::IUnknown as ::winrt::RuntimeType>::set_abi(&mut self.ptr) as _
                }
            }
        }
    }
}
