use super::*;

// TODO: need to gate all types

pub fn gen_interface(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    let name = gen_type_name(def, gen);
    let struct_phantoms = gen_phantoms(def);
    let constraints = gen_constraints(def);
    let type_signature = gen_guid_signature(def, &format!("{{{:#?}}}", def.guid()));
    let guid = gen_type_guid(def, gen);

    if include == TypeInclude::Full {
        let abi_name = gen_abi_name(def, gen);
        let abi_phantoms = gen_phantoms(def);

        let abi_signatures = def.methods().map(|method| {
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

        let doc = if is_exclusive {
            quote! { #[doc(hidden)] }
        } else {
            gen.gen_cfg_doc(&BTreeSet::new())
        };

        // The exclusive interface may be a factory interface and then we still need a type to use
        // with the factory cache. And we don't know at this stage whether the interface is for
        // the class or its factory.
        let public_type = if is_exclusive {
            TokenStream::new()
        } else {
            let interfaces = interfaces(def);
            let methods = InterfaceInfo::gen_methods(&interfaces, gen);
            let (async_get, future) = gen_async(def, &interfaces, gen, &TokenStream::new());
            let inspectable = gen_inspectable(&name, &constraints, &TokenStream::new());
            let iterator = gen_iterator(def, &interfaces, gen);

            let send_sync = if def.async_kind() == AsyncKind::None {
                quote! {}
            } else {
                quote! {
                    unsafe impl<#constraints> ::core::marker::Send for #name {}
                    unsafe impl<#constraints> ::core::marker::Sync for #name {}
                }
            };

            let conversions = interfaces.iter().filter(|interface| interface.kind != InterfaceKind::Default).map(|interface| interface.gen_conversion(&name, &constraints, &BTreeSet::new(), gen));

            quote! {
                impl<#constraints> #name {
                    #methods
                    #async_get
                }
                unsafe impl<#constraints> ::windows::core::RuntimeType for #name {
                    const SIGNATURE: ::windows::core::ConstBuffer = #type_signature;
                }
                #future
                #inspectable
                #(#conversions)*
                #send_sync
                #iterator
            }
        };

        let derive = if is_exclusive {
            quote! {}
        } else {
            quote! { #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug)] }
        };

        quote! {
            #[repr(transparent)]
            #derive
            #doc
            pub struct #name(pub ::windows::core::IInspectable, #(#struct_phantoms,)*) where #constraints;
            unsafe impl<#constraints> ::windows::core::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::core::GUID = #guid;
            }
            #public_type
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
                #(#abi_signatures)*
                #(pub #abi_phantoms,)*
            ) where #constraints;
        }
    } else {
        quote! {
            #[repr(transparent)]
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug)]
            #[doc(hidden)]
            pub struct #name(pub ::windows::core::IInspectable, #(#struct_phantoms,)*) where #constraints;
            unsafe impl<#constraints> ::windows::core::Interface for #name {
                type Vtable = <::windows::core::IUnknown as ::windows::core::Interface>::Vtable;
                const IID: ::windows::core::GUID = #guid;
            }
            unsafe impl<#constraints> ::windows::core::RuntimeType for #name {
                const SIGNATURE: ::windows::core::ConstBuffer = #type_signature;
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

                    result.push(InterfaceInfo { def, kind: InterfaceKind::NonDefault, is_base: false, version });
                }
            }
        }
    }

    let mut result = vec![InterfaceInfo { def: def.clone(), kind: InterfaceKind::Default, is_base: false, version: def.version() }];

    add_interfaces(&mut result, def, false);
    InterfaceInfo::sort(&mut result);
    result
}
