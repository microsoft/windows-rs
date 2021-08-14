use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ComInterface(pub TypeDef);

impl ComInterface {
    pub fn gen(&self, gen: &Gen, include: TypeInclude) -> TokenStream {
        let name = gen_type_name(&self.0, gen);
        let guid =  gen_type_guid(&self.0, gen);

        if include == TypeInclude::Full {
            let abi_name = gen_abi_name(&self.0, gen);

            let (bases, inspectable) = self.0.base_interfaces();

            let abi_signatures = bases
                .iter()
                .rev()
                .chain(std::iter::once(&self.0))
                .map(|def| def.methods())
                .flatten()
                .map(|method| gen_win32_abi(&method.signature(&[]), gen));

            let mut method_names = BTreeMap::<String, u32>::new();

            let base_offset = if inspectable { 3 } else { 0 };

            let methods = bases
                .iter()
                .rev()
                .chain(std::iter::once(&self.0))
                .map(|def| def.methods())
                .flatten()
                .enumerate()
                .map(|(vtable_offset, method)| {
                    gen_method(base_offset + vtable_offset, &method, &mut method_names, gen)
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
                    impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for #name {
                        fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                            ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
                        }
                    }
                    impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a #name {
                        fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                            ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(::std::clone::Clone::clone(self)))
                        }
                    }
                });

            for base in &bases {
                let into = gen_type_name(base, gen);

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
                        impl<'a> ::windows::IntoParam<'a, #into> for #name {
                            fn into_param(self) -> ::windows::Param<'a, #into> {
                                ::windows::Param::Owned(::std::convert::Into::<#into>::into(self))
                            }
                        }
                        impl<'a> ::windows::IntoParam<'a, #into> for &'a #name {
                            fn into_param(self) -> ::windows::Param<'a, #into> {
                                ::windows::Param::Owned(::std::convert::Into::<#into>::into(::std::clone::Clone::clone(self)))
                            }
                        }
                    });
            }

            let send_sync = if self.0.type_name() == TypeName::IRestrictedErrorInfo {
                quote! {
                    unsafe impl ::std::marker::Send for #name {}
                    unsafe impl ::std::marker::Sync for #name {}
                }
            } else {
                quote! {}
            };

            let inspectable_vfptrs = if inspectable {
                quote! {
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, count: *mut u32, values: *mut *mut ::windows::Guid) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut ::windows::RawPtr) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut i32) -> ::windows::HRESULT,
                }
            } else {
                quote! {}
            };

            quote! {
                #[repr(transparent)]
                #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
                pub struct #name(::windows::IUnknown);
                impl #name {
                    #(#methods)*
                }
                unsafe impl ::windows::Interface for #name {
                    type Vtable = #abi_name;
                    const IID: ::windows::Guid = #guid;
                }
                #conversions
                #send_sync
                #[repr(C)]
                #[doc(hidden)]
                pub struct #abi_name(
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    #inspectable_vfptrs
                    #(pub unsafe extern "system" fn #abi_signatures,)*
                );
            }
        } else {
            quote! {
                #[repr(transparent)]
                #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
                #[doc(hidden)]
                pub struct #name(::windows::IUnknown);
                unsafe impl ::windows::Interface for #name {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = #guid;
                }
            }
        }
    }
}

fn gen_method(
    vtable_offset: usize,
    method: &MethodDef,
    method_names: &mut BTreeMap<String, u32>,
    gen: &Gen,
) -> TokenStream {
    let signature = method.signature(&[]);
    let constraints = gen_method_constraints(&signature.params);
    let vtable_offset = Literal::usize_unsuffixed(vtable_offset + 3);

    let name = method.name();
    let overload = method_names.entry(name.to_string()).or_insert(0);
    *overload += 1;

    let name = if *overload > 1 {
        format_ident!("{}{}", name, overload)
    } else {
        to_ident(name)
    };

    if signature.has_query_interface() {
        let leading_params = &signature.params[..signature.params.len() - 2];
        let params = gen_win32_params(leading_params, gen);
        let args = leading_params.iter().map(|p| gen_win32_abi_arg(p));

        quote! {
            pub unsafe fn #name<#constraints T: ::windows::Interface>(&self, #params) -> ::windows::Result<T> {
                let mut result__ = ::std::option::Option::None;
                (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #(#args,)* &<T as ::windows::Interface>::IID, ::windows::Abi::set_abi(&mut result__)).and_some(result__)
            }
        }
    } else if signature.has_retval() {
        let leading_params = &signature.params[..signature.params.len() - 1];
        let params = gen_win32_params(leading_params, gen);
        let args = leading_params.iter().map(|p| gen_win32_abi_arg(p));

        let mut return_param = signature.params[signature.params.len() - 1].clone();

        let return_type_tokens = if return_param.signature.pointers > 1 {
            return_param.signature.pointers -= 1;
            gen_win32_param(&return_param, gen)
        } else {
            gen_name(&return_param.signature.kind, gen)
        };

        quote! {
            pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                let mut result__: <#return_type_tokens as ::windows::Abi>::Abi = ::std::mem::zeroed();
                (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #(#args,)* &mut result__)
                .from_abi::<#return_type_tokens>(result__ )
            }
        }
    } else if signature.has_udt_return() {
        let params = gen_win32_params(&signature.params, gen);
        let args = signature.params.iter().map(|p| gen_win32_abi_arg(p));
        let return_type = gen_abi_type_name(&signature.return_type.unwrap().kind, gen);

        quote! {
            pub unsafe fn #name<#constraints>(&self, #params) -> #return_type {
                let mut result__: #return_type = ::std::default::Default::default();
                (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #(#args,)* &mut result__);
                result__
            }
        }
    } else if let Some(return_type) = &signature.return_type {
        let params = gen_win32_params(&signature.params, gen);
        let args = signature.params.iter().map(|p| gen_win32_abi_arg(p));

        if return_type.kind == ElementType::HRESULT {
            quote! {
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::Result<()> {
                    (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #(#args,)*).ok()
                }
            }
        } else {
            let return_type = gen_win32_abi_sig(return_type, gen);

            quote! {
                pub unsafe fn #name<#constraints>(&self, #params) -> #return_type {
                    (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #(#args,)*)
                }
            }
        }
    } else {
        let params = gen_win32_params(&signature.params, gen);
        let args = signature.params.iter().map(|p| gen_win32_abi_arg(p));

        quote! {
            pub unsafe fn #name<#constraints>(&self, #params) {
                (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #(#args,)*)
            }
        }
    }
}
