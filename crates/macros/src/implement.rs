use super::*;

pub fn gen(
    attribute: proc_macro::TokenStream,
    original_type: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let impl_type = original_type.clone();

    let implements = syn::parse_macro_input!(attribute as ImplementMacro);
    let impl_type = syn::parse_macro_input!(impl_type as syn::ItemStruct);
    let impl_name = impl_type.ident.to_string();
    let impl_ident = format_ident!("{}", impl_name); // because squote doesn't know how to deal with syn::*
    let box_ident = format_ident!("{}_box", impl_name);

    // TODO: always implement a dedicate IInspectable vtable - if we know we're implementing a runtimeclass this should return the class name
    // otherwise an empty string. Having a dedicate IInspectable just avoids all the complexity around finding the "nearest" IInspectable 
    // vtable when multiple are implemented and having one for WinRT objects that don't implement anything, not as uncommon as it sounds.
    let mut tokens = TokenStream::new();
    let mut vtable_idents = vec![];
    let mut vtable_ordinals = vec![];
    let mut vtable_ctors = TokenStream::new();
    let mut shims = TokenStream::new();
    let mut queries = TokenStream::new();
    let reader = TypeReader::get();

    for (interface_count, implement) in implements.implement.iter().enumerate() {
        if let gen::ElementType::Interface(t) = reader.resolve_type(implement.0, implement.1) {
            vtable_ordinals.push(Literal::usize_unsuffixed(interface_count));

            let query_interface = format_ident!("QueryInterface_abi{}", interface_count);
            let add_ref = format_ident!("AddRef_abi{}", interface_count);
            let release = format_ident!("Release_abi{}", interface_count);

            let mut vtable_ptrs = quote! {
                Self::#query_interface,
                Self::#add_ref,
                Self::#release,
                Self::GetIids,
                Self::GetRuntimeClassName, // TODO: needs to be vtable specific unless implementing a class
                Self::GetTrustLevel,
            };

            shims.combine(&quote! {
                unsafe extern "system" fn #query_interface(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT {
                    let this = (this as *mut ::windows::RawPtr).sub(2 + #interface_count) as *mut Self;
                    (*this).QueryInterface(iid, interface)
                }
                unsafe extern "system" fn #add_ref(this: ::windows::RawPtr) -> u32 {
                    let this = (this as *mut ::windows::RawPtr).sub(2 + #interface_count) as *mut Self;
                    (*this).AddRef()
                }
                unsafe extern "system" fn #release(this: ::windows::RawPtr) -> u32 {
                    let this = (this as *mut ::windows::RawPtr).sub(2 + #interface_count) as *mut Self;
                    (*this).Release()
                }
            });

            let empty = gen::TypeTree::from_namespace("");
            let gen = gen::Gen::absolute(&empty);

            let vtable_ident = t.0.gen_abi_name(&gen);
            let interface_ident = t.0.gen_name(&gen);
            let interface_literal = Literal::usize_unsuffixed(interface_count);

            for (vtable_offset, method) in t.0.def.methods().enumerate() {
                let method_ident = gen::to_ident(&method.rust_name());
                let vcall_ident = format_ident!("abi{}_{}", interface_count, vtable_offset + 6);

                vtable_ptrs.combine(&quote! {
                    Self::#vcall_ident,
                });

                let signature = method.signature(&[]);
                let abi_signature = signature.gen_winrt_abi(&gen);
                let upcall = signature
                    .gen_winrt_upcall(quote! { (*this).implementation.#method_ident }, &gen);

                shims.combine(&quote! {
                    unsafe extern "system" fn #vcall_ident #abi_signature {
                        let this = (this as *mut ::windows::RawPtr).sub(2 + #interface_count) as *mut Self;
                        #upcall
                    }
                });

                queries.combine(&quote! {
                    &<#interface_ident as ::windows::Interface>::IID => {
                        &mut self.vtables.#interface_literal as *mut _ as _
                    }
                });
            }

            tokens.combine(&quote! {
                impl ::std::convert::From<#impl_ident> for #interface_ident {
                    fn from(implementation: #impl_ident) -> Self {
                        let com = #box_ident::new(implementation);

                        unsafe {
                            let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                            ::std::mem::transmute_copy(&::std::ptr::NonNull::new_unchecked(&mut (*ptr).vtables.#interface_literal as *mut _ as _))
                        }
                    }
                }
            });

            vtable_ctors.combine(&quote! {
                #vtable_ident(
                    #vtable_ptrs
                ),
            });

            vtable_idents.push(vtable_ident);
        }
    }

    tokens.combine(&quote! {
        impl ::std::convert::From<#impl_ident> for ::windows::IUnknown {
            fn from(implementation: #impl_ident) -> Self {
                let com = #box_ident::new(implementation);

                unsafe {
                    let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                    ::std::mem::transmute_copy(&::std::ptr::NonNull::new_unchecked(&mut (*ptr).identity_vtable as *mut _ as _))
                }
            }
        }
        impl ::std::convert::From<#impl_ident> for ::windows::IInspectable {
            fn from(implementation: #impl_ident) -> Self {
                let com = #box_ident::new(implementation);

                unsafe {
                    let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                    ::std::mem::transmute_copy(&::std::ptr::NonNull::new_unchecked(&mut (*ptr).identity_vtable as *mut _ as _))
                }
            }
        }
        #[repr(C)]
        struct #box_ident {
            base: ::std::option::Option<::windows::IInspectable>,
            identity_vtable: *const ::windows::IInspectable_abi,
            vtables: (#(*const #vtable_idents,)*),
            implementation: #impl_ident,
            count: ::windows::WeakRefCount,
        }
        impl #box_ident {
            const VTABLES: (#(#vtable_idents,)*) = (
                #vtable_ctors
            );
            const IDENTITY_VTABLE: ::windows::IInspectable_abi = ::windows::IInspectable_abi(
                Self::identity_query_interface,
                Self::identity_add_ref,
                Self::identity_release,
                Self::GetIids,
                Self::GetRuntimeClassName, // TODO: identity_type_name,
                Self::GetTrustLevel,
            );
            fn new(implementation: #impl_ident) -> Self {
                Self {
                    base: ::std::option::Option::None,
                    identity_vtable: &Self::IDENTITY_VTABLE,
                    vtables: (#(&Self::VTABLES.#vtable_ordinals,)*),
                    implementation,
                    count: ::windows::WeakRefCount::new(),
                }
            }
            fn QueryInterface(&mut self, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT {
                unsafe {
                    *interface = match iid {
                        #queries
                        &<::windows::IUnknown as ::windows::Interface>::IID
                        | &<::windows::IInspectable as ::windows::Interface>::IID
                        | &<::windows::IAgileObject as ::windows::Interface>::IID => {
                            &mut self.identity_vtable as *mut _ as _
                        }
                        _ => ::std::ptr::null_mut(),
                    };

                    if !(*interface).is_null() {
                        self.count.add_ref();
                        return ::windows::HRESULT(0);
                    }

                    *interface = self.count.query(iid, &mut self.identity_vtable as *mut _ as _);

                    if (*interface).is_null() {
                        if let Some(base) = &self.base {
                            ::windows::Interface::query(base, iid, interface)
                        } else {
                            ::windows::HRESULT(0x8000_4002) // E_NOINTERFACE
                        }
                    } else {
                        ::windows::HRESULT(0)
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
                _: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *count = 0;
                *values = ::std::ptr::null_mut();
                ::windows::HRESULT(0)
            }
            unsafe extern "system" fn GetRuntimeClassName(
                _: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT {
                // TODO: if a class is being implemented then the class name should be returned in all cases.
                // Otherwise, the interface name should be returned on a per-interface basis and the identity
                // implementation should return an empty string.

                let h: ::windows::HSTRING = "Thing".into(); // TODO: replace with class name or first interface
                *value = ::std::mem::transmute(h);
                ::windows::HRESULT(0)
            }
            unsafe extern "system" fn GetTrustLevel(_: ::windows::RawPtr, value: *mut i32) -> ::windows::HRESULT {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *value = 0;
                ::windows::HRESULT(0)
            }
            unsafe extern "system" fn identity_query_interface(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT {
                let this = (this as *mut ::windows::RawPtr).sub(1) as *mut Self;
                (*this).QueryInterface(iid, interface)
            }
            unsafe extern "system" fn identity_add_ref(this: ::windows::RawPtr) -> u32 {
                let this = (this as *mut ::windows::RawPtr).sub(1) as *mut Self;
                (*this).AddRef()
            }
            unsafe extern "system" fn identity_release(this: ::windows::RawPtr) -> u32 {
                let this = (this as *mut ::windows::RawPtr).sub(1) as *mut Self;
                (*this).Release()
            }
            #shims
        }
    });

    let mut tokens = tokens.parse::<proc_macro::TokenStream>().unwrap();
    tokens.extend(std::iter::once(original_type));
    tokens
}
