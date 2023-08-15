use super::*;

pub fn writer(writer: &Writer, def: TypeDef) -> TokenStream {
    if writer.reader.type_def_kind(def) != TypeKind::Interface || (!writer.implement && writer.reader.has_attribute(def, "ExclusiveToAttribute")) {
        return quote! {};
    }

    let generics = &type_def_generics(writer.reader, def);
    let type_ident = to_ident(writer.reader.type_def_name(def));
    let impl_ident = type_ident.join("_Impl");
    let vtbl_ident = type_ident.join("_Vtbl");
    let implvtbl_ident = impl_ident.join("Vtbl");
    let constraints = writer.generic_constraints(generics);
    let generic_names = writer.generic_names(generics);
    let named_phantoms = writer.generic_named_phantoms(generics);
    let cfg = type_def_cfg_impl(writer.reader, def, generics);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);
    let mut requires = quote! {};
    let type_ident = quote! { #type_ident<#generic_names> };
    let vtables = type_def_vtables(writer.reader, def);
    let has_unknown_base = matches!(vtables.first(), Some(Type::IUnknown));

    fn gen_required_trait(writer: &Writer, def: TypeDef, generics: &[Type]) -> TokenStream {
        let name = writer.type_def_name_imp(def, generics, "_Impl");
        quote! {
            + #name
        }
    }

    let mut matches = quote! { iid == &<#type_ident as ::windows_core::ComInterface>::IID };

    if let Some(Type::TypeDef(def, _)) = vtables.last() {
        requires.combine(&gen_required_trait(writer, *def, &[]))
    }

    for def in &vtables {
        if let Type::TypeDef(def, generics) = def {
            let name = writer.type_def_name(*def, generics);

            matches.combine(&quote! {
                || iid == &<#name as ::windows_core::ComInterface>::IID
            })
        }
    }

    if writer.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
        // TODO: this awkward wrapping of TypeDefs needs fixing
        for interface in type_interfaces(writer.reader, &Type::TypeDef(def, generics.to_vec())) {
            if let Type::TypeDef(def, generics) = interface.ty {
                requires.combine(&gen_required_trait(writer, def, &generics));
            }
        }
    }

    let runtime_name = writer.runtime_name_trait(def, generics, &type_ident, &constraints, &features);

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(writer, def);

    let method_traits = writer.reader.type_def_methods(def).map(|method| {
        let name = method_names.add(writer, method);

        let signature = method_def_signature(writer.reader, writer.reader.type_def_namespace(def), method, generics);

        let signature_tokens = writer.impl_signature(def, &signature);
        quote! { fn #name #signature_tokens; }
    });

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(writer, def);

    let method_impls = writer.reader.type_def_methods(def).map(|method| {
        let name = method_names.add(writer, method);
        let signature = method_def_signature(writer.reader, writer.reader.type_def_namespace(def), method, generics);
        let vtbl_signature = writer.vtbl_signature(def, generics, &signature);

        let invoke_upcall = if writer.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) { winrt_methods::gen_upcall(writer, &signature, quote! { this.#name }) } else { com_methods::gen_upcall(writer, &signature, quote! { this.#name }) };

        if has_unknown_base {
            quote! {
                unsafe extern "system" fn #name<#constraints Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: #impl_ident<#generic_names>, const OFFSET: isize> #vtbl_signature {
                    // offset the `this` pointer by `OFFSET` times the size of a pointer and cast it as an IUnknown implementation
                    let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
                    let this = (*this).get_impl();
                    #invoke_upcall
                }
            }
        } else {
            quote! {
                unsafe extern "system" fn #name<Impl: #impl_ident> #vtbl_signature {
                    let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
                    let this = &*((*this).this as *const Impl);
                    #invoke_upcall
                }
            }
        }
    });

    let mut methods = quote! {};

    match vtables.last() {
        Some(Type::IUnknown) => methods.combine(&quote! { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), }),
        Some(Type::IInspectable) => methods.combine(&quote! { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, #type_ident, OFFSET>(), }),
        Some(Type::TypeDef(def, generics)) => {
            let name = writer.type_def_name_imp(*def, generics, "_Vtbl");
            if has_unknown_base {
                methods.combine(&quote! { base__: #name::new::<Identity, Impl, OFFSET>(), });
            } else {
                methods.combine(&quote! { base__: #name::new::<Impl>(), });
            }
        }
        _ => {}
    }

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(writer, def);

    for method in writer.reader.type_def_methods(def) {
        let name = method_names.add(writer, method);
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
                pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: #impl_ident<#generic_names>, const OFFSET: isize>() -> #vtbl_ident<#generic_names> {
                    #(#method_impls)*
                    Self{
                        #methods
                        #(#named_phantoms)*
                    }
                }
                pub fn matches(iid: &::windows_core::GUID) -> bool {
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
                pub fn new<'a, T: #impl_ident>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
                    let this = ::windows_core::ScopedHeap { vtable: &#implvtbl_ident::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                    let this = ::std::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                    unsafe { ::windows_core::ScopedInterface::new(::std::mem::transmute(&this.vtable)) }
                }
            }
        }
    }
}
