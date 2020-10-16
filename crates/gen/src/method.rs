use crate::*;
use squote::{quote, TokenStream};
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Param>,
}

impl Method {
    pub fn from_method_def(
        reader: &winmd::TypeReader,
        method: winmd::MethodDef,
        generics: &[TypeKind],
        calling_namespace: &str,
    ) -> Method {
        let name = if method.flags(reader).special() {
            let name = method.name(reader);

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
            Method::name(reader, method)
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
            let name = "result__".to_owned();
            let array = blob.peek_unsigned().0 == 0x1D;
            let kind = TypeKind::from_blob(&mut blob, generics, calling_namespace);
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
                let kind = TypeKind::from_blob(&mut blob, generics, calling_namespace);

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
            params,
            return_type,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.return_type
            .iter()
            .chain(self.params.iter())
            .flat_map(|i| i.kind.dependencies())
            .collect()
    }

    fn name(reader: &winmd::TypeReader, method: winmd::MethodDef) -> String {
        if let Some(attribute) =
            method.find_attribute(reader, ("Windows.Foundation.Metadata", "OverloadAttribute"))
        {
            for (_, arg) in attribute.args(reader) {
                if let winmd::AttributeArg::String(name) = arg {
                    return to_snake(&name, MethodKind::Normal);
                }
            }
        }

        to_snake(method.name(reader), MethodKind::Normal)
    }

    pub fn gen_abi(&self, self_name: &TypeName) -> TokenStream {
        let type_name = self_name.gen();
        let name = format_ident(&self.name);
        let params = TokenStream::from_iter(
            self.params
                .iter()
                .chain(self.return_type.iter())
                .map(|param| param.gen_abi()),
        );

        quote! {
            pub #name: unsafe extern "system" fn(::winrt::NonNullRawComPtr<#type_name>, #params) -> ::winrt::ErrorCode,
        }
    }

    pub fn gen_abi_impl(&self, self_name: &TypeName) -> TokenStream {
        let type_name = self_name.gen();
        let name = format_ident(&self.name);
        let params = self
            .params
            .iter()
            .chain(self.return_type.iter())
            .map(|param| {
                let abi = param.gen_abi();
                quote! { #abi }
            });

        quote! {
            unsafe extern "system" fn #name(this: ::winrt::NonNullRawComPtr<#type_name>, #(#params)*) -> ::winrt::ErrorCode
        }
    }

    pub fn gen_binding_abi_impl(&self, self_name: &TypeName) -> TokenStream {
        let type_name = self_name.gen_binding();
        let name = format_ident(&self.name);
        let params = self
            .params
            .iter()
            .chain(self.return_type.iter())
            .map(|param| {
                let abi = param.gen_abi();
                quote! { #abi }
            });

        quote! {
            unsafe extern "system" fn #name(this: ::winrt::NonNullRawComPtr<#type_name>, #(#params)*) -> ::winrt::ErrorCode
        }
    }

    pub fn gen_default(&self) -> TokenStream {
        let method_name = format_ident(&self.name);
        let params = gen_param(&self.params);
        let constraints = gen_constraint(&self.params);

        let args = TokenStream::from_iter(self.params.iter().map(|param| param.gen_abi_arg()));

        if let Some(return_type) = &self.return_type {
            let return_arg = return_type.gen_abi_return_arg();
            let return_type = return_type.gen_return();

            quote! {
                pub fn #method_name<#constraints>(&self, #params) -> ::winrt::Result<#return_type> {
                    let this = <Self as ::winrt::AbiTransferable>::get_abi(self).expect("The `this` pointer was null when calling method");
                    unsafe {
                        let mut result__: #return_type = ::std::mem::zeroed();
                        (this.vtable().#method_name)(this, #args #return_arg)
                            .and_then(|| result__ )
                    }
                }
            }
        } else {
            quote! {
                pub fn #method_name<#constraints>(&self, #params) -> ::winrt::Result<()> {
                    let this = <Self as ::winrt::AbiTransferable>::get_abi(self).expect("The `this` pointer was null when calling method");
                    unsafe {
                        (this.vtable().#method_name)(this, #args).ok()
                    }
                }
            }
        }
    }

    pub fn gen_non_default(&self, interface: &RequiredInterface) -> TokenStream {
        let method_name = format_ident(&self.name);
        let params = gen_param(&self.params);
        let constraints = gen_constraint(&self.params);
        let args = gen_arg(&self.params);
        let interface = interface.name.gen();

        let return_type = if let Some(return_type) = &self.return_type {
            return_type.gen_return()
        } else {
            quote! { () }
        };

        quote! {
            pub fn #method_name<#constraints>(&self, #params) -> ::winrt::Result<#return_type> {
                <#interface as ::std::convert::From<&Self>>::from(self).#method_name(#args)
            }
        }
    }

    pub fn gen_static(&self, interface: &RequiredInterface) -> TokenStream {
        let method_name = format_ident(&self.name);
        let params = gen_param(&self.params);
        let constraints = gen_constraint(&self.params);
        let args = gen_arg(&self.params);
        let interface = format_ident(&interface.name.name);

        let return_type = if let Some(return_type) = &self.return_type {
            return_type.gen_return()
        } else {
            quote! { () }
        };

        quote! {
            pub fn #method_name<#constraints>(#params) -> ::winrt::Result<#return_type> {
                Self::#interface(|f| f.#method_name(#args))
            }
        }
    }

    pub fn gen_composable(&self, interface: &RequiredInterface) -> TokenStream {
        let method_name = format_ident(&self.name);
        let interface = format_ident(&interface.name.name);

        if self.params.len() == 2 {
            // For non-aggregation scenarios this is just a default constructor, hence the
            // "new" method name in line with non-composable default constructors.
            quote! {
                pub fn new() -> ::winrt::Result<Self> {
                    Self::#interface(|f| f.#method_name(::winrt::Object::default(), &mut ::winrt::Object::default()))
                }
            }
        } else {
            let params = &self.params[..self.params.len() - 2];
            let constraints = gen_constraint(params);
            let args = gen_arg(params);
            let params = gen_param(params);

            quote! {
                pub fn #method_name<#constraints>(#params) -> ::winrt::Result<Self> {
                    Self::#interface(|f| f.#method_name(#args ::winrt::Object::default(), &mut ::winrt::Object::default()))
                }
            }
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

fn gen_arg(params: &[Param]) -> TokenStream {
    TokenStream::from_iter(params.iter().map(|param| {
        let name = format_ident(&param.name);
        quote! { #name, }
    }))
}

fn gen_constraint(params: &[Param]) -> TokenStream {
    let mut tokens = Vec::new();

    for (position, param) in params.iter().enumerate() {
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
        let reader = &winmd::TypeReader::from_os();
        let def = reader.resolve_type_def((namespace, type_name));

        let t = match Type::from_type_def(reader, def) {
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
