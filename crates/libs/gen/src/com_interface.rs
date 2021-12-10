use super::*;

pub fn gen_com_interface(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    let name = gen_type_name(def, gen);
    let guid = gen_type_guid(def, gen);

    if include == TypeInclude::Full {
        let abi_name = gen_abi_name(def, gen);

        let (bases, inspectable) = def.base_interfaces();

        let abi_signatures = bases.iter().rev().chain(core::iter::once(def)).flat_map(|def| def.methods()).map(|method| {
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

        let (method_bases, dispatch) = if !bases.is_empty() && bases[0].type_name() == TypeName::IDispatch { (bases.iter().skip(1), true) } else { (bases.iter().skip(0), false) };

        let base_offset = if inspectable {
            3
        } else if dispatch {
            4
        } else {
            0
        };

        let methods = method_bases.rev().chain(core::iter::once(def)).flat_map(|def| def.methods()).enumerate().map(|(vtable_offset, method)| gen_method(base_offset + vtable_offset, &method, &mut method_names, gen));

        let mut conversions = gen_unknown(&name);

        for base in &bases {
            let into = gen_type_name(base, gen);
            let mut features = BTreeSet::new();
            features.insert(base.namespace());
            let cfg = gen.gen_cfg(&features);

            conversions.combine(&quote! {
                #cfg
                impl ::core::convert::From<#name> for #into {
                    fn from(value: #name) -> Self {
                        unsafe { ::core::mem::transmute(value) }
                    }
                }
                #cfg
                impl ::core::convert::From<&#name> for #into {
                    fn from(value: &#name) -> Self {
                        ::core::convert::From::from(::core::clone::Clone::clone(value))
                    }
                }
                #cfg
                impl<'a> ::windows::core::IntoParam<'a, #into> for #name {
                    fn into_param(self) -> ::windows::core::Param<'a, #into> {
                        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
                    }
                }
                #cfg
                impl<'a> ::windows::core::IntoParam<'a, #into> for &#name {
                    fn into_param(self) -> ::windows::core::Param<'a, #into> {
                        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
                    }
                }
            });
        }

        let send_sync = if def.type_name() == TypeName::IRestrictedErrorInfo {
            quote! {
                unsafe impl ::core::marker::Send for #name {}
                unsafe impl ::core::marker::Sync for #name {}
            }
        } else {
            quote! {}
        };

        let inspectable_vfptrs = if inspectable {
            quote! {
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
            }
        } else {
            quote! {}
        };

        let doc = gen.gen_cfg_doc(&BTreeSet::new());

        quote! {
            #doc
            #[repr(transparent)]
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug)]
            pub struct #name(pub ::windows::core::IUnknown);
            impl #name {
                #(#methods)*
            }
            unsafe impl ::windows::core::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::core::GUID = #guid;
            }
            #conversions
            #send_sync
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
                #inspectable_vfptrs
                #(#abi_signatures)*
            );
        }
    } else {
        quote! {
            #[repr(transparent)]
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug)]
            #[doc(hidden)]
            pub struct #name(pub ::windows::core::IUnknown);
            unsafe impl ::windows::core::Interface for #name {
                type Vtable = <::windows::core::IUnknown as ::windows::core::Interface>::Vtable;
                const IID: ::windows::core::GUID = #guid;
            }
        }
    }
}

fn gen_method(vtable_offset: usize, method: &MethodDef, method_names: &mut BTreeMap<String, u32>, gen: &Gen) -> TokenStream {
    let signature = method.signature(&[]);
    let constraints = gen_method_constraints(&signature.params, gen);
    let vtable_offset = Literal::usize_unsuffixed(vtable_offset + 3);

    let name = method.rust_name();
    let overload = method_names.entry(name.to_string()).or_insert(0);
    *overload += 1;
    let name: TokenStream = if *overload > 1 { format_token!("{}{}", name, overload) } else { to_ident(&name) };

    let features = signature.method_features();
    let cfg = gen.gen_cfg(&features);
    let doc = gen.gen_cfg_doc(&features);

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params) -> ::windows::core::Result<T> {
                    let mut result__ = ::core::option::Option::None;
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);
            let return_type_tokens = gen_win32_result_type(&signature, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    let mut result__: <#return_type_tokens as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &mut result__)
                    .from_abi::<#return_type_tokens>(result__ )
                }
            }
        }
        SignatureKind::ResultVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*).ok()
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_abi_type_name(&signature.return_sig.unwrap().kind, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints>(&self, #params) -> #return_sig {
                    let mut result__: #return_sig = ::core::default::Default::default();
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), &mut result__ #(,#args)*);
                    result__
                }
            }
        }
        SignatureKind::PreserveSig | SignatureKind::ReturnVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_win32_return_sig(&signature, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints>(&self, #params) #return_sig {
                    ::core::mem::transmute((::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*))
                }
            }
        }
    }
}
