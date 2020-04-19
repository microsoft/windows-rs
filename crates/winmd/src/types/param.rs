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
                | TypeKind::Delegate(_) => {
                    let tokens = quote::format_ident!("__{}", position);
                    quote! { #name: #tokens, }
                }
                _ => quote! { #name: #tokens, },
            }
        } else {
            quote! { #name: &mut #tokens, }
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
}
