use super::*;

pub fn gen_com_interface(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    let name = gen_type_name(def, gen);
    let guid = gen_type_guid(def, gen);

    if include == TypeInclude::Full {
        let abi_name = gen_abi_name(def, gen);

        let (bases, inspectable) = def.base_interfaces();

        let abi_signatures = bases
            .iter()
            .rev()
            .chain(std::iter::once(def))
            .map(|def| def.methods())
            .flatten()
            .map(|method| {
                let signature = method.signature(&[]);
                let abi = gen_win32_abi(&signature, gen);
                if gen.root.is_empty() {
                    quote! {
                        pub unsafe extern "system" fn #abi,
                    }
                } else {
                    let features = method_features(&signature, gen);
                    let not_features = not_method_features(&signature, gen);
                    if features.is_empty() {
                        quote! {
                            pub unsafe extern "system" fn #abi,
                        }
                    } else {
                        quote! {
                            #features
                            pub unsafe extern "system" fn #abi,
                            #not_features
                            usize,
                        }
                    }
                }
            });

        let mut method_names = BTreeMap::<String, u32>::new();

        let (method_bases, dispatch) =
            if !bases.is_empty() && bases[0].type_name() == TypeName::IDispatch {
                (bases.iter().skip(1), true)
            } else {
                (bases.iter().skip(0), false)
            };

        let base_offset = if inspectable {
            3
        } else if dispatch {
            4
        } else {
            0
        };

        let methods = method_bases
            .rev()
            .chain(std::iter::once(def))
            .map(|def| def.methods())
            .flatten()
            .enumerate()
            .map(|(vtable_offset, method)| {
                gen_method(base_offset + vtable_offset, &method, &mut method_names, gen)
            });

        let mut conversions = TokenStream::with_capacity();

        conversions.combine(&quote! {
                    impl ::std::convert::From<#name> for ::windows::runtime::IUnknown {
                        fn from(value: #name) -> Self {
                            unsafe { ::std::mem::transmute(value) }
                        }
                    }
                    impl ::std::convert::From<&#name> for ::windows::runtime::IUnknown {
                        fn from(value: &#name) -> Self {
                            ::std::convert::From::from(::std::clone::Clone::clone(value))
                        }
                    }
                    impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for #name {
                        fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                            ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
                        }
                    }
                    impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &#name {
                        fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                            ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
                        }
                    }
                });

        for base in &bases {
            let into = gen_type_name(base, gen);
            let mut features = BTreeSet::new();
            features.insert(base.namespace());
            let cfg = gen.gen_cfg(&features);

            conversions.combine(&quote! {
                        #cfg
                        impl ::std::convert::From<#name> for #into {
                            fn from(value: #name) -> Self {
                                unsafe { ::std::mem::transmute(value) }
                            }
                        }
                        #cfg
                        impl ::std::convert::From<&#name> for #into {
                            fn from(value: &#name) -> Self {
                                ::std::convert::From::from(::std::clone::Clone::clone(value))
                            }
                        }
                        #cfg
                        impl<'a> ::windows::runtime::IntoParam<'a, #into> for #name {
                            fn into_param(self) -> ::windows::runtime::Param<'a, #into> {
                                ::windows::runtime::Param::Owned(::std::convert::Into::<#into>::into(self))
                            }
                        }
                        #cfg
                        impl<'a> ::windows::runtime::IntoParam<'a, #into> for &#name {
                            fn into_param(self) -> ::windows::runtime::Param<'a, #into> {
                                ::windows::runtime::Param::Owned(::std::convert::Into::<#into>::into(::std::clone::Clone::clone(self)))
                            }
                        }
                    });
        }

        let send_sync = if def.type_name() == TypeName::IRestrictedErrorInfo {
            quote! {
                unsafe impl ::std::marker::Send for #name {}
                unsafe impl ::std::marker::Sync for #name {}
            }
        } else {
            quote! {}
        };

        let inspectable_vfptrs = if inspectable {
            quote! {
                pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
            }
        } else {
            quote! {}
        };

        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name(::windows::runtime::IUnknown);
            impl #name {
                #(#methods)*
            }
            unsafe impl ::windows::runtime::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::runtime::GUID = #guid;
            }
            #conversions
            #send_sync
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                #inspectable_vfptrs
                #(#abi_signatures)*
            );
        }
    } else {
        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            #[doc(hidden)]
            pub struct #name(::windows::runtime::IUnknown);
            unsafe impl ::windows::runtime::Interface for #name {
                type Vtable = <::windows::runtime::IUnknown as ::windows::runtime::Interface>::Vtable;
                const IID: ::windows::runtime::GUID = #guid;
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
    let constraints = gen_method_constraints(&signature.params, gen);
    let vtable_offset = Literal::usize_unsuffixed(vtable_offset + 3);

    let name = method.rust_name();
    let overload = method_names.entry(name.to_string()).or_insert(0);
    *overload += 1;

    let name: TokenStream = if *overload > 1 {
        format_token!("{}{}", name, overload)
    } else {
        to_ident(&name)
    };

    let features = method_features(&signature, gen);

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #features
                pub unsafe fn #name<#constraints T: ::windows::runtime::Interface>(&self, #params) -> ::windows::runtime::Result<T> {
                    let mut result__ = ::std::option::Option::None;
                    (::windows::runtime::Interface::vtable(self).#vtable_offset)(::std::mem::transmute_copy(self), #(#args,)* &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #features
                pub unsafe fn #name<#constraints T: ::windows::runtime::Interface>(&self, #params result__: *mut ::std::option::Option<T>) -> ::windows::runtime::Result<()> {
                    (::windows::runtime::Interface::vtable(self).#vtable_offset)(::std::mem::transmute_copy(self), #(#args,)* &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);
            let return_type_tokens = gen_win32_result_type(&signature, gen);

            quote! {
                #features
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::runtime::Result<#return_type_tokens> {
                    let mut result__: <#return_type_tokens as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::runtime::Interface::vtable(self).#vtable_offset)(::std::mem::transmute_copy(self), #(#args,)* &mut result__)
                    .from_abi::<#return_type_tokens>(result__ )
                }
            }
        }
        SignatureKind::ResultVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #features
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::runtime::Result<()> {
                    (::windows::runtime::Interface::vtable(self).#vtable_offset)(::std::mem::transmute_copy(self), #(#args,)*).ok()
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_abi_type_name(&signature.return_sig.unwrap().kind, gen);

            quote! {
                #features
                pub unsafe fn #name<#constraints>(&self, #params) -> #return_sig {
                    let mut result__: #return_sig = ::std::default::Default::default();
                    (::windows::runtime::Interface::vtable(self).#vtable_offset)(::std::mem::transmute_copy(self), &mut result__ #(,#args)*);
                    result__
                }
            }
        }
        SignatureKind::PreserveSig => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_win32_return_sig(&signature, gen);

            quote! {
                #features
                pub unsafe fn #name<#constraints>(&self, #params) #return_sig {
                    ::std::mem::transmute((::windows::runtime::Interface::vtable(self).#vtable_offset)(::std::mem::transmute_copy(self), #(#args,)*))
                }
            }
        }
    }
}
