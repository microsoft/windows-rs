use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        if def.default_interface().is_some() {
            let name = gen_type_ident(def, gen);
            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    } else {
        gen_class(def, gen)
    }
}

fn gen_class(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let has_default = def.default_interface().is_some();

    let mut tokens = if has_default {
        quote! {
            #[repr(transparent)]
            pub struct #name(::windows::core::IUnknown);
            impl #name {
                // #methods
                // #(#factories)*
            }
        }
    } else {
        quote! {
            pub struct #name {}
            impl #name {
                // #methods
                // #(#factories)*
            }
        }
    };

    tokens.combine(&gen_runtime_name(def, gen));
    tokens
}

fn gen_runtime_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let runtime_name = format!("{}", def.type_name());

    quote! {
        impl ::windows::core::RuntimeName for #name {
            const NAME: &'static str = #runtime_name;
        }
    }
}

