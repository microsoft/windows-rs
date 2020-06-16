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
    pub fn from_type_name(reader: &TypeReader, name: TypeName) -> Self {
        let mut interfaces = Vec::new();

        // Ensures that the default interface is first in line.
        let mut default_interface =
            RequiredInterface::from_type_def(reader, name.def, &name.namespace);
        default_interface.kind = InterfaceKind::Default;
        interfaces.push(default_interface);

        RequiredInterface::append_required(reader, &name, &name.namespace, &mut interfaces);
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
        let default_interface = &self.interfaces[0];
        debug_assert!(default_interface.kind == InterfaceKind::Default);
        let guid = self.name.to_guid_tokens(&default_interface.guid);
        let conversions = TokenStream::from_iter(
            self.interfaces
                .iter()
                .skip(1)
                .map(|interface| interface.to_conversions_tokens(&name, &constraints)),
        );

        let object = to_object_tokens(&name, &constraints);
        let methods = to_method_tokens(&self.interfaces);
        let abi_methods = default_interface.to_abi_method_tokens();
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
                __base: [ usize; 6],
                #abi_methods
                #phantoms
            }
            unsafe impl<#constraints> ::winrt::RuntimeType for #name {
                fn signature() -> String {
                    #signature
                }
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
