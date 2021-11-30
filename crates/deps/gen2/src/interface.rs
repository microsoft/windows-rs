use super::*;

pub fn gen_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_ident(def.name());
    let is_exclusive = def.is_exclusive();
    let is_class = def.kind() == TypeKind::Class;
    let has_default = def.has_default();

    if gen.sys {
        if is_exclusive {
            quote! {}
        } else {
            // TODO: Generate a minimal C style interface similar to what MIDL does when compiling for C
            // with a matching INameVtbl struct for the vtable (instead of IName_abi) just for consistency
            // with MIDL. Any non-static classes (has_default) will have their types pointing to the default
            // interface Vtbl instead of generating a new one.

            if !is_class || has_default {
                // TODO: should be *const?
                quote! {
                    pub type #name = *mut ::core::ffi::c_void;
                }
            } else {
                quote! {}
            }
        }
    } else {
        //let class_name = format!("{}", def.type_name());

        let mut tokens = if is_exclusive {
            quote! { #[doc(hidden)] }
        } else {
            quote! {}
        };

        tokens.combine(&quote! {
            #[repr(transparent)]
            pub struct #name(pub ::windows::core::IUnknown);
        });

        // TODO: add methods

        tokens
    }
}
