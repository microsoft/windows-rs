use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.reader.type_def_kind(def) != TypeKind::Interface || (!gen.component && !gen.reader.type_def_can_implement(def)) {
        return quote! {};
    }

    let generics: &Vec<Type> = &gen.reader.type_def_generics(def).collect();
    let type_ident = to_ident(gen.reader.type_def_name(def));
    let impl_ident = type_ident.join("_Impl");
    let vtbl_ident = type_ident.join("_Vtbl");
    let implvtbl_ident = impl_ident.join("Vtbl");
    let constraints = gen.generic_constraints(generics);
    let generic_names = gen.generic_names(generics);
    let named_phantoms = gen.generic_named_phantoms(generics);
    let cfg = gen.reader.type_def_cfg_impl(def, generics);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);
    let mut requires = quote! {};
    let type_ident = quote! { #type_ident<#generic_names> };
    let vtables = gen.reader.type_def_vtables(def);
    let has_unknown_base = matches!(vtables.first(), Some(Type::IUnknown));

    fn gen_required_trait(gen: &Gen, def: TypeDef, generics: &[Type]) -> TokenStream {
        let name = gen.type_def_name_imp(def, generics, "_Impl");
        quote! {
            + #name
        }
    }

    let mut matches = quote! { iid == &<#type_ident as ::windows::core::Interface>::IID };

    if let Some(Type::TypeDef((def, _))) = vtables.last() {
        requires.combine(&gen_required_trait(gen, *def, &[]))
    }

    for def in &vtables {
        if let Type::TypeDef((def, generics)) = def {
            let name = gen.type_def_name(*def, generics);

            matches.combine(&quote! {
                || iid == &<#name as ::windows::core::Interface>::IID
            })
        }
    }

    if gen.reader.type_def_flags(def).contains(TypeAttributes::WINRT) {
        // TODO: this awkward wrapping of TypeDefs needs fixing
        for interface in gen.reader.type_interfaces(&Type::TypeDef((def, generics.to_vec()))) {
            if let Type::TypeDef((def, generics)) = interface.ty {
                requires.combine(&gen_required_trait(gen, def, &generics));
            }
        }
    }

    let runtime_name = gen.runtime_name_trait(def, generics, &type_ident, &constraints, &features);

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(gen, def);

    let method_traits = gen.reader.type_def_methods(def).map(|method| {
        let name = method_names.add(gen, method);
        let signature = gen.reader.method_def_signature(method, generics);
        let signature_tokens = gen.impl_signature(def, &signature);
        quote! { fn #name #signature_tokens; }
    });

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(gen, def);

    let method_impls = gen.reader.type_def_methods(def).map(|method| {
        let name = method_names.add(gen, method);
        let signature = gen.reader.method_def_signature(method, generics);
        let vtbl_signature = gen.vtbl_signature(def, generics, &signature);

        let invoke_upcall = if gen.reader.type_def_flags(def).contains(TypeAttributes::WINRT) { winrt_methods::gen_upcall(gen, &signature, quote! { this.#name }) } else { com_methods::gen_upcall(gen, &signature, quote! { this.#name }) };

        if has_unknown_base {
            quote! {
                unsafe extern "system" fn #name<#constraints Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: #impl_ident<#generic_names>, const OFFSET: isize> #vtbl_signature {
                    // offset the `this` pointer by `OFFSET` times the size of a pointer and cast it as an IUnknown implementation
                    let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
                    let this = (*this).get_impl();
                    #invoke_upcall
                }
            }
        } else {
            quote! {
                unsafe extern "system" fn #name<Impl: #impl_ident> #vtbl_signature {
                    let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
                    let this = &*((*this).this as *const Impl);
                    #invoke_upcall
                }
            }
        }
    });

    let mut methods = quote! {};

    match vtables.last() {
        Some(Type::IUnknown) => methods.combine(&quote! { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), }),
        Some(Type::IInspectable) => methods.combine(&quote! { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, #type_ident, OFFSET>(), }),
        Some(Type::TypeDef((def, generics))) => {
            let name = gen.type_def_name_imp(*def, generics, "_Vtbl");
            if has_unknown_base {
                methods.combine(&quote! { base__: #name::new::<Identity, Impl, OFFSET>(), });
            } else {
                methods.combine(&quote! { base__: #name::new::<Impl>(), });
            }
        }
        _ => {}
    }

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(gen, def);

    for method in gen.reader.type_def_methods(def) {
        let name = method_names.add(gen, method);
        if has_unknown_base {
            methods.combine(&quote! { #name: #name::<#generic_names Identity, Impl, OFFSET>, });
        } else {
            methods.combine(&quote! { #name: #name::<Impl>, });
        }
    }

    if has_unknown_base {
        quote! {
            #doc
            #features
            pub trait #impl_ident<#generic_names> : Sized #requires where #constraints {
                #(#method_traits)*
            }
            #runtime_name
            #features
            impl<#constraints> #vtbl_ident<#generic_names> {
                pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: #impl_ident<#generic_names>, const OFFSET: isize>() -> #vtbl_ident<#generic_names> {
                    #(#method_impls)*
                    Self{
                        #methods
                        #(#named_phantoms)*
                    }
                }
                pub fn matches(iid: &windows::core::GUID) -> bool {
                    #matches
                }
            }
        }
    } else {
        quote! {
            #doc
            #features
            pub trait #impl_ident : Sized #requires {
                #(#method_traits)*
            }
            #features
            impl #vtbl_ident {
                pub const fn new<Impl: #impl_ident>() -> #vtbl_ident {
                    #(#method_impls)*
                    Self{
                        #methods
                        #(#named_phantoms)*
                    }
                }
            }
            #[doc(hidden)]
            #features
            struct #implvtbl_ident<T: #impl_ident> (::std::marker::PhantomData<T>);
            #features
            impl<T: #impl_ident> #implvtbl_ident<T> {
                const VTABLE: #vtbl_ident = #vtbl_ident::new::<T>();
            }
            #features
            impl #type_ident {
                pub fn new<'a, T: #impl_ident>(this: &'a T) -> ::windows::core::ScopedInterface<'a, Self> {
                    let this = ::windows::core::ScopedHeap { vtable: &#implvtbl_ident::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                    let this = ::std::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                    unsafe { ::windows::core::ScopedInterface::new(::std::mem::transmute(&this.vtable)) }
                }
            }
        }
    }
}
