use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);

    if gen.sys {
        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        gen_win_delegate(def, gen)
    }
}

fn gen_win_delegate(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let phantoms = gen_phantoms(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let method = def.invoke_method();
    let cfg = gen.function_cfg(&method);

    let mut tokens = quote! {
        #cfg
        #[repr(transparent)]
        pub struct #name(pub ::windows::core::IUnknown, #(#phantoms)*) where #(#constraints)*;
        #cfg
        impl<#(#constraints)*> #name {
            
        }
    };

    // tokens.combine(&gen_methods(def, gen));
    tokens.combine(&gen_std_traits(def, &quote!{}, gen));
    tokens.combine(&gen_interface_trait(def, &quote!{}, gen));
    tokens.combine(&gen_runtime_trait(def, &quote!{}, gen));
    tokens.combine(&gen_vtbl(def, gen));

    tokens
}
