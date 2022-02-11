use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.kind() != TypeKind::Interface || !def.can_implement() {
        return quote! {};
    }

    let type_ident = gen_ident(def.name());
    let impl_ident = type_ident.join("_Impl");
    let vtbl_ident = type_ident.join("_Vtbl");
    let constraints = gen_type_constraints(def, gen);
    let generics = gen_type_generics(def, gen);
    let phantoms = gen_named_phantoms(def, gen);
    let cfg = gen.type_impl_cfg(def);
    let mut requires = quote! {};

    fn gen_required_trait(def: &TypeDef, gen: &Gen) -> TokenStream {
        let name = gen_impl_ident(def, gen);
        let namespace = gen.namespace(def.namespace());
        quote! {
            + #namespace #name
        }
    }

    let mut matches = quote! { iid == &<#type_ident<#(#generics)*> as ::windows::core::Interface>::IID };

    for def in def.vtable_types() {
        if let Type::TypeDef(def) = def {
            requires.combine(&gen_required_trait(&def, gen));

            let name = gen_ident(def.name());
            let namespace = gen.namespace(def.namespace());

            matches.combine(&quote! {
                || iid == &<#namespace #name as ::windows::core::Interface>::IID
            })
        }
    }

    if def.is_winrt() {
        for def in def.required_interfaces() {
            requires.combine(&gen_required_trait(&def, gen));
        }
    }

    let runtime_name = gen_runtime_name(def, &cfg, gen);
    let cfg = cfg.gen(gen);
    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    let method_traits = def.methods().map(|method| {
        let name = method_names.add(&method);
        let signature = gen_impl_signature(def, &method, gen);
        // If it can be implemented but is exclusive and has no return value then
        // it is a Xaml override so give it a default implementation to make it easier
        // to override individual methods for Xaml notifications.
        if def.is_exclusive() && method.signature(&def.generics).return_sig.is_none() {
            quote! {
                fn #name #signature {
                    ::core::result::Result::Ok(())
                }
            }
        } else {
            quote! { fn #name #signature; }
        }
    });

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    let method_impls = def.methods().map(|method| {
        let name = method_names.add(&method);
        let signature = method.signature(&def.generics);
        let vtbl_signature = gen_vtbl_signature(def, &method, gen);

        let invoke_upcall = if def.is_winrt() { gen_winrt_upcall(&signature, quote! { (*this).#name }) } else { gen_win32_upcall(&signature, quote! { (*this).#name }) };

        quote! {
            unsafe extern "system" fn #name<#(#constraints)* Identity: ::windows::core::IUnknownImpl, Impl: #impl_ident<#(#generics)*>, const OFFSET: isize> #vtbl_signature {
                let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
                let this = (*this).get_impl() as *mut Impl;
                #invoke_upcall
            }
        }
    });

    let mut methods = quote! {};

    match def.vtable_types().last() {
        Some(Type::IUnknown) => methods.combine(&quote! { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), }),
        Some(Type::IInspectable) => methods.combine(&quote! { base: ::windows::core::IInspectableVtbl::new::<Identity, #type_ident<#(#generics)*>, OFFSET>(), }),
        Some(Type::TypeDef(def)) => {
            let vtbl = gen_vtbl_ident(def, gen);
            let namespace = gen.namespace(def.namespace());
            methods.combine(&quote! { base: #namespace #vtbl::new::<Identity, Impl, OFFSET>(), });
        }
        _ => {}
    }

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    for method in def.methods() {
        let name = method_names.add(&method);
        methods.combine(&quote! { #name: #name::<#(#generics)* Identity, Impl, OFFSET>, });
    }

    quote! {
        #cfg
        pub trait #impl_ident<#(#generics)*> : Sized #requires where #(#constraints)* {
            #(#method_traits)*
        }
        #runtime_name
        #cfg
        impl<#(#constraints)*> #vtbl_ident<#(#generics)*> {
            pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: #impl_ident<#(#generics)*>, const OFFSET: isize>() -> #vtbl_ident<#(#generics)*> {
                #(#method_impls)*
                Self{
                    #methods
                    #(#phantoms)*
                }
            }
            pub fn matches(iid: &windows::core::GUID) -> bool {
                #matches
            }
        }
    }
}
