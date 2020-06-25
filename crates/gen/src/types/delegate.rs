use crate::tables::*;
use crate::types::debug;
use crate::types::*;
use crate::TypeReader;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Delegate {
    pub name: TypeName,
    pub method: Method,
    pub guid: TypeGuid,
    pub signature: String,
}

impl Delegate {
    pub fn from_type_name(reader: &TypeReader, name: TypeName) -> Self {
        let method = name
            .def
            .methods(reader)
            .find(|method| method.name(reader) == "Invoke")
            .unwrap();
        let method = Method::from_method_def(reader, method, &name.generics, &name.namespace);
        let guid = TypeGuid::from_type_def(reader, name.def);
        let signature = name.base_delegate_signature(reader);
        Self {
            name,
            method,
            guid,
            signature,
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.method.dependencies()
    }

    pub fn to_tokens(&self) -> TokenStream {
        let definition = self.name.to_definition_tokens();
        let abi_definition = self.name.to_abi_definition_tokens();
        let fn_constraint = self.to_fn_constraint_tokens();
        let impl_definition = self.to_impl_definition_tokens(&fn_constraint);
        let name = &self.name.tokens;
        let abi_name = self.name.to_abi_tokens();
        let impl_name = self.to_impl_name_tokens();
        let phantoms = self.name.phantoms();
        let constraints = &self.name.constraints;
        let method = self.method.to_default_tokens();
        let abi_method = self.method.to_abi_tokens(&self.name);
        let guid = self.name.to_guid_tokens(&self.guid);
        let signature = self.name.to_signature_tokens(&self.signature);
        let invoke_sig = self.method.to_abi_impl_tokens(&self.name);
        let invoke_args = self
            .method
            .params
            .iter()
            .map(|param| param.to_invoke_arg_tokens());
        let debug = debug::default_debug_tokens(&self.name);

        let invoke_upcall = if let Some(return_type) = &self.method.return_type {
            if return_type.array {
                quote! {
                    match ((*this).invoke)(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(_) => {
                            ::winrt::ErrorCode(0)
                        }
                        ::std::result::Result::Err(result__) => result__.into()
                    }
                }
            } else {
                quote! {
                    match ((*this).invoke)(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(_) => {
                            ::winrt::ErrorCode(0)
                        }
                        ::std::result::Result::Err(result__) => result__.into()
                    }
                }
            }
        } else {
            quote! {
                ((*this).invoke)(#(#invoke_args,)*).into()
            }
        };

        quote! {
            #[repr(transparent)]
            pub struct #definition where #constraints {
                ptr: ::winrt::ComPtr<#name>,
                #phantoms
            }
            impl<#constraints> #name {
                #method
                pub fn new<#fn_constraint>(invoke: F) -> Self {
                    #impl_name::new(invoke)
                }
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
                pub unknown_query_interface: extern "system" fn(::winrt::NonNullRawComPtr<::winrt::IUnknown>, &::winrt::Guid, *mut ::winrt::RawPtr) -> ::winrt::ErrorCode,
                pub unknown_add_ref: extern "system" fn(::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32,
                pub unknown_release: extern "system" fn(::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32,
                #abi_method
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
            #[repr(C)]
            struct #impl_definition where #constraints {
                vtable: *const #abi_definition,
                count: ::winrt::RefCount,
                invoke: F,
            }
            impl<#constraints #fn_constraint> #impl_name {
                const VTABLE: #abi_definition = #abi_name {
                    unknown_query_interface: #impl_name::unknown_query_interface,
                    unknown_add_ref: #impl_name::unknown_add_ref,
                    unknown_release: #impl_name::unknown_release,
                    invoke: #impl_name::invoke,
                    #phantoms
                };
                pub fn new(invoke: F) -> #name {
                    let value = Self {
                        vtable: &Self::VTABLE,
                        count: ::winrt::RefCount::new(),
                        invoke,
                    };
                    unsafe {
                        let mut result: #name = std::mem::zeroed();
                        let ptr: ::std::ptr::NonNull<Self> = ::std::ptr::NonNull::new_unchecked(::std::boxed::Box::into_raw(::std::boxed::Box::new(value)));
                        *<#name as ::winrt::AbiTransferable>::set_abi(&mut result) = Some(::winrt::NonNullRawComPtr::new(ptr.cast()));
                        result
                    }
                }
                extern "system" fn unknown_query_interface(
                    this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>,
                    iid: &::winrt::Guid,
                    interface: *mut ::winrt::RawPtr,
                ) -> ::winrt::ErrorCode {
                    unsafe {
                        let this: *mut Self = this.as_raw() as _;

                        if iid == &<#name as ::winrt::ComInterface>::iid()
                            || iid == &<::winrt::IUnknown as ::winrt::ComInterface>::iid()
                            || iid == &<::winrt::IAgileObject as ::winrt::ComInterface>::iid()
                        {
                            *interface = this as ::winrt::RawPtr;
                            (*this).count.add_ref();
                            return ::winrt::ErrorCode(0);
                        }

                        *interface = std::ptr::null_mut();
                        ::winrt::ErrorCode(0x80004002)
                    }
                }
                extern "system" fn unknown_add_ref(this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32 {
                    unsafe {
                        let this: *mut Self = this.as_raw() as _;
                        (*this).count.add_ref()
                    }
                }
                extern "system" fn unknown_release(this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32 {
                    unsafe {
                        let this: *mut Self = this.as_raw() as _;
                        let remaining = (*this).count.release();

                        if remaining == 0 {
                            Box::from_raw(this);
                        }

                        remaining
                    }
                }
                #invoke_sig {
                    let this: *mut Self = this.as_raw() as _;
                    #invoke_upcall
                }
            }
        }
    }

    fn to_fn_constraint_tokens(&self) -> TokenStream {
        let params = self.method.params.iter().map(|param| param.to_fn_tokens());

        let return_type = if let Some(return_type) = &self.method.return_type {
            return_type.to_return_tokens()
        } else {
            quote! { () }
        };

        quote! { F: FnMut(#(#params)*) -> ::winrt::Result<#return_type> + 'static }
    }

    fn to_impl_definition_tokens(&self, fn_constraint: &TokenStream) -> TokenStream {
        if self.name.generics.is_empty() {
            let name = format_impl_ident(&self.name.name);
            quote! { #name<#fn_constraint> }
        } else {
            let name = format_impl_ident(&self.name.name[..self.name.name.len() - 2]);
            let generics = self.name.generics.iter().map(|g| g.to_tokens());
            quote! { #name<#(#generics,)* #fn_constraint> }
        }
    }

    fn to_impl_name_tokens(&self) -> TokenStream {
        if self.name.generics.is_empty() {
            let name = format_impl_ident(&self.name.name);
            quote! { #name::<F> }
        } else {
            let name = format_impl_ident(&self.name.name[..self.name.name.len() - 2]);
            let generics = self.name.generics.iter().map(|g| g.to_tokens());
            quote! { #name::<#(#generics,)* F> }
        }
    }
}

fn format_impl_ident(name: &str) -> proc_macro2::Ident {
    quote::format_ident!("impl_{}", name)
}
