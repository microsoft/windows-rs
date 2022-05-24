use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.reader.type_def_kind(def) != TypeKind::Interface || (!gen.component && !gen.reader.type_def_can_implement(def)) {
        return quote! {};
    }

    if gen.reader.type_def_name(def).starts_with("Disp") && gen.reader.type_def_methods(def).next().is_none() {
        return quote! {};
    }

    let generics: &Vec<Type> = &gen.reader.type_def_generics(def).collect();
    let type_ident = to_ident(gen.reader.type_def_name(def));
    let impl_ident = type_ident.join("_Impl");
    let vtbl_ident = type_ident.join("_Vtbl");
    let constraints = gen.generic_constraints(generics);
    let generic_names = gen.generic_names(generics);
    let named_phantoms = gen.generic_named_phantoms(generics);
    let cfg = gen.reader.type_def_cfg_impl(def, generics);
    let features = gen.cfg_features(&cfg);
    let mut requires = quote! {};
    let type_ident = quote! { #type_ident<#generic_names> };

    fn gen_required_trait(gen: &Gen, def: TypeDef, generics: &[Type]) -> TokenStream {
        let name = gen.type_def_name_imp(def, generics, "_Impl");
        quote! {
            + #name
        }
    }

    let mut matches = quote! { iid == &<#type_ident as ::windows::core::Interface>::IID };

    for def in gen.reader.type_def_vtables(def) {
        if let Type::TypeDef((def, generics)) = def {
            requires.combine(&gen_required_trait(gen, def, &generics));
            let name = gen.type_def_name(def, &generics);

            matches.combine(&quote! {
                || iid == &<#name as ::windows::core::Interface>::IID
            })
        }
    }

    if gen.reader.type_def_flags(def).winrt() {
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
        // If it can be implemented but is exclusive and has no return value then
        // it is a Xaml override so give it a default implementation to make it easier
        // to override individual methods for Xaml notifications.
        if !gen.component && gen.reader.type_def_is_exclusive(def) && signature.return_type.is_none() {
            quote! {
                fn #name #signature_tokens {
                    ::core::result::Result::Ok(())
                }
            }
        } else {
            quote! { fn #name #signature_tokens; }
        }
    });

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(gen, def);

    let method_impls = gen.reader.type_def_methods(def).map(|method| {
        let name = method_names.add(gen, method);
        let signature = gen.reader.method_def_signature(method, generics);
        let vtbl_signature = gen.vtbl_signature(def, generics, &signature);

        let invoke_upcall = if gen.reader.type_def_flags(def).winrt() { winrt_methods::gen_upcall(gen, &signature, quote! { this.#name }) } else { com_methods::gen_upcall(gen, &signature, quote! { this.#name }) };

        quote! {
            unsafe extern "system" fn #name<#constraints Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: #impl_ident<#generic_names>, const OFFSET: isize> #vtbl_signature {
                // offset the `this` pointer by `OFFSET` times the size of a pointer and cast it as an IUnknown implementation
                let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
                let this = (*this).get_impl();
                #invoke_upcall
            }
        }
    });

    let mut methods = quote! {};

    match gen.reader.type_def_vtables(def).last() {
        Some(Type::IUnknown) => methods.combine(&quote! { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), }),
        Some(Type::IInspectable) => methods.combine(&quote! { base__: ::windows::core::IInspectableVtbl::new::<Identity, #type_ident, OFFSET>(), }),
        Some(Type::TypeDef((def, generics))) => {
            let name = gen.type_def_name_imp(*def, generics, "_Vtbl");
            methods.combine(&quote! { base__: #name::new::<Identity, Impl, OFFSET>(), });
        }
        _ => {}
    }

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(gen, def);

    for method in gen.reader.type_def_methods(def) {
        let name = method_names.add(gen, method);
        methods.combine(&quote! { #name: #name::<#generic_names Identity, Impl, OFFSET>, });
    }

    quote! {
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
}
