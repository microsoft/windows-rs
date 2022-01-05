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

    let ident = gen_impl_ident(def, gen);
    let constraints = gen_type_constraints(def, gen);
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

    let cfg = cfg.gen(gen);

    let methods = def.methods().map(|method| {
        let name = gen_ident(&method.rust_name());
        let signature = gen_impl_signature(def, &method, gen);
        quote! { fn #name #signature; }
    });

    quote!{
        #cfg
        pub trait #ident : Sized #(+#requires)* where #(#constraints)* {
            #(#methods)*
        }
    }
}

fn gen_class(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    // TODO: gen trait for classes and cfg based on all interfaces being featured 
    // and only provide implement trait if "implement_exclusive" is featured.
    // Also cfg should include all method cfgs.
    quote!{}
}
