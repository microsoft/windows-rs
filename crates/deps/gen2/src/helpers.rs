use super::*;

pub fn gen_std_traits(def: &TypeDef, cfg: &TokenStream, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let phantoms = gen_phantoms(def, gen);

    quote! {
        #cfg
        impl<#(#constraints)*> ::core::clone::Clone for #name {
            fn clone(&self) -> Self {
                Self(self.0.clone(), #(#phantoms)*)
            }
        }
        #cfg
        impl<#(#constraints)*> ::core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        #cfg
        impl<#(#constraints)*> ::core::cmp::Eq for #name {}
    }
}

pub fn gen_interface_trait(def: &TypeDef, cfg: &TokenStream, gen: &Gen) -> TokenStream {
    if let Some(default) = def.default_interface() {
        let name = gen_type_ident(def, gen);
        let vtbl = gen_vtbl_ident(&default, gen);
        let guid = gen_type_guid(&default, gen, &"Self".into());
        let namespace = gen.namespace(default.namespace());
        
        quote! {
            #cfg
            unsafe impl ::windows::core::Interface for #name {
                type Vtable = #namespace #vtbl;
                const IID: ::windows::core::GUID = #guid;
            }
        }
    } else {
        let name = gen_type_ident(def, gen);
        let constraints = gen_type_constraints(def, gen);
        let vtbl = gen_vtbl_ident(def, gen);
        let guid = gen_type_guid(def, gen, &"Self".into());

        quote! {
            unsafe impl<#(#constraints)*> ::windows::core::Interface for #name {
                type Vtable = #vtbl;
                const IID: ::windows::core::GUID = #guid;
            }
        }
    }
}

pub fn gen_runtime_trait(def: &TypeDef, cfg: &TokenStream, gen: &Gen) -> TokenStream {
    if def.is_winrt() {
        let name = gen_type_ident(def, gen);
        let constraints = gen_type_constraints(def, gen);
        let type_signature = if def.kind() == TypeKind::Class {
            let type_signature = Literal::byte_string(def.type_signature().as_bytes());
            quote! { ::windows::core::ConstBuffer::from_slice(#type_signature) }
        } else { gen_guid_signature(def, &format!("{{{:#?}}}", def.guid()), gen) };

        quote! {
            #cfg
            unsafe impl<#(#constraints)*> ::windows::core::RuntimeType for #name {
                const SIGNATURE: ::windows::core::ConstBuffer = #type_signature;
            }
        }
    } else {
        quote! {}
    }
}

pub fn gen_vtbl(def: &TypeDef, gen: &Gen) -> TokenStream {
    // TODO: consider using parent field to avoid duplicating inherited vfptrs.
    // And then consider naming them to simplify traits and debugging.
    // Should the first param be the Vtbl type?

    let vtbl = gen_vtbl_ident(def, gen);
    let guid = gen_element_name(&ElementType::GUID, gen);
    let hresult = gen_element_name(&ElementType::HRESULT, gen);
    let is_winrt = def.is_winrt();
    let phantoms = gen_phantoms(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let mut methods = quote! {};

    for def in def.vtable_types() {
        match def {
            ElementType::TypeDef(def) => {
                for method in def.methods() {
                    if method.name() == ".ctor" {
                        continue;
                    }

                    let signature = method.signature(&def.generics);
                    let (feature_cfg, not_feature_cfg) = gen.method_cfg(&def, &method);

                    let (trailing_return_type, return_type) = if is_winrt {
                        if let Some(return_sig) = &signature.return_sig {
                            let tokens = gen_abi_sig(return_sig, gen);
                            if return_sig.is_array {
                                (quote! { result_size__: *mut u32, result__: *mut *mut #tokens }, quote! { -> #hresult })
                            } else {
                                (quote! { *mut #tokens }, quote! { -> #hresult })
                            }
                        } else {
                            (quote! {}, quote! { -> #hresult })
                        }
                    } else {
                        if let Some(return_sig) = &signature.return_sig {
                            if return_sig.is_udt() {
                                let tokens = gen_abi_sig(return_sig, gen);
                                (quote! { *mut #tokens }, quote! {})
                            } else {
                                let tokens = gen_sig(return_sig, gen);
                                (quote! {}, quote! { -> #tokens })
                            }
                        } else {
                            (quote! {}, quote! {})
                        }
                    };

                    let params = signature.params.iter().map(|p| {
                        let name = gen_param_name(&p.param);
                        if is_winrt {
                            let signature = &p.signature;
                            let abi = gen_abi_sig(signature, gen);

                            if signature.is_array {
                                let abi_size_name = gen_ident(&format!("{}_array_size", p.param.name()));

                                if p.param.is_input() {
                                    quote! { #abi_size_name: u32, #name: *const #abi, }
                                } else if p.signature.by_ref {
                                    quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi, }
                                } else {
                                    quote! { #abi_size_name: u32, #name: *mut #abi, }
                                }
                            } else if p.param.is_input() {
                                if p.signature.is_const {
                                    quote! { #name: &#abi, }
                                } else {
                                    quote! { #name: #abi, }
                                }
                            } else {
                                quote! { #name: *mut #abi, }
                            }
                        } else {
                            let abi = gen_abi_param_sig(p, gen);
                            quote! { #name: #abi, }
                        }
                    });

                    let signature = quote! { pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, #(#params)* #trailing_return_type) #return_type, };

                    if feature_cfg.is_empty() {
                        methods.combine(&signature);
                    } else {
                        methods.combine(&quote! {
                            #feature_cfg
                            #signature
                            #not_feature_cfg
                            usize,
                        });
                    }
                }
            }
            ElementType::IInspectable => methods.combine(&quote! {
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut #guid) -> #hresult,
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> #hresult,
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> #hresult,
            }),
            ElementType::IUnknown => methods.combine(&quote! {
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &#guid, interface: *mut *mut ::core::ffi::c_void) -> #hresult,
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
            }),
            _ => unimplemented!(),
        }
    }

    quote! {
        #[repr(C)] #[doc(hidden)] pub struct #vtbl (
            #methods
            #(pub #phantoms)*
        ) where #(#constraints)*;
    }
}

fn gen_type_guid(def: &TypeDef, gen: &Gen, type_name: &TokenStream) -> TokenStream {
    if def.generics.is_empty() {
        match GUID::from_attributes(def.attributes()) {
            Some(guid) => gen_guid(&guid, gen),
            None => {
                quote! {
                    ::windows::core::GUID::zeroed()
                }
            }
        }
    } else {
        quote! {
            ::windows::core::GUID::from_signature(<#type_name as ::windows::core::RuntimeType>::SIGNATURE)
        }
    }
}

pub fn gen_constant_type_value(value: &ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { bool = #value },
        ConstantValue::U8(value) => quote! { u8 = #value },
        ConstantValue::I8(value) => quote! { i8 = #value },
        ConstantValue::U16(value) => quote! { u16 = #value },
        ConstantValue::I16(value) => quote! { i16 = #value },
        ConstantValue::U32(value) => quote! { u32 = #value },
        ConstantValue::I32(value) => quote! { i32 = #value },
        ConstantValue::U64(value) => quote! { u64 = #value },
        ConstantValue::I64(value) => quote! { i64 = #value },
        ConstantValue::F32(value) => quote! { f32 = #value },
        ConstantValue::F64(value) => quote! { f64 = #value },
        ConstantValue::String(value) => quote! { &'static str = #value },
        _ => unimplemented!(),
    }
}

pub fn gen_guid(value: &GUID, gen: &Gen) -> TokenStream {
    let guid = gen_element_name(&ElementType::GUID, gen);

    if gen.sys {
        let a = Literal::u32_unsuffixed(value.0);
        let b = Literal::u16_unsuffixed(value.1);
        let c = Literal::u16_unsuffixed(value.2);
        let d = Literal::u8_unsuffixed(value.3);
        let e = Literal::u8_unsuffixed(value.4);
        let f = Literal::u8_unsuffixed(value.5);
        let g = Literal::u8_unsuffixed(value.6);
        let h = Literal::u8_unsuffixed(value.7);
        let i = Literal::u8_unsuffixed(value.8);
        let j = Literal::u8_unsuffixed(value.9);
        let k = Literal::u8_unsuffixed(value.10);

        // TODO: once code complete measure how much longer it takes if-any to use from_u128 to produce a more compact package

        quote! {
            #guid { data1:#a, data2:#b, data3:#c, data4:[#d, #e, #f, #g, #h, #i, #j, #k] }
        }
    } else {
        format!("{}::from_u128(0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?})", guid.into_string(), value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10).into()
    }
}

pub fn gen_constant_value(value: &ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { #value },
        ConstantValue::U8(value) => quote! { #value },
        ConstantValue::I8(value) => quote! { #value },
        ConstantValue::U16(value) => quote! { #value },
        ConstantValue::I16(value) => quote! { #value },
        ConstantValue::U32(value) => quote! { #value },
        ConstantValue::I32(value) => quote! { #value },
        ConstantValue::U64(value) => quote! { #value },
        ConstantValue::I64(value) => quote! { #value },
        ConstantValue::F32(value) => quote! { #value },
        ConstantValue::F64(value) => quote! { #value },
        ConstantValue::String(value) => quote! { #value },
        _ => unimplemented!(),
    }
}
