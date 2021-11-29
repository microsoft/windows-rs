use super::*;

// TODO: add support (even for windows-sys) behind implement feature to implement COM/WinRT interfaces
// TODO: add skeleton support for calling COM/WinRT interfaces for windows-sys and providing their GUIDs
pub fn gen_com_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name: TokenStream = def.name().into();

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
