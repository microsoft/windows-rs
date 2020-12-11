use crate::*;
use squote::{format_ident, quote, Ident, Literal, TokenStream};
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Param>,
    pub vtable_offset: u32,
    pub overload: u32,
}

impl Method {
    pub fn from_method_def(
        method: &winmd::MethodDef,
        vtable_offset: u32,
        generics: &[TypeKind],
        calling_namespace: &'static str,
    ) -> Method {
        let name = if method.flags().special() {
            let name = method.name();

            if name.starts_with("get") {
                to_snake(&name[4..], MethodKind::Get)
            } else if name.starts_with("put") {
                to_snake(&name[4..], MethodKind::Set)
            } else if name.starts_with("add") {
                to_snake(&name[4..], MethodKind::Add)
            } else if name.starts_with("remove") {
                to_snake(&name[7..], MethodKind::Remove)
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                "invoke".to_owned()
            }
        } else {
            Method::name(method)
        };

        let mut blob = method.sig();

        if blob.read_unsigned() & 0x10 != 0 {
            blob.read_unsigned();
        }

        let param_count = blob.read_unsigned();
        blob.read_modifiers();
        blob.read_expected(0x10);

        let return_type = if blob.read_expected(0x01) {
            None
        } else {
            let name = "result__".to_owned();
            let array = blob.peek_unsigned().0 == 0x1D;
            let t = Type::from_blob(&mut blob, generics, calling_namespace);
            let input = false;
            let by_ref = true;
            let is_const = false;
            Some(Param {
                name,
                kind: t.kind,
                array,
                input,
                by_ref,
                is_const,
            })
        };

        let mut params = Vec::with_capacity(param_count as usize);

        for param in method.params() {
            if return_type.is_none() || param.sequence() != 0 {
                let name = to_snake(param.name(), MethodKind::Normal);
                let input = !param.flags().output();

                let is_const = blob
                    .read_modifiers()
                    .iter()
                    .any(|def| def.name() == ("System.Runtime.CompilerServices", "IsConst"));

                let by_ref = blob.read_expected(0x10);
                let array = blob.peek_unsigned().0 == 0x1D;
                let t = Type::from_blob(&mut blob, generics, calling_namespace);

                params.push(Param {
                    name,
                    kind: t.kind,
                    array,
                    input,
                    by_ref,
                    is_const,
                });
            }
        }

        Method {
            name,
            params,
            return_type,
            vtable_offset,
            overload: 1,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.return_type
            .iter()
            .chain(self.params.iter())
            .flat_map(|i| i.kind.dependencies())
            .collect()
    }

    fn name(method: &winmd::MethodDef) -> String {
        for attribute in method.attributes() {
            if attribute.name() == ("Windows.Foundation.Metadata", "OverloadAttribute") {
                for (_, arg) in attribute.args() {
                    if let winmd::AttributeArg::String(name) = arg {
                        return to_snake(&name, MethodKind::Normal);
                    }
                }
            }
        }

        to_snake(method.name(), MethodKind::Normal)
    }

    pub fn gen_abi(&self) -> TokenStream {
        let params = self
            .params
            .iter()
            .chain(self.return_type.iter())
            .map(|param| param.gen_abi());

        quote! {
            (this: ::winrt::RawPtr, #(#params),*) -> ::winrt::ErrorCode
        }
    }

    pub fn gen_method(&self, interface: &TypeName, kind: InterfaceKind) -> TokenStream {
        // Composable interface methods drop their two trailing parameters when not aggregating
        // and forms the "default constructor" that projects as a "new" method in Rust.
        let method_name = if kind == InterfaceKind::Composable && self.params.len() == 2 {
            format_ident!("new")
        } else {
            self.gen_name()
        };

        let params = if kind == InterfaceKind::Composable {
            &self.params[..self.params.len() - 2]
        } else {
            &self.params
        };

        let constraints = gen_constraint(params);
        let args = params.iter().map(|param| param.gen_abi_arg());
        let params = gen_param(params);

        // The ABI obviously still has the two composable parameters. Here we just pass the default in and out
        // arguments to ensure the call succeeds in the non-aggregating case.
        let composable_args = if kind == InterfaceKind::Composable {
            quote! {
                ::std::ptr::null_mut(), ::winrt::Abi::set_abi(&mut ::std::option::Option::<::winrt::Object>::None),
            }
        } else {
            TokenStream::new()
        };

        let return_type_tokens = if let Some(return_type) = &self.return_type {
            return_type.gen_return()
        } else {
            quote! { () }
        };

        let vtable_offset = Literal::u32_unsuffixed(self.vtable_offset);

        let vcall = if let Some(return_type) = &self.return_type {
            let return_arg = return_type.gen_abi_return_arg();

            if return_type.array {
                quote! {
                    let mut result__: #return_type_tokens = ::std::mem::zeroed();
                    (::winrt::Interface::vtable(this).#vtable_offset)(::winrt::Abi::abi(this), #(#args)* #composable_args #return_arg)
                        .and_then(|| result__ )
                }
            } else {
                quote! {
                    let mut result__: <#return_type_tokens as ::winrt::Abi>::Abi = ::std::mem::zeroed();
                        (::winrt::Interface::vtable(this).#vtable_offset)(::winrt::Abi::abi(this), #(#args)* #composable_args #return_arg)
                            .from_abi::<#return_type_tokens>(result__ )
                }
            }
        } else {
            quote! {
                (::winrt::Interface::vtable(this).#vtable_offset)(::winrt::Abi::abi(this), #(#args)* #composable_args).ok()
            }
        };

        match kind {
            InterfaceKind::Default => quote! {
                pub fn #method_name<#constraints>(&self, #params) -> ::winrt::Result<#return_type_tokens> {
                    let this = self;
                    unsafe {
                        #vcall
                    }
                }
            },
            InterfaceKind::NonDefault | InterfaceKind::Overrides => {
                let interface = interface.gen();
                quote! {
                    pub fn #method_name<#constraints>(&self, #params) -> ::winrt::Result<#return_type_tokens> {
                        let this = &::winrt::Interface::cast::<#interface>(self).unwrap();
                        unsafe {
                            #vcall
                        }
                    }
                }
            }
            InterfaceKind::Statics | InterfaceKind::Composable => {
                let interface = interface.gen();
                quote! {
                    pub fn #method_name<#constraints>(#params) -> ::winrt::Result<#return_type_tokens> {
                        Self::#interface(|this| unsafe { #vcall })
                    }
                }
            }
        }
    }

    fn gen_name(&self) -> Ident {
        if self.overload > 1 {
            format_ident!("{}{}", &self.name, self.overload)
        } else {
            format_ident(&self.name)
        }
    }

    pub fn gen_upcall(&self, inner: TokenStream, relative: bool) -> TokenStream {
        let invoke_args = self
            .params
            .iter()
            .map(|param| param.gen_invoke_arg(relative));

        match &self.return_type {
            Some(return_type) if return_type.array => {
                let result = format_ident(&return_type.name);
                let result_size = squote::format_ident!("array_size_{}", &return_type.name);

                quote! {
                    match #inner(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(ok__) => {
                            let (ok_data__, ok_data_len__) = ok__.into_abi();
                            *#result = ok_data__;
                            *#result_size = ok_data_len__;
                            ::winrt::ErrorCode(0)
                        }
                        ::std::result::Result::Err(err) => err.into()
                    }
                }
            }
            Some(return_type) => {
                let return_name = format_ident(&return_type.name);

                quote! {
                    match #inner(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(ok__) => {
                            *#return_name = ::std::mem::transmute_copy(&ok__);
                            ::std::mem::forget(ok__);
                            ::winrt::ErrorCode(0)
                        }
                        ::std::result::Result::Err(err) => err.into()
                    }
                }
            }
            None => quote! {
                #inner(#(#invoke_args,)*).into()
            },
        }
    }
}

fn gen_param(params: &[Param]) -> TokenStream {
    TokenStream::from_iter(
        params
            .iter()
            .enumerate()
            .map(|(position, param)| param.gen(position)),
    )
}

fn gen_constraint(params: &[Param]) -> TokenStream {
    let mut tokens = Vec::new();

    for (position, param) in params.iter().enumerate() {
        if !param.input || param.array {
            continue;
        }

        match &param.kind {
            TypeKind::String
            | TypeKind::Object
            | TypeKind::Guid
            | TypeKind::Class(_)
            | TypeKind::Interface(_)
            | TypeKind::Struct(_)
            | TypeKind::Delegate(_)
            | TypeKind::Generic(_) => {
                let name = squote::format_ident!("T{}__", position);
                let into = param.kind.gen();
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

#[cfg(test)]
mod tests {
    use crate::*;

    fn method((namespace, type_name): (&str, &str), method_name: &str) -> Method {
        let reader = &winmd::TypeReader::from_build();
        let def = reader.resolve_type_def((namespace, type_name));

        let t = match TypeDefinition::from_type_def(&def) {
            TypeDefinition::Interface(t) => t,
            _ => panic!("TypeDefinition not an interface"),
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
