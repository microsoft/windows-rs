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

        let mut blob = method.sig();

        if blob.read_unsigned() & 0x10 != 0 {
            panic!();
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
                let name = to_snake(param.name());
                let input = !param.flags().output();

                let is_const = blob
                    .read_modifiers()
                    .iter()
                    .any(|def| def.name() == ("System.Runtime.CompilerServices", "IsConst"));

                    // if is_const {
                    //     panic!(format!("{}", method.name()));
                    // }

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

        let signature = Signature::new(method, generics, calling_namespace);

        Method {
            name,
            signature,
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
                        return method_to_snake(&name, MethodKind::Normal);
                    }
                }
            }
        }

        method_to_snake(method.name(), MethodKind::Normal)
    }

    pub fn gen_abi(&self) -> TokenStream {
        let params = self
            .params
            .iter()
            .chain(self.return_type.iter())
            .map(|param| param_gen_abi(param));

        quote! {
            (this: ::winrt::RawPtr, #(#params),*) -> ::winrt::ErrorCode
        }
    }

    pub fn gen_full_abi(&self) -> TokenStream {
        let params = self
            .params
            .iter()
            .chain(self.return_type.iter())
            .map(|param| param_gen_full_abi(param));

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
        let args = params.iter().map(|param| param_gen_abi_arg(param));
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

        // TODO: move duplicate code to Type
        let return_type_tokens = if let Some(return_type) = &self.return_type {
            param_gen_return(return_type)
        } else {
            quote! { () }
        };

        let vtable_offset = Literal::u32_unsuffixed(self.vtable_offset);

        let vcall = if let Some(return_type) = &self.return_type {
            let return_arg = param_gen_abi_return_arg(return_type);

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
            .map(|param| param_gen_invoke_arg(param, relative));

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
            .map(|(position, param)| param_gen(param, position)),
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

fn param_gen(param: &Param, position: usize) -> TokenStream {
    let name = format_ident(&param.name);
    let tokens = param.kind.gen();

    if param.array {
        if param.input {
            quote! { #name: &[<#tokens as ::winrt::RuntimeType>::DefaultType], }
        } else if param.by_ref {
            quote! { #name: &mut ::winrt::Array<#tokens>, }
        } else {
            quote! { #name: &mut [<#tokens as ::winrt::RuntimeType>::DefaultType], }
        }
    } else if param.input {
        match &param.kind {
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
        match param.kind {
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

pub fn param_gen_return(param: &Param) -> TokenStream {
    let tokens = param.kind.gen();

    if param.array {
        quote! { ::winrt::Array<#tokens> }
    } else {
        quote! { #tokens }
    }
}

fn gen_abi_wrap(param:&Param, kind_tokens: TokenStream) -> TokenStream {
    let name = format_ident(&param.name);

    if param.array {
        let name_size = squote::format_ident!("array_size_{}", &param.name);

        if param.input {
            quote! { #name_size: u32, #name: *const #kind_tokens }
        } else if param.by_ref {
            quote! { #name_size: *mut u32, #name: *mut *mut #kind_tokens }
        } else {
            quote! { #name_size: u32, #name: *mut #kind_tokens }
        }
    } else if param.input {
        if param.is_const {
            quote! { #name: &#kind_tokens }
        } else {
            quote! { #name: #kind_tokens }
        }
    } else {
        quote! { #name: *mut #kind_tokens }
    }
}

pub fn param_gen_abi(param:&Param) -> TokenStream {
    let tokens = param.kind.gen_abi();

    gen_abi_wrap(param, tokens)
}

pub fn param_gen_full_abi(param:&Param) -> TokenStream {
    let tokens = param.kind.gen_full_abi();

    gen_abi_wrap(param, tokens)
}

pub fn param_gen_abi_return_arg(param:&Param) -> TokenStream {
    if param.array {
        let return_type = param.kind.gen();
        quote! { ::winrt::Array::<#return_type>::set_abi_len(&mut result__), winrt::Array::<#return_type>::set_abi(&mut result__), }
    } else {
        quote! { &mut result__ }
    }
}

pub fn param_gen_abi_arg(param:&Param) -> TokenStream {
    let name = format_ident(&param.name);

    if param.array {
        if param.input {
            quote! { #name.len() as u32, ::std::mem::transmute(#name.as_ptr()), }
        } else if param.by_ref {
            quote! { #name.set_abi_len(), #name.set_abi(), }
        } else {
            quote! { #name.len() as u32, ::std::mem::transmute_copy(&#name), }
        }
    } else if param.input {
        if param.kind.primitive() {
            quote! { #name, }
        } else {
            match param.kind {
                TypeKind::String
                | TypeKind::Object
                | TypeKind::Class(_)
                | TypeKind::Interface(_)
                | TypeKind::Delegate(_)
                | TypeKind::Generic(_) => quote! { #name.into().abi(), },
                TypeKind::Enum(_) => quote! { #name, },
                TypeKind::Guid | TypeKind::Struct(_) => {
                    if param.is_const {
                        quote! { &#name.into().abi(), }
                    } else {
                        quote! { #name.into().abi(), }
                    }
                }
                _ => quote! { ::winrt::Abi::abi(#name), },
            }
        }
    } else if param.kind.primitive() {
        quote! { #name, }
    } else {
        quote! { ::winrt::Abi::set_abi(#name), }
    }
}

pub fn param_gen_invoke_arg(param:&Param, relative: bool) -> TokenStream {
    let name = format_ident(&param.name);

    let kind = if relative {
        param.kind.gen()
    } else {
        param.kind.gen_full()
    };

    // TODO: This compiles but doesn't property handle delegates with array parameters.
    // https://github.com/microsoft/winrt-rs/issues/212

    if param.array {
        if param.input {
            quote! { ::std::mem::transmute_copy(&#name) }
        } else if param.by_ref {
            quote! { ::std::mem::transmute_copy(&#name) }
        } else {
            quote! { ::std::mem::transmute_copy(&#name) }
        }
    } else if param.input {
        if param.kind.primitive() {
            quote! { #name }
        } else if let TypeKind::Enum(_) = param.kind {
            quote! { #name }
        } else {
            if param.is_const {
                quote! { &*(#name as *const <#kind as ::winrt::Abi>::Abi as *const <#kind as ::winrt::RuntimeType>::DefaultType) }
            } else {
                quote! { &*(&#name as *const <#kind as ::winrt::Abi>::Abi as *const <#kind as ::winrt::RuntimeType>::DefaultType) }
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
