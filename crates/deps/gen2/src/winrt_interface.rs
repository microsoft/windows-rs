use super::*;

pub fn gen_winrt_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        if def.is_exclusive() {
            quote! {}
        } else {
            let name = gen_generic_ident(def.name());

            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        }
    } else {
        quote! {}
    }
}
