use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        if def.default_interface().is_some() {
            let name = gen_type_ident(def, gen);
            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    } else if let Some(_default_interface) = def.default_interface() {
        gen_class(def, gen)
    } else {
        gen_static_class(def, gen)
    }
}

fn gen_class(def: &TypeDef, gen: &Gen) -> TokenStream {
    quote! {}
}

fn gen_static_class(def: &TypeDef, gen: &Gen) -> TokenStream {
    quote! {}
}
