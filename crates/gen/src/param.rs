use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub kind: TypeKind,
    pub array: bool,
    pub input: bool,
    pub by_ref: bool,
    pub is_const: bool,
}

impl Param {
    pub fn gen(&self, position: usize) -> TokenStream {
        let name = format_ident(&self.name);
        let tokens = self.kind.gen();

        if self.array {
            if self.input {
                quote! { #name: &[<#tokens as ::winrt::RuntimeType>::DefaultType], }
            } else if self.by_ref {
                quote! { #name: &mut ::winrt::Array<#tokens>, }
            } else {
                quote! { #name: &mut [<#tokens as ::winrt::RuntimeType>::DefaultType], }
            }
        } else if self.input {
            match &self.kind {
                TypeKind::String
                | TypeKind::Object
                | TypeKind::Guid
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Struct(_)
                | TypeKind::Delegate(_)
                | TypeKind::Generic(_) => {
                    let tokens = squote::format_ident!("T{}__", position);
                    quote! { #name: #tokens, }
                }
                _ => quote! { #name: #tokens, },
            }
        } else {
            match self.kind {
                TypeKind::Object
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Delegate(_) => {
                    quote! { #name: &mut ::std::option::Option<#tokens>, }
                }
                TypeKind::Generic(_) => {
                    quote! { &mut <#tokens as ::winrt::RuntimeType>::DefaultType, }
                }
                _ => quote! { #name: &mut #tokens, },
            }
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
                TypeKind::String | TypeKind::Guid | TypeKind::Struct(_) => {
                    quote! { &#tokens, }
                }
                TypeKind::Generic(_) => {
                    quote! { &<#tokens as ::winrt::RuntimeType>::DefaultType, }
                }
                TypeKind::Object
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Delegate(_) => {
                    quote! { &::std::option::Option<#tokens>, }
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
            if self.is_const {
                quote! { #name: &#tokens }
            } else {
                quote! { #name: #tokens }
            }
        } else {
            quote! { #name: *mut #tokens }
        }
    }

    pub fn gen_abi_return_arg(&self) -> TokenStream {
        if self.array {
            let return_type = self.kind.gen();
            quote! { ::winrt::Array::<#return_type>::set_abi_len(&mut result__), winrt::Array::<#return_type>::set_abi(&mut result__), }
        } else {
            quote! { &mut result__ }
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
                    TypeKind::String
                    | TypeKind::Object
                    | TypeKind::Class(_)
                    | TypeKind::Interface(_)
                    | TypeKind::Delegate(_)
                    | TypeKind::Generic(_) => quote! { #name.into().abi(), },
                    TypeKind::Enum(_) => quote! { #name, },
                    TypeKind::Guid | TypeKind::Struct(_) => {
                        if self.is_const {
                            quote! { &#name.into().abi(), }
                        } else {
                            quote! { #name.into().abi(), }
                        }
                    }
                    _ => quote! { ::winrt::Abi::abi(#name), },
                }
            }
        } else if self.kind.primitive() {
            quote! { #name, }
        } else {
            quote! { ::winrt::Abi::set_abi(#name), }
        }
    }

    pub fn gen_invoke_arg(&self, relative: bool) -> TokenStream {
        let name = format_ident(&self.name);

        let kind = if relative {
            self.kind.gen()
        } else {
            self.kind.gen_full()
        };

        // TODO: This compiles but doesn't property handle delegates with array parameters.
        // https://github.com/microsoft/winrt-rs/issues/212

        if self.array {
            if self.input {
                quote! { ::std::mem::transmute_copy(&#name) }
            } else if self.by_ref {
                quote! { ::std::mem::transmute_copy(&#name) }
            } else {
                quote! { ::std::mem::transmute_copy(&#name) }
            }
        } else if self.input {
            if self.kind.primitive() {
                quote! { #name }
            } else if let TypeKind::Enum(_) = self.kind {
                quote! { #name }
            } else {
                if self.is_const {
                    quote! { &*(#name as *const <#kind as ::winrt::Abi>::Abi as *const <#kind as ::winrt::RuntimeType>::DefaultType) }
                } else {
                    quote! { &*(&#name as *const <#kind as ::winrt::Abi>::Abi as *const <#kind as ::winrt::RuntimeType>::DefaultType) }
                }
            }
        } else {
            quote! { ::std::mem::transmute_copy(&#name) }
        }
    }
}
