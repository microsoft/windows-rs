use super::*;

pub fn gen_winrt_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_ident(def.name());

    if gen.sys {
        if def.is_exclusive() {
            quote! {}
        } else {
            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        }
    } else {
        quote! {
            pub type #name = u32;
        }
    }
}
