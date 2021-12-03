use super::*;

pub fn gen_delegate(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_name(def, gen);

    if gen.sys {
        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        gen_win_delegate(def, gen)
    }
}

pub fn gen_win_delegate(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_generic_name(def, gen);
    let phantoms = gen_phantoms(def, gen);
    let constraints = gen_type_constraints(def, gen);

    let mut tokens = quote! {
        #[repr(transparent)]
        pub struct #name(pub ::windows::core::IUnknown, #(#phantoms)*) where #(#constraints)*;
    };

    // tokens.combine(&gen_methods(def, gen));
    tokens.combine(&gen_std_traits(def, gen));
    tokens.combine(&gen_interface_trait(def, gen));
    tokens.combine(&gen_runtime_trait(def, gen));
    tokens.combine(&gen_vtbl(def, gen));

    tokens
}
