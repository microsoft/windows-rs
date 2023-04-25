use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen
        .reader
        .type_def_flags(def)
        .contains(TypeAttributes::WINRT)
    {
        gen_delegate(gen, def)
    } else {
        gen_callback(gen, def)
    }
}

fn gen_callback(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = to_ident(gen.reader.type_def_name(def));

    let method = gen.reader.type_def_invoke_method(def);
    let signature = gen.reader.method_def_signature(method, &[]);
    let return_type = gen.return_sig(&signature);
    let cfg = gen.reader.type_def_cfg(def, &[]);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    let params = signature.params.iter().map(|p| {
        let name = gen.param_name(p.def);
        let tokens = gen.type_default_name(&p.ty);
        quote! { #name: #tokens }
    });

    quote! {
        #doc
        #features
        pub type #name = ::core::option::Option<unsafe extern "system" fn(#(#params),*) #return_type>;
    }
}

fn gen_delegate(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.sys {
        let name = to_ident(gen.reader.type_def_name(def));
        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        gen_win_delegate(gen, def)
    }
}

fn gen_win_delegate(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = to_ident(gen.reader.type_def_name(def));
    let vtbl = name.join("_Vtbl");
    let boxed = name.join("Box");

    let generics: &Vec<Type> = &gen.reader.type_def_generics(def).collect();
    let phantoms = gen.generic_phantoms(generics);
    let named_phantoms = gen.generic_named_phantoms(generics);
    let constraints = gen.generic_constraints(generics);
    let generic_names = gen.generic_names(generics);

    let ident = gen.type_def_name(def, generics);

    let method = gen.reader.type_def_invoke_method(def);
    let signature = gen.reader.method_def_signature(method, generics);
    let fn_constraint = gen_fn_constraint(gen, def, &signature);

    let cfg = gen.reader.type_def_cfg(def, generics);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    let vtbl_signature = gen.vtbl_signature(def, generics, &signature);
    let invoke = winrt_methods::gen(
        gen,
        def,
        generics,
        InterfaceKind::Default,
        method,
        &mut MethodNames::new(),
        &mut MethodNames::new(),
    );
    let invoke_upcall = winrt_methods::gen_upcall(gen, &signature, quote! { ((*this).invoke) });

    let mut tokens = quote! {
        #doc
        #features
        #[repr(transparent)]
        pub struct #ident(pub ::windows_core::IUnknown, #phantoms) where #constraints;
        #features
        impl<#constraints> #ident {
            pub fn new<#fn_constraint>(invoke: F) -> Self {
                let com = #boxed::<#generic_names F> {
                    vtable: &#boxed::<#generic_names F>::VTABLE,
                    count: ::windows_core::imp::RefCount::new(1),
                    invoke,
                };
                unsafe {
                    ::core::mem::transmute(::std::boxed::Box::new(com))
                }
            }
            #invoke
        }
        #features
        #[repr(C)]
        struct #boxed<#generic_names #fn_constraint> where #constraints {
            vtable: *const #vtbl<#generic_names>,
            invoke: F,
            count: ::windows_core::imp::RefCount,
        }
        #features
        impl<#constraints #fn_constraint> #boxed<#generic_names F> {
            const VTABLE: #vtbl<#generic_names> = #vtbl::<#generic_names>{
                base__: ::windows_core::IUnknown_Vtbl{QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release},
                Invoke: Self::Invoke,
                #(#named_phantoms)*
            };
            unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
                let this = this as *mut *mut ::core::ffi::c_void as *mut Self;

                *interface = if iid == &<#ident as ::windows_core::ComInterface>::IID ||
                    iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID ||
                    iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID {
                        &mut (*this).vtable as *mut _ as _
                    } else {
                        ::core::ptr::null_mut()
                    };

                // TODO: implement IMarshal

                if (*interface).is_null() {
                    ::windows_core::HRESULT(-2147467262) // E_NOINTERFACE
                } else {
                    (*this).count.add_ref();
                    ::windows_core::HRESULT(0)
                }
            }
            unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
                let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
                (*this).count.add_ref()
            }
            unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
                let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
                let remaining = (*this).count.release();

                if remaining == 0 {
                    let _ = ::std::boxed::Box::from_raw(this);
                }

                remaining
            }
            unsafe extern "system" fn Invoke #vtbl_signature {
                let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
                #invoke_upcall
            }
        }
    };

    tokens.combine(&gen.interface_core_traits(
        def,
        generics,
        &ident,
        &constraints,
        &phantoms,
        &features,
    ));
    tokens.combine(&gen.interface_trait(def, generics, &ident, &constraints, &features, true));
    tokens.combine(&gen.interface_winrt_trait(
        def,
        generics,
        &ident,
        &constraints,
        &phantoms,
        &features,
    ));
    tokens.combine(&gen.interface_vtbl(def, generics, &ident, &constraints, &features));
    tokens
}

fn gen_fn_constraint(gen: &Gen, def: TypeDef, signature: &Signature) -> TokenStream {
    let signature = gen.impl_signature(def, signature);

    quote! { F: FnMut #signature + ::core::marker::Send + 'static }
}
