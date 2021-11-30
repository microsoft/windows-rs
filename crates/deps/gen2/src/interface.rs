use super::*;

pub fn gen_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_ident(def.name());
    let mut vtbl = name.clone();
    vtbl.push_str("Vtbl");
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

        let mut tokens = quote! {};

        if !is_exclusive {
            tokens.combine(&quote! {
                #[repr(transparent)]
                pub struct #name(pub ::windows::core::IUnknown);
                unsafe impl ::windows::core::Interface for #name {
                    type Vtable = #vtbl;
                    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e365e57_48b2_4160_956f_c7385120bbfc);
                }
            });

            // TODO: add methods
        }

        // TODO: add vtbl

        tokens
    }
}
