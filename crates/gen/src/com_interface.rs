use crate::*;
use squote::{format_ident, quote, Ident, Literal, TokenStream};
use std::collections::BTreeMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct ComInterface {
    pub name: TypeName,
    methods: Vec<Method>,
    bases: Vec<TypeName>,
}

#[derive(Debug)]
struct Method {
    signature: Signature,
    overload: u32,
}

impl Method {
    fn gen_name(&self) -> Ident {
        if self.overload > 1 {
            format_ident!("{}{}", self.signature.method.name(), self.overload)
        } else {
            format_ident(self.signature.method.name())
        }
    }
}

impl ComInterface {
    pub fn from_type_name(name: TypeName) -> Self {
        let mut count = BTreeMap::new();

        let mut bases = Vec::new();
        let mut next = name.def;

        loop {
            let base = if let Some(next) = next.interfaces().next() {
                next.interface()
            } else {
                break;
            };

            if base.name() == ("Windows.Win32.Com", "IUnknown") {
                break;
            }

            let base = TypeName::new(&base.resolve(), Vec::new(), name.namespace);
            next = base.def;
            bases.push(base);
        }

        let mut methods = Vec::new();

        for def in bases
            .iter()
            .rev()
            .map(|name| name.def)
            .chain(std::iter::once(name.def))
        {
            for method in def.methods() {
                let count = count.entry(method.name()).or_insert(0);
                *count += 1;

                let mut signature = Signature::new(&method, &[], &name.namespace);

                // Many years ago, the Visual C++ compiler engineers decided to diverge from the stdcall calling convention
                // for virtual functions returning UDTs. These return values must actually be returned via a hidden output
                // pointer. This is now part of the COM ABI and other compilers must follow along to ensure the ABI is
                // correctly preserved.

                if let Some(t) = &signature.return_type {
                    if let TypeKind::Struct(_) = t.kind {
                        let mut param = t.clone();
                        param.pointers += 1;
                        signature.params.insert(0, param);
                        signature.return_type = None;
                    }
                }

                methods.push(Method {
                    signature,
                    overload: *count,
                });
            }
        }

        Self {
            name,
            methods,
            bases,
        }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();
        let abi_name = self.name.gen_abi_definition();
        let guid = TypeGuid::from_type_def(&self.name.def);
        let guid = self.name.gen_guid(&guid);

        // TODO: here we're looking up the param name (from the file) repeatedly - cache name in Type

        let methods = self.methods.iter().enumerate().map(|(vtable_offset, method)| {
            let return_type = if let Some(t) = &method.signature.return_type {
                let tokens = t.gen_field();
                quote! { -> #tokens }
            } else {
                TokenStream::new()
            };

            let constraints = gen_constraint(method);
            let params = gen_params(method);
            let args = gen_abi_args(method);
            let name = method.gen_name();
            let vtable_offset = Literal::u32_unsuffixed((vtable_offset + 3) as u32);

            quote! {
                pub unsafe fn #name<#constraints>(&self, #params) #return_type {
                    (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #args)
                }
            }
        });

        let abi_methods = self.methods.iter().map(|method| {
            let return_type = if let Some(t) = &method.signature.return_type {
                let tokens = t.gen_field();
                quote! { -> #tokens }
            } else {
                TokenStream::new()
            };

            let params = method.signature.params.iter().map(|param| {
                let name = format_ident(&param.name);
                match &param.kind {
                    TypeKind::IUnknown | TypeKind::Interface(_)
                        if param.is_input && !param.is_array =>
                    {
                        quote! { #name: ::windows::RawPtr }
                    }
                    _ => {
                        let tokens = param.gen_field();
                        quote! { #name: #tokens }
                    }
                }
            });

            quote! {
                pub unsafe extern "system" fn (this: ::windows::RawPtr, #(#params),*) #return_type
            }
        });

        let mut conversions = TokenStream::new();

        conversions.combine(&quote! {
            impl ::std::convert::From<#name> for ::windows::IUnknown {
                fn from(value: #name) -> Self {
                    unsafe { ::std::mem::transmute(value) }
                }
            }
            impl ::std::convert::From<&#name> for ::windows::IUnknown {
                fn from(value: &#name) -> Self {
                    ::std::convert::From::from(::std::clone::Clone::clone(value))
                }
            }
            impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for #name {
                fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
                }
            }
            impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a #name {
                fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(::std::clone::Clone::clone(self)))
                }
            }
        });

        for base in &self.bases {
            let into = base.gen();

            conversions.combine(&quote! {
                impl ::std::convert::From<#name> for #into {
                    fn from(value: #name) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&#name> for #into {
                    fn from(value: &#name) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::std::convert::Into<::windows::Param<'a, #into>> for #name {
                    fn into(self) -> ::windows::Param<'a, #into> {
                        ::windows::Param::Owned(::std::convert::Into::<#into>::into(self))
                    }
                }
                impl<'a> ::std::convert::Into<::windows::Param<'a, #into>> for &'a #name {
                    fn into(self) -> ::windows::Param<'a, #into> {
                        ::windows::Param::Owned(::std::convert::Into::<#into>::into(::std::clone::Clone::clone(self)))
                    }
                }
            });
        }

        quote! {
            #[repr(transparent)]
            #[allow(non_camel_case_types)]
            pub struct #name(::windows::IUnknown);
            impl ::std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl ::std::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }
            impl ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for #name {}
            unsafe impl ::windows::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::Guid = #guid;
            }
            #[repr(C)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                #(#abi_methods,)*
            );
            #[allow(non_snake_case)]
            impl #name {
                #(#methods)*
            }
            #conversions
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.methods
            .iter()
            .map(|method| method.signature.dependencies())
            .chain(self.bases.iter().map(|base| base.dependencies()))
            .flatten()
            .collect()
    }
}

fn gen_constraint(method: &Method) -> TokenStream {
    let mut tokens = Vec::new();

    for (position, param) in method.signature.params.iter().enumerate() {
        if !param.is_input || param.is_array {
            continue;
        }

        match &param.kind {
            TypeKind::IUnknown | TypeKind::Interface(_) => {
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

fn gen_params(method: &Method) -> TokenStream {
    TokenStream::from_iter(
        method
            .signature
            .params
            .iter()
            .enumerate()
            .map(|(position, param)| {
                let name = format_ident(&param.name);

                match &param.kind {
                    TypeKind::IUnknown | TypeKind::Interface(_)
                        if param.is_input && !param.is_array =>
                    {
                        let type_tokens = squote::format_ident!("T{}__", position);
                        quote! { #name: #type_tokens, }
                    }
                    _ => {
                        let type_tokens = param.gen_field();
                        quote! { #name: #type_tokens, }
                    }
                }
            }),
    )
}

fn gen_abi_args(method: &Method) -> TokenStream {
    TokenStream::from_iter(method.signature.params.iter().map(|param| {
        let name = format_ident(&param.name);

        match &param.kind {
            TypeKind::IUnknown | TypeKind::Interface(_) if param.is_input && !param.is_array => {
                quote! { #name.into().abi(), }
            }
            _ => {
                quote! { #name, }
            }
        }
    }))
}
