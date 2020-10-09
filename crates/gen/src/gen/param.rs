use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub kind: gen::TypeKind,
    pub array: bool,
    pub input: bool,
    pub by_ref: bool,
}

impl Param {
    pub fn gen(&self, position: usize) -> TokenStream {
        let name = format_ident(&self.name);
        let tokens = self.kind.gen();

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
                gen::TypeKind::String
                | gen::TypeKind::Object
                | gen::TypeKind::Guid
                | gen::TypeKind::Class(_)
                | gen::TypeKind::Interface(_)
                | gen::TypeKind::Struct(_)
                | gen::TypeKind::Delegate(_)
                | gen::TypeKind::Generic(_) => {
                    let tokens = squote::format_ident!("T{}__", position);
                    quote! { #name: #tokens, }
                }
                _ => quote! { #name: #tokens, },
            }
        } else {
            quote! { #name: &mut #tokens, }
        }
    }

    pub fn gen_fn(&self) -> TokenStream {
        let tokens = self.kind.gen();

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
                gen::TypeKind::String
                | gen::TypeKind::Object
                | gen::TypeKind::Guid
                | gen::TypeKind::Class(_)
                | gen::TypeKind::Interface(_)
                | gen::TypeKind::Struct(_)
                | gen::TypeKind::Delegate(_)
                | gen::TypeKind::Generic(_) => {
                    quote! { &#tokens, }
                }
                _ => quote! { #tokens, },
            }
        } else {
            quote! { &mut #tokens, }
        }
    }

    pub fn gen_return(&self) -> TokenStream {
        let tokens = self.kind.gen();

        if self.array {
            quote! { ::winrt::Array<#tokens> }
        } else {
            quote! { #tokens }
        }
    }

    pub fn gen_abi(&self) -> TokenStream {
        let name = format_ident(&self.name);
        let tokens = self.kind.gen_abi();

        if self.array {
            let name_size = squote::format_ident!("array_size_{}", &self.name);

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

    pub fn gen_abi_return_arg(&self) -> TokenStream {
        let return_type = self.kind.gen();

        if self.array {
            quote! { ::winrt::Array::<#return_type>::set_abi_len(&mut result__), winrt::Array::<#return_type>::set_abi(&mut result__), }
        } else {
            quote! { <#return_type as ::winrt::AbiTransferable>::set_abi(&mut result__) }
        }
    }

    pub fn gen_abi_arg(&self) -> TokenStream {
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
                    gen::TypeKind::String
                    | gen::TypeKind::Object
                    | gen::TypeKind::Guid
                    | gen::TypeKind::Class(_)
                    | gen::TypeKind::Interface(_)
                    | gen::TypeKind::Struct(_)
                    | gen::TypeKind::Delegate(_)
                    | gen::TypeKind::Generic(_) => quote! { #name.into().get_abi(), },
                    gen::TypeKind::Enum(_) => quote! { ::winrt::AbiTransferable::get_abi(&#name), },
                    _ => quote! { ::winrt::AbiTransferable::get_abi(#name), },
                }
            }
        } else if self.kind.primitive() {
            quote! { #name, }
        } else {
            quote! { ::winrt::AbiTransferable::set_abi(#name), }
        }
    }

    pub fn gen_invoke_arg(&self) -> TokenStream {
        let name = format_ident(&self.name);

        if self.array {
            let kind = self.kind.gen();
            let name_size = squote::format_ident!("array_size_{}", name);
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
            } else if let gen::TypeKind::Enum(_) = self.kind {
                quote! { *::winrt::AbiTransferable::from_abi(&#name) }
            } else {
                quote! { ::winrt::AbiTransferable::from_abi(&#name) }
            }
        } else {
            quote! { ::winrt::AbiTransferable::from_mut_abi(&mut *#name) }
        }
    }
}
