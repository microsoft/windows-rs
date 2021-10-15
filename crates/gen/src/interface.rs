use super::*;

pub fn gen_interface(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    let name = gen_type_name(def, gen);
    let struct_phantoms = gen_phantoms(def);
    let constraints = gen_constraints(def);
    let type_signature = gen_guid_signature(def, &format!("{{{:#?}}}", def.guid()));
    let guid = gen_type_guid(def, gen);

    if include == TypeInclude::Full {
        let abi_name = gen_abi_name(def, gen);
        let abi_phantoms = gen_phantoms(def);

        let abi_signatures = def
            .methods()
            .map(|method| {
                let signature = method.signature(&def.generics);
                let features = method_features(&signature, gen);
                let not_features = not_method_features(&signature, gen);
                let signature = gen_winrt_abi(&signature, gen);

                if features.is_empty() {
                    quote! {
                        pub unsafe extern "system" fn #signature,
                    }
                } else {
                    quote! {
                        #features
                        pub unsafe extern "system" fn #signature,
                        #not_features
                        usize,

                    }
                }
            });

        let is_exclusive = def.is_exclusive();

        let hidden = if is_exclusive {
            quote! { #[doc(hidden)] }
        } else {
            quote! {}
        };

        // The exclusive interface may be a factory interface and then we still need a type to use
        // with the factory cache. And we don't know at this stage whether the interface is for
        // the class or its factory.
        let public_type = if is_exclusive {
            TokenStream::new()
        } else {
            let interfaces = interfaces(def);
            let methods = InterfaceInfo::gen_methods(&interfaces, gen);
            let (async_get, future) = gen_async(def, &interfaces, gen);
            let object = gen_object(&name, &constraints);
            let iterator = gen_iterator(def, &interfaces, gen);

            let send_sync = if async_kind(def) == AsyncKind::None {
                quote! {}
            } else {
                quote! {
                    unsafe impl<#constraints> ::std::marker::Send for #name {}
                    unsafe impl<#constraints> ::std::marker::Sync for #name {}
                }
            };

            let conversions = interfaces
                .iter()
                .filter(|interface| interface.kind != InterfaceKind::Default)
                .map(|interface| interface.gen_conversion(&name, &constraints, gen));

            quote! {
                impl<#constraints> #name {
                    #methods
                    #async_get
                }
                unsafe impl<#constraints> ::windows::RuntimeType for #name {
                    const SIGNATURE: ::windows::ConstBuffer = #type_signature;
                }
                #future
                #object
                #(#conversions)*
                #send_sync
                #iterator
            }
        };

        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            #hidden
            pub struct #name(::windows::IInspectable, #(#struct_phantoms,)*) where #constraints;
            unsafe impl<#constraints> ::windows::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::Guid = #guid;
            }
            #public_type
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, count: *mut u32, values: *mut *mut ::windows::Guid) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr, value: *mut i32) -> ::windows::HRESULT,
                #(#abi_signatures)*
                #(pub #abi_phantoms,)*
            ) where #constraints;
        }
    } else {
        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            #[doc(hidden)]
            pub struct #name(::windows::IInspectable, #(#struct_phantoms,)*) where #constraints;
            unsafe impl<#constraints> ::windows::Interface for #name {
                type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                const IID: ::windows::Guid = #guid;
            }
            unsafe impl<#constraints> ::windows::RuntimeType for #name {
                const SIGNATURE: ::windows::ConstBuffer = #type_signature;
            }
        }
    }
}

fn interfaces(def: &TypeDef) -> Vec<InterfaceInfo> {
    fn add_interfaces(result: &mut Vec<InterfaceInfo>, parent: &TypeDef, is_base: bool) {
        for child in parent.interface_impls() {
            if let ElementType::TypeDef(def) = child.generic_interface(&parent.generics) {
                if !result.iter().any(|info| info.def == def) {
                    add_interfaces(result, &def, is_base);
                    let version = def.version();

                    result.push(InterfaceInfo {
                        def,
                        kind: InterfaceKind::NonDefault,
                        is_base: false,
                        version,
                    });
                }
            }
        }
    }

    let mut result = vec![InterfaceInfo {
        def: def.clone(),
        kind: InterfaceKind::Default,
        is_base: false,
        version: def.version(),
    }];

    add_interfaces(&mut result, def, false);
    InterfaceInfo::sort(&mut result);
    result
}
