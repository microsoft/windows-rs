use super::*;

pub fn gen_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        gen_sys_interface(def, gen)
    } else if def.kind() == TypeKind::Class {
        gen_win_class(def, gen)
    } else {
        gen_win_interface(def, gen)
    }
}

fn gen_sys_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_name(def, gen);

    if def.is_exclusive() {
        quote! {}
    } else {
        if def.kind() == TypeKind::Interface || def.default_interface().is_some() {
            // TODO: should be *const?
            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    }
}

fn gen_win_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_name(def, gen);
    let is_exclusive = def.is_exclusive();
    let phantoms = gen_phantoms(def, gen);
    let constraints = gen_type_constraints(def, gen);

    let mut tokens = if is_exclusive {
        quote! { #[doc(hidden)] }
    } else {
        quote! {}
    };

    tokens.combine(&quote! {
        #[repr(transparent)]
        pub struct #name(pub ::windows::core::IUnknown, #(#phantoms)*) where #(#constraints)*;
    });

    tokens.combine(&gen_methods(def, gen));
    tokens.combine(&gen_std_traits(def, gen));
    tokens.combine(&gen_interface_trait(def, gen));
    tokens.combine(&gen_runtime_trait(def, gen));
    tokens.combine(&gen_vtbl(def, gen));
    tokens
}

fn gen_win_class(def: &TypeDef, _gen: &Gen) -> TokenStream {
    if let Some(_default_interface) = def.default_interface() {
        quote! {}
    } else {
        quote! {}
    }
}

fn gen_std_traits(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.is_exclusive() {
        quote! {}
    } else {
        let name = gen_generic_name(def, gen);
        let constraints = gen_type_constraints(def, gen);
        let phantoms = gen_phantoms(def, gen);

        quote! {
            impl<#(#constraints)*> ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone(), #(#phantoms)*)
                }
            }
            impl<#(#constraints)*> ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl<#(#constraints)*> ::core::cmp::Eq for #name {}
        }
    }
}

fn gen_interface_trait(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_name(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let vtbl = gen_vtbl_name(def, gen);
    let guid = gen_type_guid(def, gen, &"Self".into());

    quote! {
        unsafe impl<#(#constraints)*> ::windows::core::Interface for #name {
            type Vtable = #vtbl;
            const IID: ::windows::core::GUID = #guid;
        }
    }
}

fn gen_runtime_trait(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.is_winrt() {
        let name = gen_generic_name(def, gen);
        let constraints = gen_type_constraints(def, gen);
        let type_signature = gen_guid_signature(def, &format!("{{{:#?}}}", def.guid()), gen);

        quote! {
            unsafe impl<#(#constraints)*> ::windows::core::RuntimeType for #name {
                const SIGNATURE: ::windows::core::ConstBuffer = #type_signature;
            }
        }
    } else {
        quote! {}
    }
}

fn gen_methods(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.is_exclusive() {
        return quote! {};
    }

    let name = gen_generic_name(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let mut methods = quote! {};
    let is_winrt = def.is_winrt();

    for def in def.vtable_types() {
        if let ElementType::TypeDef(def) = def {
            for method in def.methods() {
                if is_winrt {
                    methods.combine(&gen_winrt_method(&def, &method, gen));
                } else {
                    methods.combine(&gen_com_method(&def, &method, gen));
                }
            }
        }
    }

    quote! {
        impl<#(#constraints)*> #name {
            #methods
        }
    }
}

fn gen_winrt_method(def: &TypeDef, method: &MethodDef, gen: &Gen) -> TokenStream {
    let signature = method.signature(&def.generics);
    let name = gen_ident(&method.rust_name());
    let constraints = gen_param_constraints(&signature.params, gen);
    let arch_cfg = gen.arch_cfg(method.attributes());
    let feature_cfg = gen.method_cfg(&method).0;
    let params = gen_params(&signature, gen);

    let return_type = if let Some(return_sig) = &signature.return_sig {
        let tokens = gen_sig(return_sig, gen);
        quote! { -> ::windows::core::Result<#tokens> }
    } else {
        quote! { -> ::windows::core::Result<()> }
    };

    quote! {
        #arch_cfg
        #feature_cfg
        pub fn #name<#constraints>(&self, #params) #return_type {
            unimplemented!()
        }
    }
}

fn gen_com_method(def: &TypeDef, method: &MethodDef, gen: &Gen) -> TokenStream {
    let signature = method.signature(&def.generics);
    let name = gen_ident(&method.rust_name());
    let constraints = gen_param_constraints(&signature.params, gen);
    let arch_cfg = gen.arch_cfg(method.attributes());
    let feature_cfg = gen.method_cfg(&method).0;
    let params = gen_params(&signature, gen);

    // TODO: use SignatureKind...
    let return_type = gen_return_sig(&signature, gen);

    quote! {
        #arch_cfg
        #feature_cfg
        pub unsafe fn #name<#constraints>(&self, #params) #return_type {
            unimplemented!()
        }
    }
}

fn gen_params(signature: &MethodSignature, gen: &Gen) -> TokenStream {
    let mut tokens = quote! {};

    for (position, param) in signature.params.iter().enumerate() {
        let name = gen_param_name(&param.param);
        let kind = if param.is_convertible() { format!("Param{}", position).into() } else { gen_param_sig(param, gen) };
        tokens.combine(&quote! { #name: #kind, });
    }

    tokens
}

fn gen_vtbl(def: &TypeDef, gen: &Gen) -> TokenStream {
    // TODO: consider using parent field to avoid duplicating inherited vfptrs.
    // And then consider naming them to simplify traits and debugging.
    // Should the first param be the Vtbl type?

    let vtbl = gen_vtbl_name(def, gen);
    let mut tokens: TokenStream = format!("#[repr(C)] #[doc(hidden)] pub struct {} (", vtbl.as_str()).into();
    let guid = gen_element_name(&ElementType::GUID, gen);
    let hresult = gen_element_name(&ElementType::HRESULT, gen);

    for def in def.vtable_types() {
        match def {
            ElementType::TypeDef(def) => {
                for method in def.methods() {
                    let signature = method.signature(&def.generics);
                    let return_type = gen_return_sig(&signature, gen);
                    let (feature_cfg, not_feature_cfg) = gen.method_cfg(&method);

                    let params = signature.params.iter().map(|param| {
                        let name = gen_param_name(&param.param);
                        let tokens = gen_abi_param_sig(param, gen);
                        quote! { #name: #tokens }
                    });

                    let signature = quote! { pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, #(#params),*) #return_type, };

                    if feature_cfg.is_empty() {
                        tokens.combine(&signature);
                    } else {
                        tokens.combine(&quote! {
                            #feature_cfg
                            #signature
                            #not_feature_cfg
                            usize,
                        });
                    }
                }
            }
            ElementType::IInspectable => tokens.combine(&quote! {
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut #guid) -> #hresult,
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> #hresult,
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> #hresult,
            }),
            ElementType::IUnknown => tokens.combine(&quote! {
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &#guid, interface: *mut *mut ::core::ffi::c_void) -> #hresult,
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
                pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
            }),
            _ => unimplemented!(),
        }
    }

    let phantoms = gen_phantoms(def, gen);
    tokens.combine(&quote! {
        #(pub #phantoms)*
    });

    tokens.push_str(");");
    tokens
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
