use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.sys {
        gen_sys_interface(gen, def)
    } else {
        gen_win_interface(gen, def)
    }
}

fn gen_sys_interface(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = gen.reader.type_def_name(def);
    let ident = to_ident(name);

    if gen.reader.type_def_is_exclusive(def) {
        quote! {}
    } else {
        quote! {
            pub type #ident = *mut ::core::ffi::c_void;
        }
    }
}

fn gen_win_interface(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = gen.reader.type_def_name(def);
    let ident = to_ident(name);

    if gen.reader.type_def_methods(def).next().is_none() {
        if name.starts_with("Disp") {
            if let Some(guid) = gen.reader.type_def_guid(def) {
                let value = gen.guid(&guid);
                let guid = gen.type_name(&Type::GUID);
                return quote! { pub const #ident: #guid = #value; };
            }
        }
    }

    let is_exclusive = gen.reader.type_def_is_exclusive(def);
    let generics: Vec<Type> = gen.reader.type_def_generics(def).collect();
    let phantoms = gen.generic_phantoms(&generics);
    let constraints = gen.generic_constraints(&generics);
    let cfg = gen.reader.type_def_cfg(def, &[]);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    let mut tokens = if is_exclusive {
        quote! { #[doc(hidden)] }
    } else {
        quote! { #doc }
    };

    tokens.combine(&quote! {
        #features
        #[repr(transparent)]
        pub struct #ident(::windows::core::IUnknown, #phantoms) where #constraints;
    });

    if !is_exclusive {
        let methods = gen_methods(gen, def, &generics);
        tokens.combine(&quote! {
            #features
            impl<#constraints> #ident {
                #methods
            }
        });

        //     tokens.combine(&gen_methods(gen, def, &cfg, gen));
        //     tokens.combine(&gen_conversions(gen, def, &cfg, gen));
        //     tokens.combine(&gen_std_traits(gen, def, &cfg, gen));
        //     tokens.combine(&gen_runtime_trait(gen, def, &cfg, gen));
        //     tokens.combine(&gen_async(gen, def, &cfg, gen));
        //     tokens.combine(&gen_iterator(gen, def, &cfg, gen));
        //     tokens.combine(&gen_agile(gen, def, gen));
    }

    // tokens.combine(&gen_interface_trait(def, &cfg, gen));
    // tokens.combine(&gen_vtbl(def, &cfg, gen));
    // tokens

    tokens
}

fn gen_methods(gen: &Gen, def: TypeDef, generics: &[Type]) -> TokenStream {
    let mut methods = quote! {};
    // TODO: why do we need to distinguish between public and virtual methods?
    let method_names = &mut MethodNames::new();
    let virtual_names = &mut MethodNames::new();

    if gen.reader.type_def_flags(def).winrt() {
        for method in gen.reader.type_def_methods(def) {
            methods.combine(&winrt_methods::gen(gen, def, InterfaceKind::Default, method, method_names, virtual_names));
        }
        if !gen.min_inherit {
            for ty in gen.reader.type_def_interfaces(def, generics) {
                for method in gen.reader.type_def_methods(def) {
                    methods.combine(&winrt_methods::gen(gen, def, InterfaceKind::None, method, method_names, virtual_names));
                }
            }
        }
    } else {
        let vtable_types = gen.reader.type_def_vtables(def);
        let mut bases = vtable_types.len();
        for ty in vtable_types {
            match ty {
                Type::IUnknown | Type::IInspectable => {}
                Type::TypeDef((def, _)) => {
                    let kind = if gen.reader.type_def_type_name(def) == TypeName::IDispatch { InterfaceKind::None } else { InterfaceKind::Default };
                    for method in gen.reader.type_def_methods(def) {
                        methods.combine(&com_methods::gen(gen, def, kind, method, method_names, virtual_names, bases));
                    }
                }
                _ => unimplemented!(),
            }

            bases -= 1;
        }
        for method in gen.reader.type_def_methods(def) {
            methods.combine(&com_methods::gen(gen, def, InterfaceKind::Default, method, method_names, virtual_names, 0));
        }
    }
    methods
}

// fn gen_conversions(gen: &Gen, def: TypeDef, cfg: &Cfg) -> TokenStream {
//     let name = gen_type_ident(def, gen);
//     let constraints = gen_type_constraints(def, gen);
//     let mut tokens = quote! {};

//     for def in def.vtable_types() {
//         let into = gen_element_name(&def, gen);
//         let cfg = gen.cfg(&cfg.union(&def.cfg()));
//         tokens.combine(&quote! {
//             #cfg
//             impl<#(#constraints)*> ::core::convert::From<#name> for #into {
//                 fn from(value: #name) -> Self {
//                     unsafe { ::core::mem::transmute(value) }
//                 }
//             }
//             #cfg
//             impl<#(#constraints)*> ::core::convert::From<&#name> for #into {
//                 fn from(value: &#name) -> Self {
//                     ::core::convert::From::from(::core::clone::Clone::clone(value))
//                 }
//             }
//             #cfg
//             impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for #name {
//                 fn into_param(self) -> ::windows::core::Param<'a, #into> {
//                     ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
//                 }
//             }
//             #cfg
//             impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for &'a #name {
//                 fn into_param(self) -> ::windows::core::Param<'a, #into> {
//                     ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
//                 }
//             }
//         });
//     }

//     if def.is_winrt() {
//         for def in def.required_interfaces() {
//             let into = gen_type_name(&def, gen);
//             let cfg = gen.cfg(&cfg.union(&def.cfg()));
//             tokens.combine(&quote! {
//                 #cfg
//                 impl<#(#constraints)*> ::core::convert::TryFrom<#name> for #into {
//                     type Error = ::windows::core::Error;
//                     fn try_from(value: #name) -> ::windows::core::Result<Self> {
//                         ::core::convert::TryFrom::try_from(&value)
//                     }
//                 }
//                 #cfg
//                 impl<#(#constraints)*> ::core::convert::TryFrom<&#name> for #into {
//                     type Error = ::windows::core::Error;
//                     fn try_from(value: &#name) -> ::windows::core::Result<Self> {
//                         ::windows::core::Interface::cast(value)
//                     }
//                 }
//                 #cfg
//                 impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for #name {
//                     fn into_param(self) -> ::windows::core::Param<'a, #into> {
//                         ::windows::core::IntoParam::into_param(&self)
//                     }
//                 }
//                 #cfg
//                 impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for &#name {
//                     fn into_param(self) -> ::windows::core::Param<'a, #into> {
//                         ::core::convert::TryInto::<#into>::try_into(self)
//                             .map(::windows::core::Param::Owned)
//                             .unwrap_or(::windows::core::Param::None)
//                     }
//                 }
//             });
//         }
//     }

//     tokens
// }

// fn gen_agile(gen: &Gen, def: TypeDef) -> TokenStream {
//     if def.type_name() == TypeName::IRestrictedErrorInfo || def.async_kind() != AsyncKind::None {
//         let name = gen_type_ident(def, gen);
//         let constraints = gen_type_constraints(def, gen);
//         quote! {
//             unsafe impl<#(#constraints)*> ::core::marker::Send for #name {}
//             unsafe impl<#(#constraints)*> ::core::marker::Sync for #name {}
//         }
//     } else {
//         TokenStream::new()
//     }
// }
