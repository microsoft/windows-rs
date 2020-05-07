use crate::types::*;
use crate::*;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub kind: TypeKind,
    pub array: bool,
    pub input: bool,
    pub by_ref: bool,
}

impl Param {
    pub fn to_tokens(&self, calling_namespace: &str, position: usize) -> TokenStream {
        let name = format_ident(&self.name);
        let tokens = self.kind.to_tokens(calling_namespace);

        if self.array {
            if self.input {
                quote! { #name: &[#tokens], }
            } else if self.by_ref {
                quote! { #name: &mut ::winrt::Array<#tokens>, }
            } else {
                quote! { #name: &mut [#tokens], }
            }
        } else if self.input {
            match self.kind {
                TypeKind::String
                | TypeKind::Object
                | TypeKind::Guid
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Struct(_)
                | TypeKind::Delegate(_)
                | TypeKind::Generic(_) => {
                    let tokens = quote::format_ident!("__{}", position);
                    quote! { #name: #tokens, }
                }
                _ => quote! { #name: #tokens, },
            }
        } else {
            quote! { #name: &mut #tokens, }
        }
    }

    pub fn to_fn_tokens(&self, calling_namespace: &str) -> TokenStream {
        let tokens = self.kind.to_tokens(calling_namespace);

        if self.array {
            if self.input {
                quote! { &[#tokens], }
            } else if self.by_ref {
                quote! { &mut ::winrt::Array<#tokens>, }
            } else {
                quote! { &mut [#tokens], }
            }
        } else if self.input {
            match self.kind {
                TypeKind::String
                | TypeKind::Object
                | TypeKind::Guid
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Struct(_)
                | TypeKind::Delegate(_)
                | TypeKind::Generic(_) => {
                    quote! { &#tokens, }
                }
                _ => quote! { #tokens, },
            }
        } else {
            quote! { &mut #tokens, }
        }
    }

    pub fn to_return_tokens(&self, calling_namespace: &str) -> TokenStream {
        let tokens = self.kind.to_tokens(calling_namespace);

        if self.array {
            quote! { ::winrt::Array<#tokens> }
        } else {
            quote! { #tokens }
        }
    }

    pub fn to_abi_tokens(&self, calling_namespace: &str) -> TokenStream {
        let tokens = self.kind.to_abi_tokens(calling_namespace);

        if self.array {
            if self.input {
                quote! { u32, *const #tokens }
            } else if self.by_ref {
                quote! { *mut u32, *mut *mut #tokens }
            } else {
                quote! { u32, *mut #tokens }
            }
        } else if self.input {
            tokens
        } else {
            quote! { *mut #tokens }
        }
    }

    pub fn to_abi_return_arg_tokens(&self, calling_namespace: &str) -> TokenStream {
        let return_type = self.kind.to_tokens(calling_namespace);

        if self.array {
            quote! { ::winrt::Array::<#return_type>::set_abi_len(&mut __ok), winrt::Array::<#return_type>::set_abi(&mut __ok), }
        } else {
            quote! { <#return_type as ::winrt::RuntimeType>::set_abi(&mut __ok) }
        }
    }

    pub fn to_abi_arg_tokens(&self) -> TokenStream {
        let name = format_ident(&self.name);

        if self.array {
            if self.input {
                quote! { #name.len() as u32, ::std::mem::transmute(#name.as_ptr()), }
            } else if self.by_ref {
                quote! { #name.set_abi_len(), #name.set_abi(), }
            } else {
                quote! { #name.len() as u32, ::std::mem::transmute_copy(&#name), }
            }
        } else if self.input {
            if self.kind.blittable() {
                quote! { #name, }
            } else {
                match self.kind {
                    TypeKind::String
                    | TypeKind::Object
                    | TypeKind::Guid
                    | TypeKind::Class(_)
                    | TypeKind::Interface(_)
                    | TypeKind::Struct(_)
                    | TypeKind::Delegate(_)
                    | TypeKind::Generic(_) => quote! { #name.into().abi(), },
                    TypeKind::Enum(_) => quote! { ::winrt::RuntimeType::abi(&#name), },
                    _ => quote! { ::winrt::RuntimeType::abi(#name), },
                }
            }
        } else if self.kind.blittable() {
            quote! { #name, }
        } else {
            quote! { ::winrt::RuntimeType::set_abi(#name), }
        }
    }

    pub fn to_invoke_arg_tokens(&self) -> TokenStream {
        let name = format_ident(&self.name);

        if self.array {
            // TODO: delegate with array parameters are challenging to say the least.
            // I'll get to them shortly.
            panic!("array");
        } else if self.input {
            match self.kind {
                TypeKind::Enum(_) => quote! { *::winrt::RuntimeType::from_abi(&#name) },
                _ => quote! { ::winrt::RuntimeType::from_abi(&#name) },
            }
        } else if self.kind.blittable() {
            quote! { #name, }
        } else {
            quote! { ::winrt::RuntimeType::from_mut_abi(#name) }
        }
    }
}
