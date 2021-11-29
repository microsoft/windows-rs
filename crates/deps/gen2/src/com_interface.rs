use super::*;

// TODO: add support (even for windows-sys) behind implement feature to implement COM/WinRT interfaces
// TODO: add skeleton support for calling COM/WinRT interfaces for windows-sys and providing their GUIDs
pub fn gen_com_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        let name: TokenStream = def.name().into();

        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        quote! {}
    }
}
