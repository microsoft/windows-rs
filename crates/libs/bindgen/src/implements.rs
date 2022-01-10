use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    let cfg = gen.type_cfg(def);

    match def.kind() {
        TypeKind::Interface => gen_interface(def, &cfg, gen),
        TypeKind::Class => gen_class(def, &cfg, gen),
        _ => quote! {}
    }
}

fn gen_interface(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    // TODO: gen trait for interface and cfg based on all interfaces being featured 
    // and if interface is exclusive then only provide implement trait if "implement_exclusive" is featured.
    // Also cfg should include all method cfgs.

    let type_ident = gen_ident(def.name());
    let impl_ident = type_ident.join("Impl");
    let vtbl_ident = type_ident.join("Vtbl");
    let constraints = gen_type_constraints(def, gen);
    let generics = gen_type_generics(def, gen);
    let phantoms = gen_phantoms(def, gen);
    let mut cfg = cfg.clone();
    let mut requires = vec![];

    // vtable_types includes self at the end so reverse and skip it
    for def in def.vtable_types().iter().rev().skip(1) {
        if let ElementType::TypeDef(def) = def {
            cfg = cfg.union(gen.type_cfg(def));
            requires.push(gen_impl_ident(def, gen));
        }
    }

    if def.is_winrt() {
        for def in def.required_interfaces() {
            cfg = cfg.union(gen.type_cfg(&def));
            requires.push(gen_impl_ident(&def, gen));
        }
    }

    if def.is_exclusive() {
        cfg.features.insert("implement_exclusive");
    }

    let runtime_name = gen_runtime_name(def, &cfg, gen);
    let cfg = cfg.gen(gen);

    let method_traits = def.methods().map(|method| {
        let name = gen_ident(&method.rust_name());
        let signature = gen_impl_signature(def, &method, gen);
        quote! { fn #name #signature; }
    });

    let method_impls = def.methods().map(|method| {
        let name = gen_ident(&method.rust_name());
        let signature = method.signature(&def.generics);
        let vtbl_signature = gen_vtbl_signature(&def, &method, gen);
        let invoke_upcall = gen_winrt_upcall(&signature, quote! { (*this).#name }, gen);

        quote! {
            unsafe extern "system" fn #name<#(#constraints)* Impl: #impl_ident<#(#generics)*>, const OFFSET: isize> #vtbl_signature {
                let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
                #invoke_upcall
            }
        }
    });

    let method_ptrs = def.methods().map(|method| {
        let name = gen_ident(&method.rust_name());
        quote! { #name::<#(#generics)* Impl, IMPL_OFFSET>, }
    });

    quote!{
        #cfg
        pub trait #impl_ident<#(#generics)*> : Sized #(+#requires)* where #(#constraints)* {
            #(#method_traits)*
        }
        
        #runtime_name
        #cfg
        impl<#(#constraints)*> #vtbl_ident<#(#generics)*> {
            pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: #impl_ident<#(#generics)*>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> #vtbl_ident<#(#generics)*> {
                #(#method_impls)*
                Self(
                    ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
                    ::windows::core::AddRef::<Identity, BASE_OFFSET>,
                    ::windows::core::Release::<Identity, BASE_OFFSET>,
                    ::windows::core::GetIids,
                    ::windows::core::GetRuntimeClassName::<#type_ident<#(#generics)*>>,
                    ::windows::core::GetTrustLevel,
                    #(#method_ptrs)*
                    #(#phantoms)*
                )
            }
            pub fn matches(iid: &windows::core::GUID) -> bool {
                // TODO: match this vtable's IID as well as any base IIDs for COM interfaces

                iid == &<#type_ident<#(#generics)*> as ::windows::core::Interface>::IID
            }
        }
    }
}

fn gen_class(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    // TODO: gen trait for classes and cfg based on all interfaces being featured 
    // and only provide implement trait if "implement_exclusive" is featured.
    // Also cfg should include all method cfgs.
    quote!{}
}
