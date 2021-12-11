use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);

    if gen.sys {
        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        gen_win_delegate(def, gen)
    }
}

fn gen_win_delegate(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident2(def);
    let vtbl = name.join("Vtbl");
    let boxed = name.join("Box");

    let phantoms = gen_phantoms(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let generics = gen_type_generics(def, gen);

    let method = def.invoke_method();
    let signature = method.signature(&def.generics);
    let fn_constraint = gen_fn_constraint(&signature, gen);
    let cfg = gen.type_cfg(def);
    let doc = cfg.gen_doc(gen);
    let features = cfg.gen(gen);
    let vtbl_signature = gen_vtbl_signature(def, &method, gen);
    let invoke = gen_winrt_method(def, InterfaceKind::Default, &method, 3, &mut BTreeMap::new(), gen);
    let invoke_upcall = gen_winrt_upcall(&signature, quote! { ((*this).invoke) }, gen);

    let mut tokens = quote! {
        #doc
        #features
        #[repr(transparent)]
        pub struct #name<#(#generics)*>(pub ::windows::core::IUnknown, #(#phantoms)*) where #(#constraints)*;
        #features
        impl<#(#constraints)*> #name<#(#generics)*> {
            pub fn new<#fn_constraint>(invoke: F) -> Self {
                let com = #boxed::<#(#generics)* F> {
                    vtable: &#boxed::<#(#generics)* F>::VTABLE,
                    count: ::windows::core::RefCount::new(1),
                    invoke,
                };
                unsafe {
                    ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com))
                }
            }
            #invoke
        }
        #features
        #[repr(C)]
        struct #boxed<#(#generics)* #fn_constraint> where #(#constraints)* {
            vtable: *const #vtbl<#(#generics)*>,
            invoke: F,
            count: ::windows::core::RefCount,
        }
        #features
        impl<#(#constraints)* #fn_constraint> #boxed<#(#generics)* F> {
            const VTABLE: #vtbl<#(#generics)*> = #vtbl::<#(#generics)*>(
                Self::QueryInterface,
                Self::AddRef,
                Self::Release,
                Self::Invoke,
                #(#phantoms)*
            );
            unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
                let this = this as *mut ::windows::core::RawPtr as *mut Self;

                *interface = if iid == &<#name<#(#generics)*> as ::windows::core::Interface>::IID ||
                    iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID ||
                    iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
                        &mut (*this).vtable as *mut _ as _
                    } else {
                        ::core::ptr::null_mut()
                    };

                // TODO: implement IMarshal

                if (*interface).is_null() {
                    ::windows::core::HRESULT(-2147467262) // E_NOINTERFACE
                } else {
                    (*this).count.add_ref();
                    ::windows::core::HRESULT(0)
                }
            }
            unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
                let this = this as *mut ::windows::core::RawPtr as *mut Self;
                (*this).count.add_ref()
            }
            unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
                let this = this as *mut ::windows::core::RawPtr as *mut Self;
                let remaining = (*this).count.release();

                if remaining == 0 {
                    ::windows::core::alloc::boxed::Box::from_raw(this);
                }

                remaining
            }
            unsafe extern "system" fn Invoke #vtbl_signature {
                let this = this as *mut ::windows::core::RawPtr as *mut Self;
                #invoke_upcall
            }
        }
    };

    tokens.combine(&gen_std_traits(def, &cfg, gen));
    tokens.combine(&gen_interface_trait(def, &cfg, gen));
    tokens.combine(&gen_runtime_trait(def, &cfg, gen));
    tokens.combine(&gen_vtbl(def, &cfg, gen));

    tokens
}

fn gen_fn_constraint(sig: &MethodSignature, gen: &Gen) -> TokenStream {
    let params = sig.params.iter().map(|p| gen_produce_type(p, gen));

    let return_sig = if let Some(return_sig) = &sig.return_sig {
        let tokens = gen_element_name(&return_sig.kind, gen);

        if return_sig.is_array {
            quote! { ::windows::core::Array<#tokens> }
        } else {
            tokens
        }
    } else {
        quote! { () }
    };

    quote! { F: FnMut(#(#params),*) -> ::windows::core::Result<#return_sig> + 'static }
}

fn gen_produce_type(param: &MethodParam, gen: &Gen) -> TokenStream {
    let tokens = gen_element_name(&param.signature.kind, gen);

    if param.signature.is_array {
        if param.param.is_input() {
            quote! { &[<#tokens as ::windows::core::DefaultType>::DefaultType] }
        } else if param.signature.by_ref {
            quote! { &mut ::windows::core::Array<#tokens> }
        } else {
            quote! { &mut [<#tokens as ::windows::core::DefaultType>::DefaultType] }
        }
    } else if param.param.is_input() {
        if let ElementType::GenericParam(_) = param.signature.kind {
            quote! { &<#tokens as ::windows::core::DefaultType>::DefaultType }
        } else if param.signature.kind.is_primitive() {
            quote! { #tokens }
        } else if param.signature.kind.is_nullable() {
            quote! { &::core::option::Option<#tokens> }
        } else {
            quote! { &#tokens }
        }
    } else if param.signature.kind.is_nullable() {
        quote! { &mut ::core::option::Option<#tokens> }
    } else {
        quote! { &mut #tokens }
    }
}
