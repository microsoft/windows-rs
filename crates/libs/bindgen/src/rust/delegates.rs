use super::*;

pub fn writer(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
        gen_delegate(writer, def)
    } else {
        gen_callback(writer, def)
    }
}

fn gen_callback(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    let name = to_ident(def.name());
    let method = metadata::type_def_invoke_method(def);

    let signature = metadata::method_def_signature(def.namespace(), method, &[]);

    let return_type = writer.return_sig(&signature);
    let cfg = cfg::type_def_cfg(writer, def, &[]);
    let features = writer.cfg_features(&cfg);

    let params = signature.params.iter().map(|p| {
        let name = writer.param_name(p.def);
        let tokens = writer.type_default_name(&p.ty);
        quote! { #name: #tokens }
    });

    quote! {
        #features
        pub type #name = Option<unsafe extern "system" fn(#(#params),*) #return_type>;
    }
}

fn gen_delegate(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    if writer.sys {
        let name = to_ident(def.name());
        quote! {
            pub type #name = *mut core::ffi::c_void;
        }
    } else {
        gen_win_delegate(writer, def)
    }
}

fn gen_win_delegate(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    let name = to_ident(def.name());
    let vtbl = name.join("_Vtbl");
    let boxed = name.join("Box");

    let generics = &metadata::type_def_generics(def);
    let phantoms = writer.generic_phantoms(generics);
    let named_phantoms = writer.generic_named_phantoms(generics);
    let constraints = writer.generic_constraints(generics);
    let generic_names = writer.generic_names(generics);

    let ident = writer.type_def_name(def, generics);
    let method = metadata::type_def_invoke_method(def);

    let signature = metadata::method_def_signature(def.namespace(), method, generics);

    let fn_constraint = gen_fn_constraint(writer, def, &signature);
    let cfg = cfg::type_def_cfg(writer, def, generics);
    let features = writer.cfg_features(&cfg);

    let vtbl_signature = writer.vtbl_signature(def, true, &signature);
    let invoke = winrt_methods::writer(writer, def, generics, metadata::InterfaceKind::Default, method, &mut MethodNames::new(), &mut MethodNames::new());
    let invoke_upcall = winrt_methods::gen_upcall(writer, &signature, quote! { (this.invoke) }, false);

    let mut tokens = if generics.is_empty() {
        let iid = writer.guid_literal(metadata::type_def_guid(def));
        quote! {
            #features
            windows_core::imp::define_interface!(#ident, #vtbl, #iid);
        }
    } else {
        quote! {
            #features
            #[repr(transparent)]
            #[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
            pub struct #ident(windows_core::IUnknown, #phantoms) where #constraints;
        }
    };

    tokens.combine(&quote! {
        #features
        impl<#constraints> #ident {
            pub fn new<#fn_constraint>(invoke: F) -> Self {
                let com = #boxed::<#generic_names F> {
                    vtable: &#boxed::<#generic_names F>::VTABLE,
                    count: windows_core::imp::RefCount::new(1),
                    invoke,
                };
                unsafe {
                    core::mem::transmute(Box::new(com))
                }
            }
            #invoke
        }
        #features
        #[repr(C)]
        struct #boxed<#generic_names #fn_constraint> where #constraints {
            vtable: *const #vtbl<#generic_names>,
            invoke: F,
            count: windows_core::imp::RefCount,
        }
        #features
        impl<#constraints #fn_constraint> #boxed<#generic_names F> {
            const VTABLE: #vtbl<#generic_names> = #vtbl::<#generic_names>{
                base__: windows_core::IUnknown_Vtbl{QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release},
                Invoke: Self::Invoke,
                #(#named_phantoms)*
            };
            unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
                let this = this as *mut *mut core::ffi::c_void as *mut Self;

                if iid.is_null() || interface.is_null() {
                    return windows_core::HRESULT(-2147467261); // E_POINTER
                }

                *interface = if *iid == <#ident as windows_core::Interface>::IID ||
                    *iid == <windows_core::IUnknown as windows_core::Interface>::IID ||
                    *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                        &mut (*this).vtable as *mut _ as _
                    } else {
                        core::ptr::null_mut()
                    };

                // TODO: implement IMarshal

                if (*interface).is_null() {
                    windows_core::HRESULT(-2147467262) // E_NOINTERFACE
                } else {
                    (*this).count.add_ref();
                    windows_core::HRESULT(0)
                }
            }
            unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
                let this = this as *mut *mut core::ffi::c_void as *mut Self;
                (*this).count.add_ref()
            }
            unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
                let this = this as *mut *mut core::ffi::c_void as *mut Self;
                let remaining = (*this).count.release();

                if remaining == 0 {
                    let _ = Box::from_raw(this);
                }

                remaining
            }
            unsafe extern "system" fn Invoke #vtbl_signature {
                let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
                #invoke_upcall
            }
        }
    });

    tokens.combine(&writer.interface_trait(def, generics, &ident, &constraints, &features, true));
    tokens.combine(&writer.interface_winrt_trait(def, generics, &ident, &constraints, &phantoms, &features));
    tokens.combine(&writer.interface_vtbl(def, generics, &constraints, &features));
    tokens
}

fn gen_fn_constraint(writer: &Writer, def: metadata::TypeDef, signature: &metadata::Signature) -> TokenStream {
    let signature = writer.impl_signature(def, signature);

    quote! { F: FnMut #signature + Send + 'static }
}
