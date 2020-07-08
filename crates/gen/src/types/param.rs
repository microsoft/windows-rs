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
    pub fn to_tokens(&self, position: usize) -> TokenStream {
        let name = format_ident(&self.name);
        let tokens = self.kind.to_tokens();

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
                | TypeKind::TimeSpan
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Struct(_)
                | TypeKind::Delegate(_)
                | TypeKind::Generic(_) => {
                    let tokens = quote::format_ident!("T{}__", position);
                    quote! { #name: #tokens, }
                }
                _ => quote! { #name: #tokens, },
            }
        } else {
            quote! { #name: &mut #tokens, }
        }
    }

    pub fn to_fn_tokens(&self) -> TokenStream {
        let tokens = self.kind.to_tokens();

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
                | TypeKind::TimeSpan
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

    pub fn to_return_tokens(&self) -> TokenStream {
        let tokens = self.kind.to_tokens();

        if self.array {
            quote! { ::winrt::Array<#tokens> }
        } else {
            quote! { #tokens }
        }
    }

    pub fn to_abi_tokens(&self) -> TokenStream {
        let name = format_ident(&self.name);
        let tokens = self.kind.to_abi_tokens();

        if self.array {
            let name_size = quote::format_ident!("array_size_{}", &self.name);

            if self.input {
                quote! { #name_size: u32, #name: *const #tokens }
            } else if self.by_ref {
                quote! { #name_size: *mut u32, #name: *mut *mut #tokens }
            } else {
                quote! { #name_size: u32, #name: *mut #tokens }
            }
        } else if self.input {
            quote! { #name: #tokens }
        } else {
            quote! { #name: *mut #tokens }
        }
    }

    pub fn to_abi_return_arg_tokens(&self) -> TokenStream {
        let return_type = self.kind.to_tokens();

        if self.array {
            quote! { ::winrt::Array::<#return_type>::set_abi_len(&mut result__), winrt::Array::<#return_type>::set_abi(&mut result__), }
        } else {
            quote! { <#return_type as ::winrt::AbiTransferable>::set_abi(&mut result__) }
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
            if self.kind.primitive() {
                quote! { #name, }
            } else {
                match self.kind {
                    TypeKind::String
                    | TypeKind::Object
                    | TypeKind::Guid
                    | TypeKind::TimeSpan
                    | TypeKind::Class(_)
                    | TypeKind::Interface(_)
                    | TypeKind::Struct(_)
                    | TypeKind::Delegate(_)
                    | TypeKind::Generic(_) => quote! { #name.into().get_abi(), },
                    TypeKind::Enum(_) => quote! { ::winrt::AbiTransferable::get_abi(&#name), },
                    _ => quote! { ::winrt::AbiTransferable::get_abi(#name), },
                }
            }
        } else if self.kind.primitive() {
            quote! { #name, }
        } else {
            quote! { ::winrt::AbiTransferable::set_abi(#name), }
        }
    }

    pub fn to_invoke_arg_tokens(&self) -> TokenStream {
        let name = format_ident(&self.name);

        if self.array {
            let kind = self.kind.to_tokens();
            let name_size = quote::format_ident!("array_size_{}", name);
            if self.input {
                quote! { <#kind as ::winrt::AbiTransferable>::slice_from_abi(#name, #name_size as usize) }
            } else if self.by_ref {
                // TODO: need to take resulting array and detach back onto the ABI
                quote! { &mut ::winrt::Array::new() }
            } else {
                quote! { <#kind as ::winrt::AbiTransferable>::slice_from_mut_abi(#name, #name_size as usize) }
            }
        } else if self.input {
            if self.kind.primitive() {
                quote! { #name }
            } else if let TypeKind::Enum(_) = self.kind {
                quote! { *::winrt::AbiTransferable::from_abi(&#name) }
            } else {
                quote! { ::winrt::AbiTransferable::from_abi(&#name) }
            }
        } else {
            quote! { ::winrt::AbiTransferable::from_mut_abi(&mut *#name) }
        }
    }
}
