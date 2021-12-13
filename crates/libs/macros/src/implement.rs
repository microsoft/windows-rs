use super::*;

pub fn gen(attribute: proc_macro::TokenStream, original_type: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let impl_type = original_type.clone();

    let implements = syn::parse_macro_input!(attribute as ImplementMacro);
    let impl_type = syn::parse_macro_input!(impl_type as syn::ItemStruct);
    let impl_name = impl_type.ident.to_string();
    let interfaces = implements.interfaces();

    let mut tokens = TokenStream::new();
    let mut vtable_idents = vec![];
    let mut vtable_ordinals = vec![];
    let mut vtable_ctors = TokenStream::new();
    let mut shims = TokenStream::new();
    let mut queries = TokenStream::new();
    let mut query_constants = TokenStream::new();
    let gen = gen::Gen::absolute();

    let mut generics = BTreeSet::new();

    for (def, _) in &interfaces {
        for generic in &def.generics {
            if let ElementType::GenericParam(generic) = generic {
                generics.insert(generic);
            }
        }
    }

    let generics: Vec<TokenStream> = generics.iter().map(|generic| (*generic).into()).collect();

    let impl_ident = format_token!("{}", impl_name);
    let impl_ident = quote! { #impl_ident::<#(#generics,)*> };
    let box_ident = format_token!("{}_box", impl_name);

    let constraints = quote! {
        #(#generics: ::windows::core::RuntimeType + 'static,)*
    };

    let interfaces_len = Literal::usize_unsuffixed(interfaces.len());

    let mut abi_count = 0;

    for (interface_count, (def, overrides)) in interfaces.iter().enumerate() {
        let is_winrt = def.is_winrt();
        vtable_ordinals.push(Literal::usize_unsuffixed(interface_count));

        let query_interface = format_token!("QueryInterface_abi{}", interface_count);
        let add_ref = format_token!("AddRef_abi{}", interface_count);
        let release = format_token!("Release_abi{}", interface_count);

        let (base_interfaces, is_inspectable) = if is_winrt { (Vec::new(), true) } else { def.base_interfaces() };

        let mut vtable_ptrs = if is_inspectable {
            quote! {
                Self::#query_interface,
                Self::#add_ref,
                Self::#release,
                Self::GetIids,
                Self::GetRuntimeClassName, // TODO: needs to be vtable specific unless implementing a class
                Self::GetTrustLevel,
            }
        } else {
            quote! {
                Self::#query_interface,
                Self::#add_ref,
                Self::#release,
            }
        };

        shims.combine(&quote! {
            unsafe extern "system" fn #query_interface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
                let this = (this as *mut ::windows::core::RawPtr).sub(2 + #interface_count) as *mut Self;
                (*this).QueryInterface(iid, interface)
            }
            unsafe extern "system" fn #add_ref(this: ::windows::core::RawPtr) -> u32 {
                let this = (this as *mut ::windows::core::RawPtr).sub(2 + #interface_count) as *mut Self;
                (*this).AddRef()
            }
            unsafe extern "system" fn #release(this: ::windows::core::RawPtr) -> u32 {
                let this = (this as *mut ::windows::core::RawPtr).sub(2 + #interface_count) as *mut Self;
                (*this).Release()
            }
        });

        let vtable_ident = gen_abi_name(def, &gen);
        let interface_ident = gen_type_name(def, &gen);
        let interface_literal = Literal::usize_unsuffixed(interface_count);
        let interface_constant = format_token!("IID{}", interface_count);

        queries.combine(&quote! {
            else if iid == &Self::#interface_constant {
                &mut self.vtables.#interface_literal as *mut _ as _
            }
        });

        for base in &base_interfaces {
            let interface_ident = gen_type_name(base, &gen);

            queries.combine(&quote! {
                else if iid == &<#interface_ident as ::windows::core::Interface>::IID {
                    &mut self.vtables.#interface_literal as *mut _ as _
                }
            });
        }

        // Constants are required for generic interfaces due to limitations in Rust.
        query_constants.combine(&quote! {
            const #interface_constant: ::windows::core::GUID = <#interface_ident as ::windows::core::Interface>::IID;
        });

        for method in base_interfaces.iter().rev().chain(core::iter::once(def)).flat_map(|def| def.methods()) {
            let method_ident = gen::to_ident(&method.rust_name());

            abi_count += 1;
            let vcall_ident = format_token!("abi{}", abi_count);

            vtable_ptrs.combine(&quote! {
                Self::#vcall_ident,
            });

            let signature = method.signature(&def.generics);

            let abi_signature = if is_winrt { gen_winrt_abi(&signature, &gen) } else { gen_win32_abi(&signature, &gen) };

            let upcall = if is_winrt {
                if *overrides {
                    if implements.overrides.contains(method.name()) {
                        gen_winrt_upcall(&signature, quote! { (*this).implementation.#method_ident }, &gen)
                    } else {
                        quote! { ::windows::core::HRESULT(0) }
                    }
                } else {
                    gen_winrt_upcall(&signature, quote! { (*this).implementation.#method_ident }, &gen)
                }
            } else {
                gen_win32_upcall(&signature, quote! { (*this).implementation.#method_ident }, &gen)
            };

            shims.combine(&quote! {
                unsafe extern "system" fn #vcall_ident #abi_signature {
                    let this = (this as *mut ::windows::core::RawPtr).sub(2 + #interface_count) as *mut Self;
                    #upcall
                }
            });
        }

        if !def.is_exclusive() {
            tokens.combine(&quote! {
                impl <#constraints> ::core::convert::From<#impl_ident> for #interface_ident {
                    fn from(implementation: #impl_ident) -> Self {
                        let com = #box_ident::<#(#generics,)*>::new(implementation);

                        unsafe {
                            let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                            ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(&mut (*ptr).vtables.#interface_literal as *mut _ as _))
                        }
                    }
                }
                impl <#constraints> ::core::convert::From<&mut #impl_ident> for #interface_ident {
                    fn from(implementation: &mut #impl_ident) -> Self {
                        unsafe {
                            let mut ptr = (implementation as *mut _ as *mut ::windows::core::RawPtr).sub(2 + #interfaces_len) as *mut #box_ident::<#(#generics,)*>;
                            (*ptr).count.add_ref();
                            ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(&mut (*ptr).vtables.#interface_literal as *mut _ as _))
                        }
                    }
                }
                impl<#constraints> ::windows::core::ToImpl<#interface_ident> for #impl_ident {
                    unsafe fn to_impl(interface: &#interface_ident) -> &mut Self {
                        let this: ::windows::core::RawPtr = ::core::mem::transmute_copy(interface);
                        let this = (this as *mut ::windows::core::RawPtr).sub(2 + #interface_count) as *mut #box_ident::<#(#generics,)*>;
                        &mut (*this).implementation
                    }
                }
            });
        }

        let mut phantoms = TokenStream::new();

        for _ in 0..def.generic_params().count() {
            phantoms.combine(&quote! { ::core::marker::PhantomData, })
        }

        vtable_ctors.combine(&quote! {
            #vtable_ident(
                #vtable_ptrs
                #phantoms
            ),
        });

        vtable_idents.push(vtable_ident);
    }

    let constructors = if let Some(extend) = implements.extend {
        let mut factories = Vec::new();

        for attribute in extend.attributes() {
            if attribute.name() == "ComposableAttribute" {
                if let Some(def) = attribute.composable_type() {
                    factories.push(InterfaceInfo { def, kind: InterfaceKind::Extend, is_base: false, version: (0, 0) });
                }
            }
        }

        InterfaceInfo::gen_methods(&factories, &gen)
    } else {
        quote! {}
    };

    tokens.combine(&quote! {
        impl <#constraints> #impl_ident {
            #constructors
            fn cast<ResultType: ::windows::core::Interface>(&self) -> ::windows::core::Result<ResultType> {
                unsafe {
                    let boxed = (self as *const #impl_ident as *mut #impl_ident as *mut ::windows::core::RawPtr).sub(2 + #interfaces_len) as *mut #box_ident::<#(#generics,)*>;
                    let mut result = None;
                    (*boxed).QueryInterface(&ResultType::IID, &mut result as *mut _ as _).and_some(result)
                }
            }
        }
        impl <#constraints> ::core::convert::From<#impl_ident> for ::windows::core::IUnknown {
            fn from(implementation: #impl_ident) -> Self {
                let com = #box_ident::<#(#generics,)*>::new(implementation);

                unsafe {
                    let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                    ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(&mut (*ptr).identity_vtable as *mut _ as _))
                }
            }
        }
        impl <#constraints> ::core::convert::From<#impl_ident> for ::windows::core::IInspectable {
            fn from(implementation: #impl_ident) -> Self {
                let com = #box_ident::<#(#generics,)*>::new(implementation);

                unsafe {
                    let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                    ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(&mut (*ptr).identity_vtable as *mut _ as _))
                }
            }
        }
        impl <#constraints> ::windows::core::Compose for #impl_ident {
            unsafe fn compose<'a>(implementation: Self) -> (::windows::core::IInspectable, &'a mut ::core::option::Option<::windows::core::IInspectable>) {
                let inspectable: ::windows::core::IInspectable = implementation.into();
                let this = (&inspectable as *const _ as *mut ::windows::core::RawPtr).sub(1) as *mut #box_ident::<#(#generics,)*>;
                (inspectable, &mut (*this).base)
            }
        }
        #[repr(C)]
        struct #box_ident<#(#generics,)*> where #constraints {
            base: ::core::option::Option<::windows::core::IInspectable>,
            identity_vtable: *const ::windows::core::IInspectableVtbl,
            vtables: (#(*const #vtable_idents,)*),
            implementation: #impl_ident,
            count: ::windows::core::WeakRefCount,
        }
        impl <#constraints> #box_ident::<#(#generics,)*> {
            const VTABLES: (#(#vtable_idents,)*) = (
                #vtable_ctors
            );
            const IDENTITY_VTABLE: ::windows::core::IInspectableVtbl = ::windows::core::IInspectableVtbl(
                Self::identity_query_interface,
                Self::identity_add_ref,
                Self::identity_release,
                Self::GetIids,
                Self::GetRuntimeClassName,
                Self::GetTrustLevel,
            );
            #query_constants
            fn new(implementation: #impl_ident) -> Self {
                Self {
                    base: ::core::option::Option::None,
                    identity_vtable: &Self::IDENTITY_VTABLE,
                    vtables: (#(&Self::VTABLES.#vtable_ordinals,)*),
                    implementation,
                    count: ::windows::core::WeakRefCount::new(),
                }
            }
            fn QueryInterface(&mut self, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
                unsafe {
                    *interface = if iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
                        || iid == &<::windows::core::IInspectable as ::windows::core::Interface>::IID
                        || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
                            &mut self.identity_vtable as *mut _ as _
                    } #queries else {
                        ::core::ptr::null_mut()
                    };

                    if !(*interface).is_null() {
                        self.count.add_ref();
                        return ::windows::core::HRESULT(0);
                    }

                    *interface = self.count.query(iid, &mut self.identity_vtable as *mut _ as _);

                    if (*interface).is_null() {
                        if let Some(base) = &self.base {
                            ::windows::core::Interface::query(base, iid, interface)
                        } else {
                            ::windows::core::HRESULT(0x8000_4002) // E_NOINTERFACE
                        }
                    } else {
                        ::windows::core::HRESULT(0)
                    }
                }
            }
            fn AddRef(&mut self) -> u32 {
                self.count.add_ref()
            }
            fn Release(&mut self) -> u32 {
                let remaining = self.count.release();
                if remaining == 0 {
                    unsafe {
                        ::std::boxed::Box::from_raw(self);
                    }
                }
                remaining
            }
            unsafe extern "system" fn GetIids(
                _: ::windows::core::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::core::GUID,
            ) -> ::windows::core::HRESULT {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *count = 0;
                *values = ::core::ptr::null_mut();
                ::windows::core::HRESULT(0)
            }
            unsafe extern "system" fn GetRuntimeClassName(
                _: ::windows::core::RawPtr,
                value: *mut ::windows::core::RawPtr,
            ) -> ::windows::core::HRESULT {
                // TODO: if a class is being implemented then the class name should be returned in all cases.
                // Otherwise, the interface name should be returned on a per-interface basis and the identity
                // implementation should return an empty string.

                let h = ::windows::core::HSTRING::new();
                *value = ::core::mem::transmute(h);
                ::windows::core::HRESULT(0)
            }
            unsafe extern "system" fn GetTrustLevel(_: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *value = 0;
                ::windows::core::HRESULT(0)
            }
            unsafe extern "system" fn identity_query_interface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
                let this = (this as *mut ::windows::core::RawPtr).sub(1) as *mut Self;
                (*this).QueryInterface(iid, interface)
            }
            unsafe extern "system" fn identity_add_ref(this: ::windows::core::RawPtr) -> u32 {
                let this = (this as *mut ::windows::core::RawPtr).sub(1) as *mut Self;
                (*this).AddRef()
            }
            unsafe extern "system" fn identity_release(this: ::windows::core::RawPtr) -> u32 {
                let this = (this as *mut ::windows::core::RawPtr).sub(1) as *mut Self;
                (*this).Release()
            }
            #shims
        }
    });

    let mut tokens = tokens.parse::<proc_macro::TokenStream>().unwrap();
    tokens.extend(core::iter::once(original_type));
    tokens
}
