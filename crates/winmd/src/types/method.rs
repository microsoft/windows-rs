use crate::case::to_snake;
use crate::tables::{AttributeArg, MethodDef, TypeDef};
use crate::types::TypeName;
use crate::types::{Param, RequiredInterface, TypeKind};
use crate::TypeReader;
use crate::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub kind: MethodKind,
    pub params: Vec<Param>,
    pub return_type: Option<Param>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MethodKind {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}

impl Method {
    pub fn from_method_def(
        reader: &TypeReader,
        method: MethodDef,
        generics: &Vec<TypeKind>,
    ) -> Method {
        let (name, kind) = if method.flags(reader).special() {
            let name = method.name(reader);

            if name.starts_with("get") {
                (case::to_snake(&name[4..], MethodKind::Get), MethodKind::Get)
            } else if name.starts_with("put") {
                (case::to_snake(&name[4..], MethodKind::Set), MethodKind::Set)
            } else if name.starts_with("add") {
                (case::to_snake(&name[4..], MethodKind::Add), MethodKind::Add)
            } else if name.starts_with("remove") {
                (
                    case::to_snake(&name[7..], MethodKind::Remove),
                    MethodKind::Remove,
                )
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                ("invoke".to_owned(), MethodKind::Normal)
            }
        } else {
            (Method::name(reader, method), MethodKind::Normal)
        };

        let mut blob = method.sig(reader);

        if blob.read_unsigned() & 0x10 != 0 {
            blob.read_unsigned();
        }

        let param_count = blob.read_unsigned();
        blob.read_modifiers();
        blob.read_expected(0x10);

        let return_type = if blob.read_expected(0x01) {
            None
        } else {
            let name = "__result".to_owned();
            let array = blob.peek_unsigned().0 == 0x1D;
            let kind = TypeKind::from_blob(&mut blob, generics);
            let input = false;
            let by_ref = true;
            Some(Param {
                name,
                kind,
                array,
                input,
                by_ref,
            })
        };

        let mut params = Vec::with_capacity(param_count as usize);

        for param in method.params(reader) {
            if return_type.is_none() || param.sequence(reader) != 0 {
                let name = to_snake(param.name(reader), MethodKind::Normal);
                let input = param.flags(reader).input();

                blob.read_modifiers();
                let by_ref = blob.read_expected(0x10);
                let array = blob.peek_unsigned().0 == 0x1D;
                let kind = TypeKind::from_blob(&mut blob, generics);

                params.push(Param {
                    name,
                    kind,
                    array,
                    input,
                    by_ref,
                });
            }
        }

        Method {
            name,
            kind,
            params,
            return_type,
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.return_type
            .iter()
            .chain(self.params.iter())
            .flat_map(|i| i.kind.dependencies())
            .collect()
    }

    fn name(reader: &TypeReader, method: MethodDef) -> String {
        if let Some(attribute) =
            method.find_attribute(reader, ("Windows.Foundation.Metadata", "OverloadAttribute"))
        {
            for (_, arg) in attribute.args(reader) {
                if let AttributeArg::String(name) = arg {
                    return case::to_snake(&name, MethodKind::Normal);
                }
            }
        }

        case::to_snake(method.name(reader), MethodKind::Normal)
    }

    pub fn to_abi_tokens(&self, self_name: &TypeName, calling_namespace: &str) -> TokenStream {
        let abi_name = self_name.to_abi_tokens(calling_namespace);
        let name = format_ident(&self.name);
        let params = TokenStream::from_iter(
            self.params
                .iter()
                .chain(self.return_type.iter())
                .map(|param| param.to_abi_tokens(calling_namespace)),
        );

        quote! {
            pub #name: extern "system" fn(*const *const #abi_name, #params) -> ::winrt::ErrorCode,
        }
    }

    pub fn to_abi_impl_tokens(&self, self_name: &TypeName, calling_namespace: &str) -> TokenStream {
        let abi_name = self_name.to_abi_tokens(calling_namespace);
        let name = format_ident(&self.name);
        let params = self
            .params
            .iter()
            .chain(self.return_type.iter())
            .map(|param| {
                let name = format_ident(&param.name);
                let abi = param.to_abi_tokens(calling_namespace);
                quote! { #name: #abi }
            });

        quote! {
            extern "system" fn #name(this: *const *const #abi_name, #(#params)*) -> ::winrt::ErrorCode
        }
    }

    fn to_param_tokens(&self, calling_namespace: &str) -> TokenStream {
        TokenStream::from_iter(
            self.params
                .iter()
                .enumerate()
                .map(|(position, param)| param.to_tokens(calling_namespace, position)),
        )
    }

    fn to_arg_tokens(&self) -> TokenStream {
        TokenStream::from_iter(self.params.iter().map(|param| {
            let name = format_ident(&param.name);
            quote! { #name, }
        }))
    }

    fn to_abi_arg_tokens(&self) -> TokenStream {
        TokenStream::from_iter(self.params.iter().map(|param| param.to_abi_arg_tokens()))
    }

    fn to_constraint_tokens(&self, calling_namespace: &str) -> TokenStream {
        let mut tokens = Vec::new();

        for (position, param) in self.params.iter().enumerate() {
            if !param.input || param.array {
                continue;
            }

            match param.kind {
                TypeKind::String
                | TypeKind::Object
                | TypeKind::Guid
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Struct(_)
                | TypeKind::Delegate(_)
                | TypeKind::Generic(_) => {
                    let name = quote::format_ident!("__{}", position);
                    let into = param.kind.to_tokens(calling_namespace);
                    tokens.push(quote! { #name: ::std::convert::Into<::winrt::Param<'a, #into>>, });
                }
                _ => {}
            };
        }

        if !tokens.is_empty() {
            tokens.insert(0, quote! { 'a, });
        }

        TokenStream::from_iter(tokens)
    }

    pub fn to_default_tokens(&self, calling_namespace: &str) -> TokenStream {
        let method_name = format_ident(&self.name);
        let params = self.to_param_tokens(calling_namespace);
        let constraints = self.to_constraint_tokens(calling_namespace);
        let args = self.to_abi_arg_tokens();

        if let Some(return_type) = &self.return_type {
            let return_arg = return_type.to_abi_return_arg_tokens(calling_namespace);
            let return_type = return_type.to_return_tokens(calling_namespace);

            quote! {
                pub fn #method_name<#constraints>(&self, #params) -> ::winrt::Result<#return_type> {
                    let this = <::winrt::ComPtr<Self> as ::winrt::ComInterface>::as_raw(&self.ptr);

                    if this.is_null() {
                        panic!("The `this` pointer was null when calling method");
                    }
                    unsafe {
                        let mut __ok: #return_type = ::std::mem::zeroed();
                        ((*(*(this))).#method_name)(this, #args #return_arg)
                            .and_then(|| __ok )
                    }
                }
            }
        } else {
            quote! {
                pub fn #method_name<#constraints>(&self, #params) -> ::winrt::Result<()> {
                    let this = <::winrt::ComPtr<Self> as ::winrt::ComInterface>::as_raw(&self.ptr);

                    if this.is_null() {
                        panic!("The `this` pointer was null when calling method");
                    }
                    unsafe {
                        ((*(*(this))).#method_name)(this, #args).ok()
                    }
                }
            }
        }
    }

    pub fn to_non_default_tokens(
        &self,
        calling_namespace: &str,
        interface: &RequiredInterface,
    ) -> TokenStream {
        let method_name = format_ident(&self.name);
        let params = self.to_param_tokens(calling_namespace);
        let constraints = self.to_constraint_tokens(calling_namespace);
        let args = self.to_arg_tokens();
        let interface = &*interface.name.to_tokens(calling_namespace);

        let return_type = if let Some(return_type) = &self.return_type {
            return_type.to_return_tokens(calling_namespace)
        } else {
            quote! { () }
        };

        quote! {
            pub fn #method_name<#constraints>(&self, #params) -> ::winrt::Result<#return_type> {
                <#interface as ::std::convert::From<&Self>>::from(self).#method_name(#args)
            }
        }
    }

    pub fn to_static_tokens(
        &self,
        calling_namespace: &str,
        interface: &RequiredInterface,
    ) -> TokenStream {
        let method_name = format_ident(&self.name);
        let params = self.to_param_tokens(calling_namespace);
        let constraints = self.to_constraint_tokens(calling_namespace);
        let args = self.to_arg_tokens();
        let interface = &*interface.name.to_tokens(calling_namespace);

        let return_type = if let Some(return_type) = &self.return_type {
            return_type.to_return_tokens(calling_namespace)
        } else {
            quote! { () }
        };

        quote! {
            pub fn #method_name<#constraints>(#params) -> ::winrt::Result<#return_type> {
                ::winrt::factory::<Self, #interface>()?.#method_name(#args)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    fn method((namespace, type_name): (&str, &str), method_name: &str) -> Method {
        let reader = &TypeReader::from_os();
        let def = reader.resolve_type_def((namespace, type_name));

        let t = match def.into_type(reader) {
            Type::Interface(t) => t,
            _ => panic!("Type not an interface"),
        };

        for interface in t.interfaces {
            for method in interface.methods {
                if method.name == method_name {
                    return method;
                }
            }
        }

        panic!("Method not found");
    }

    #[test]
    fn test_to_string() {
        let method = method(("Windows.Foundation", "IStringable"), "to_string");
        assert!(method.kind == MethodKind::Normal);
        assert!(method.params.is_empty());

        let param = method.return_type.as_ref().unwrap();
        assert!(param.kind == TypeKind::String);
    }

    #[test]
    fn test_map_changed() {
        let method = method(
            ("Windows.Foundation.Collections", "IObservableMap`2"),
            "map_changed",
        );

        assert!(method.kind == MethodKind::Add);
        assert!(method.params.len() == 1);

        let handler = &method.params[0];
        assert!(handler.array == false);
        assert!(handler.input == true);
        assert!(handler.by_ref == false);

        let handler = match &handler.kind {
            TypeKind::Delegate(delegate) => delegate,
            _ => panic!("Wrong type"),
        };

        assert!(
            handler.runtime_name()
                == "Windows.Foundation.Collections.MapChangedEventHandler`2<K, V>"
        );

        let token = method.return_type.as_ref().unwrap();
        assert!(token.array == false);
        assert!(token.input == false);
        assert!(token.by_ref == true);

        let token = match &token.kind {
            TypeKind::Struct(token) => token,
            _ => panic!("Wrong type"),
        };

        assert!(token.runtime_name() == "Windows.Foundation.EventRegistrationToken");
    }

    #[test]
    fn test_remove_map_changed() {
        let method = method(
            ("Windows.Foundation.Collections", "IObservableMap`2"),
            "remove_map_changed",
        );

        assert!(method.kind == MethodKind::Remove);
        assert!(method.params.len() == 1);

        let token = &method.params[0];
        assert!(token.array == false);
        assert!(token.input == true);
        assert!(token.by_ref == false);

        let token = match &token.kind {
            TypeKind::Struct(token) => token,
            _ => panic!("Wrong type"),
        };

        assert!(token.runtime_name() == "Windows.Foundation.EventRegistrationToken");
    }
}
