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
    let name = gen_type_ident(def, gen);
    let is_exclusive = def.is_exclusive();
    let phantoms = gen_phantoms(def, gen);
    let constraints = gen_type_constraints(def, gen);

    let mut tokens = if is_exclusive {
        quote! { #[doc(hidden)] }
    } else {
        quote! {}
    };

    // TODO: exclude all (even the type itself) but the vtable if its exclusive

    tokens.combine(&quote! {
        #[repr(transparent)]
        pub struct #name(::windows::core::IUnknown, #(#phantoms)*) where #(#constraints)*;
    });

    if !is_exclusive {
        tokens.combine(&gen_methods(def, gen));
        tokens.combine(&gen_conversions(def, gen));
        tokens.combine(&gen_std_traits(def, gen));
        tokens.combine(&gen_runtime_trait(def, gen));    
    }

    tokens.combine(&gen_interface_trait(def, gen));
    tokens.combine(&gen_vtbl(def, gen));
    tokens
}

fn gen_methods(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let mut methods = quote! {};
    let is_winrt = def.is_winrt();
    let mut vtable_offset = 0;
    let mut method_names = BTreeMap::<String, u32>::new();

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

    quote! {
        impl<#(#constraints)*> #name {
            #methods
        }
    }
}

fn gen_conversions(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let mut tokens = quote! {};

    // vtable_types includes self at the end so reverse and skip it
    for def in def.vtable_types().iter().rev().skip(1) {
        let into = gen_element_name(def, gen);
        tokens.combine(&quote! {
            impl<#(#constraints)*> ::core::convert::From<#name> for #into {
                fn from(value: #name) -> Self {
                    unsafe { ::core::mem::transmute(value) }
                }
            }
            impl<#(#constraints)*> ::core::convert::From<&#name> for #into {
                fn from(value: &#name) -> Self {
                    ::core::convert::From::from(::core::clone::Clone::clone(value))
                }
            }
            impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
                }
            }
            impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for &#name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
                }
            }
        });
    }

    tokens
}
