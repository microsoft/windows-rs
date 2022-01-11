use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        gen_sys_interface(def, gen)
    } else {
        gen_win_interface(def, gen)
    }
}

fn gen_sys_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);

    if def.is_exclusive() {
        quote! {}
    } else if def.kind() == TypeKind::Interface || def.default_interface().is_some() {
        // TODO: should be *const?
        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        quote! {}
    }
}

fn gen_win_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let is_exclusive = def.is_exclusive();
    let phantoms = gen_phantoms(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let cfg = gen.type_cfg(def);
    let doc = cfg.gen_doc(gen);
    let features = cfg.gen(gen);

    let mut tokens = if is_exclusive {
        quote! { #[doc(hidden)] }
    } else {
        quote! { #doc }
    };

    tokens.combine(&quote! {
        #features
        #[repr(transparent)]
        pub struct #name(::windows::core::IUnknown, #(#phantoms)*) where #(#constraints)*;
    });

    if !is_exclusive {
        tokens.combine(&gen_methods(def, &cfg, gen));
        tokens.combine(&gen_conversions(def, &cfg, gen));
        tokens.combine(&gen_std_traits(def, &cfg, gen));
        tokens.combine(&gen_runtime_trait(def, &cfg, gen));
        tokens.combine(&gen_async(def, &cfg, gen));
        tokens.combine(&gen_iterator(def, &cfg, gen));
        tokens.combine(&gen_agile(def, gen));
    }

    tokens.combine(&gen_interface_trait(def, &cfg, gen));
    tokens.combine(&gen_vtbl(def, &cfg, gen));
    tokens
}

fn gen_methods(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let mut methods = quote! {};
    let is_winrt = def.is_winrt();
    let mut vtable_offset = 0;
    let mut method_names = BTreeMap::<String, u32>::new();
    let cfg = cfg.gen(gen);

    for def in def.vtable_types() {
        match def {
            ElementType::IUnknown | ElementType::IInspectable => vtable_offset += 3,
            ElementType::TypeDef(def) => {
                for method in def.methods() {
                    if is_winrt {
                        methods.combine(&gen_winrt_method(&def, InterfaceKind::Default, &method, vtable_offset, &mut method_names, gen));
                    } else {
                        methods.combine(&gen_com_method(&def, &method, vtable_offset, &mut method_names, gen));
                    }
                    vtable_offset += 1;
                }
            }
            _ => unimplemented!(),
        }
    }

    if is_winrt && !gen.min_inherit {
        for def in def.required_interfaces() {
            let mut vtable_offset = 6;
            for method in def.methods() {
                methods.combine(&gen_winrt_method(&def, InterfaceKind::NonDefault, &method, vtable_offset, &mut method_names, gen));
                vtable_offset += 1;
            }
        }
    }

    quote! {
        #cfg
        impl<#(#constraints)*> #name {
            #methods
        }
    }
}

fn gen_conversions(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let mut tokens = quote! {};

    // vtable_types includes self at the end so reverse and skip it
    for def in def.vtable_types().iter().rev().skip(1) {
        let into = gen_element_name(def, gen);
        let cfg = cfg.union(gen.element_cfg(def)).gen(gen);
        tokens.combine(&quote! {
            #cfg
            impl<#(#constraints)*> ::core::convert::From<#name> for #into {
                fn from(value: #name) -> Self {
                    unsafe { ::core::mem::transmute(value) }
                }
            }
            #cfg
            impl<#(#constraints)*> ::core::convert::From<&#name> for #into {
                fn from(value: &#name) -> Self {
                    ::core::convert::From::from(::core::clone::Clone::clone(value))
                }
            }
            #cfg
            impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
                }
            }
            #cfg
            impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for &#name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
                }
            }
        });
    }

    if def.is_winrt() {
        for def in def.required_interfaces() {
            let into = gen_type_name(&def, gen);
            let cfg = cfg.union(gen.type_cfg(&def)).gen(gen);
            tokens.combine(&quote! {
                #cfg
                impl<#(#constraints)*> ::core::convert::TryFrom<#name> for #into {
                    type Error = ::windows::core::Error;
                    fn try_from(value: #name) -> ::windows::core::Result<Self> {
                        ::core::convert::TryFrom::try_from(&value)
                    }
                }
                #cfg
                impl<#(#constraints)*> ::core::convert::TryFrom<&#name> for #into {
                    type Error = ::windows::core::Error;
                    fn try_from(value: &#name) -> ::windows::core::Result<Self> {
                        ::windows::core::Interface::cast(value)
                    }
                }
                #cfg
                impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for #name {
                    fn into_param(self) -> ::windows::core::Param<'a, #into> {
                        ::windows::core::IntoParam::into_param(&self)
                    }
                }
                #cfg
                impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for &#name {
                    fn into_param(self) -> ::windows::core::Param<'a, #into> {
                        ::core::convert::TryInto::<#into>::try_into(self)
                            .map(::windows::core::Param::Owned)
                            .unwrap_or(::windows::core::Param::None)
                    }
                }
            });
        }
    }

    tokens
}

fn gen_agile(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.type_name() == TypeName::IRestrictedErrorInfo || def.async_kind() != AsyncKind::None {
        let name = gen_type_ident(def, gen);
        let constraints = gen_type_constraints(def, gen);
        quote! {
            unsafe impl<#(#constraints)*> ::core::marker::Send for #name {}
            unsafe impl<#(#constraints)*> ::core::marker::Sync for #name {}
        }
    } else {
        TokenStream::new()
    }
}
