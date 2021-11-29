use super::*;

pub fn gen_delegate(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_ident(def.name());

    if gen.sys {
        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        quote! {
            pub type #name = u32;
        }
    }
}
