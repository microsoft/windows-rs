use super::*;

pub fn gen_class(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        let has_default = def.interface_impls().any(|interface| interface.is_default());

        if has_default {
            let name: TokenStream = def.name().into();

            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    }
}
