use super::*;

pub fn gen_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    // TODO: use same representation for unscoped enums
    if gen.sys {
        gen_sys_enum(def, gen)
    } else {
        quote! {}
    }
}

fn gen_sys_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let underlying_type = def.underlying_type();
    let underlying_type = gen_element_name(&underlying_type, gen);

    let fields = def.fields().filter_map(|field| {
        if field.is_literal() {
            let field_name = gen_ident(field.name());
            let constant = field.constant().unwrap();
            let value = gen_constant_value(&constant.value());

            Some((field_name, value))
        } else {
            None
        }
    });

    if def.is_scoped() {
        let fields = fields.map(|(field_name, value)| {
            quote! {
                pub const #field_name: Self = Self(#value);
            }
        });

        quote! {
            #[repr(transparent)]
            pub struct #name(pub #underlying_type);
            impl #name {
                #(#fields)*
            }
            impl ::core::marker::Copy for #name {}
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    } else {
        let fields = fields.map(|(field_name, value)| {
            quote! {
                pub const #field_name: #name = #value;
            }
        });

        quote! {
            pub type #name = #underlying_type;
            #(#fields)*
        }
    }
}
