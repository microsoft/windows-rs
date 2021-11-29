use super::*;

pub fn gen_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_ident(def.name());

    if gen.sys {
        if def.is_exclusive() {
            quote! {}
        } else {
            // TODO: Generate a minimal C style interface similar to what MIDL does when compiling for C.
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
