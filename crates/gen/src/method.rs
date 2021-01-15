use crate::*;
use squote::{format_ident, quote, Ident, Literal, TokenStream};
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub vtable_offset: u32,
    pub overload: u32,
    pub signature: Signature,
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
                method_to_snake(&name[4..], MethodKind::Get)
            } else if name.starts_with("put") {
                method_to_snake(&name[4..], MethodKind::Set)
            } else if name.starts_with("add") {
                method_to_snake(&name[4..], MethodKind::Add)
            } else if name.starts_with("remove") {
                method_to_snake(&name[7..], MethodKind::Remove)
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                "invoke".to_owned()
            }
        } else {
            Method::name(method)
        };

        let signature = Signature::new(method, generics, calling_namespace);

        Method {
            name,
            signature,
            vtable_offset,
            overload: 1,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.signature.dependencies()
    }

    fn name(method: &winmd::MethodDef) -> String {
        for attribute in method.attributes() {
            if attribute.name() == ("Windows.Foundation.Metadata", "OverloadAttribute") {
                for (_, arg) in attribute.args() {
                    if let winmd::AttributeArg::String(name) = arg {
                        return method_to_snake(&name, MethodKind::Normal);
                    }
                }
            }
        }

        method_to_snake(method.name(), MethodKind::Normal)
    }

    pub fn gen_abi(&self) -> TokenStream {
        let params = self
            .signature
            .params
            .iter()
            .chain(self.signature.return_type.iter())
            .map(|param| param_gen_abi(param));

        quote! {
            (this: ::windows::RawPtr, #(#params),*) -> ::windows::ErrorCode
        }
    }

    pub fn gen_full_abi(&self) -> TokenStream {
        let params = self
            .signature
            .params
            .iter()
            .chain(self.signature.return_type.iter())
            .map(|param| param_gen_full_abi(param));

        quote! {
            (this: ::windows::RawPtr, #(#params),*) -> ::windows::ErrorCode
        }
    }

    pub fn gen_method(&self, interface: &TypeName, kind: InterfaceKind) -> TokenStream {
        // Composable interface methods drop their two trailing parameters when not aggregating
        // and forms the "default constructor" that projects as a "new" method in Rust.
        let method_name = if kind == InterfaceKind::Composable && self.signature.params.len() == 2 {
            format_ident!("new")
        } else {
            self.gen_name()
        };

        let params = if kind == InterfaceKind::Composable {
            &self.signature.params[..self.signature.params.len() - 2]
        } else {
            &self.signature.params
        };

        let constraints = gen_constraint(params);
        let args = params.iter().map(|param| param_gen_abi_arg(param));
        let params = gen_param2(params);

        // The ABI obviously still has the two composable parameters. Here we just pass the default in and out
        // arguments to ensure the call succeeds in the non-aggregating case.
        let composable_args = if kind == InterfaceKind::Composable {
            quote! {
                ::std::ptr::null_mut(), ::windows::Abi::set_abi(&mut ::std::option::Option::<::windows::Object>::None),
            }
        } else {
            TokenStream::new()
        };

        // TODO: move duplicate code to Type
        let return_type_tokens = if let Some(return_type) = &self.signature.return_type {
            param_gen_return(return_type)
        } else {
            quote! { () }
        };

        let vtable_offset = Literal::u32_unsuffixed(self.vtable_offset);

        let vcall = if let Some(return_type) = &self.signature.return_type {
            let return_arg = param_gen_abi_return_arg(return_type);

            if return_type.is_array {
                quote! {
                    let mut result__: #return_type_tokens = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args)* #composable_args #return_arg)
                        .and_then(|| result__ )
                }
            } else {
                quote! {
                    let mut result__: <#return_type_tokens as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args)* #composable_args #return_arg)
                            .from_abi::<#return_type_tokens>(result__ )
                }
            }
        } else {
            quote! {
                (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args)* #composable_args).ok()
            }
        };

        match kind {
            InterfaceKind::Default => quote! {
                pub fn #method_name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                    let this = self;
                    unsafe {
                        #vcall
                    }
                }
            },
            InterfaceKind::NonDefault | InterfaceKind::Overrides => {
                let interface = interface.gen();
                quote! {
                    pub fn #method_name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                        let this = &::windows::Interface::cast::<#interface>(self).unwrap();
                        unsafe {
                            #vcall
                        }
                    }
                }
            }
            InterfaceKind::Statics | InterfaceKind::Composable => {
                let interface = interface.gen();
                quote! {
                    pub fn #method_name<#constraints>(#params) -> ::windows::Result<#return_type_tokens> {
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
            .signature
            .params
            .iter()
            .map(|param| param_gen_invoke_arg(param, relative));

        match &self.signature.return_type {
            Some(return_type) if return_type.is_array => {
                let result = format_ident(&return_type.name);
                let result_size = squote::format_ident!("array_size_{}", &return_type.name);

                quote! {
                    match #inner(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(ok__) => {
                            let (ok_data__, ok_data_len__) = ok__.into_abi();
                            *#result = ok_data__;
                            *#result_size = ok_data_len__;
                            ::windows::ErrorCode(0)
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
                            ::windows::ErrorCode(0)
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

fn gen_param2(types: &[Type]) -> TokenStream {
    TokenStream::from_iter(
        types
            .iter()
            .enumerate()
            .map(|(position, param)| param_gen(param, position)),
    )
}

fn gen_constraint(types: &[Type]) -> TokenStream {
    let mut tokens = Vec::new();

    for (position, param) in types.iter().enumerate() {
        if !param.is_input || param.is_array {
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
                tokens.push(quote! { #name: ::std::convert::Into<::windows::Param<'a, #into>>, });
            }
            _ => {}
        };
    }

    if !tokens.is_empty() {
        tokens.insert(0, quote! { 'a, });
    }

    TokenStream::from_iter(tokens)
}

fn param_gen(t: &Type, position: usize) -> TokenStream {
    let name = format_ident(&t.name);
    let tokens = t.kind.gen();

    if t.is_array {
        if t.is_input {
            quote! { #name: &[<#tokens as ::windows::RuntimeType>::DefaultType], }
        } else if t.by_ref {
            quote! { #name: &mut ::windows::Array<#tokens>, }
        } else {
            quote! { #name: &mut [<#tokens as ::windows::RuntimeType>::DefaultType], }
        }
    } else if t.is_input {
        match &t.kind {
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
        match t.kind {
            TypeKind::Object
            | TypeKind::Class(_)
            | TypeKind::Interface(_)
            | TypeKind::Delegate(_) => {
                quote! { #name: &mut ::std::option::Option<#tokens>, }
            }
            TypeKind::Generic(_) => {
                quote! { &mut <#tokens as ::windows::RuntimeType>::DefaultType, }
            }
            _ => quote! { #name: &mut #tokens, },
        }
    }
}

pub fn param_gen_return(t: &Type) -> TokenStream {
    let tokens = t.kind.gen();

    if t.is_array {
        quote! { ::windows::Array<#tokens> }
    } else {
        quote! { #tokens }
    }
}

fn gen_abi_wrap(t: &Type, kind_tokens: TokenStream) -> TokenStream {
    let name = format_ident(&t.name);

    if t.is_array {
        let name_size = squote::format_ident!("array_size_{}", &t.name);

        if t.is_input {
            quote! { #name_size: u32, #name: *const #kind_tokens }
        } else if t.by_ref {
            quote! { #name_size: *mut u32, #name: *mut *mut #kind_tokens }
        } else {
            quote! { #name_size: u32, #name: *mut #kind_tokens }
        }
    } else if t.is_input {
        if t.is_const {
            quote! { #name: &#kind_tokens }
        } else {
            quote! { #name: #kind_tokens }
        }
    } else {
        quote! { #name: *mut #kind_tokens }
    }
}

fn param_gen_abi(t: &Type) -> TokenStream {
    let tokens = t.kind.gen_abi();

    gen_abi_wrap(t, tokens)
}

fn param_gen_full_abi(t: &Type) -> TokenStream {
    let tokens = t.kind.gen_full_abi();

    gen_abi_wrap(t, tokens)
}

fn param_gen_abi_return_arg(t: &Type) -> TokenStream {
    if t.is_array {
        let return_type = t.kind.gen();
        quote! { ::windows::Array::<#return_type>::set_abi_len(&mut result__), windows::Array::<#return_type>::set_abi(&mut result__), }
    } else {
        quote! { &mut result__ }
    }
}

fn param_gen_abi_arg(t: &Type) -> TokenStream {
    let name = format_ident(&t.name);

    if t.is_array {
        if t.is_input {
            quote! { #name.len() as u32, ::std::mem::transmute(#name.as_ptr()), }
        } else if t.by_ref {
            quote! { #name.set_abi_len(), #name.set_abi(), }
        } else {
            quote! { #name.len() as u32, ::std::mem::transmute_copy(&#name), }
        }
    } else if t.is_input {
        if t.kind.primitive() {
            quote! { #name, }
        } else {
            match t.kind {
                TypeKind::String
                | TypeKind::Object
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Delegate(_)
                | TypeKind::Generic(_) => quote! { #name.into().abi(), },
                TypeKind::Enum(_) => quote! { #name, },
                TypeKind::Guid | TypeKind::Struct(_) => {
                    if t.is_const {
                        quote! { &#name.into().abi(), }
                    } else {
                        quote! { #name.into().abi(), }
                    }
                }
                _ => quote! { ::windows::Abi::abi(#name), },
            }
        }
    } else if t.kind.primitive() {
        quote! { #name, }
    } else {
        quote! { ::windows::Abi::set_abi(#name), }
    }
}

fn param_gen_invoke_arg(t: &Type, relative: bool) -> TokenStream {
    let name = format_ident(&t.name);

    let kind = if relative {
        t.kind.gen()
    } else {
        t.kind.gen_full()
    };

    // TODO: This compiles but doesn't property handle delegates with array parameters.
    // https://github.com/microsoft/windows-rs/issues/212

    if t.is_array {
        if t.is_input {
            quote! { ::std::mem::transmute_copy(&#name) }
        } else if t.by_ref {
            quote! { ::std::mem::transmute_copy(&#name) }
        } else {
            quote! { ::std::mem::transmute_copy(&#name) }
        }
    } else if t.is_input {
        if t.kind.primitive() {
            quote! { #name }
        } else if let TypeKind::Enum(_) = t.kind {
            quote! { #name }
        } else {
            if t.is_const {
                quote! { &*(#name as *const <#kind as ::windows::Abi>::Abi as *const <#kind as ::windows::RuntimeType>::DefaultType) }
            } else {
                quote! { &*(&#name as *const <#kind as ::windows::Abi>::Abi as *const <#kind as ::windows::RuntimeType>::DefaultType) }
            }
        }
    } else {
        quote! { ::std::mem::transmute_copy(&#name) }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn method((namespace, type_name): (&str, &str), method_name: &str) -> Method {
        let reader = &winmd::TypeReader::get();
        let def = reader.expect_type_def((namespace, type_name));

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
        assert!(method.signature.params.is_empty());

        let param = method.signature.return_type.as_ref().unwrap();
        assert!(param.kind == TypeKind::String);
    }

    #[test]
    fn test_map_changed() {
        let method = method(
            ("Windows.Foundation.Collections", "IObservableMap`2"),
            "map_changed",
        );

        assert!(method.signature.params.len() == 1);

        let handler = &method.signature.params[0];
        assert!(handler.is_array == false);
        assert!(handler.is_input == true);
        assert!(handler.by_ref == false);

        let handler = match &handler.kind {
            TypeKind::Delegate(delegate) => delegate,
            _ => panic!("Wrong type"),
        };

        assert!(
            handler.runtime_name()
                == "Windows.Foundation.Collections.MapChangedEventHandler`2<K, V>"
        );

        let token = method.signature.return_type.as_ref().unwrap();
        assert!(token.is_array == false);
        assert!(token.is_input == false);
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

        assert!(method.signature.params.len() == 1);

        let token = &method.signature.params[0];
        assert!(token.is_array == false);
        assert!(token.is_input == true);
        assert!(token.by_ref == false);

        let token = match &token.kind {
            TypeKind::Struct(token) => token,
            _ => panic!("Wrong type"),
        };

        assert!(token.runtime_name() == "Windows.Foundation.EventRegistrationToken");
    }
}
