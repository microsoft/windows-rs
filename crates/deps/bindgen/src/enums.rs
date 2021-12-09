use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let underlying_type = def.underlying_type();
    let underlying_type = gen_element_name(&underlying_type, gen);
    let is_scoped = def.is_scoped();

    let mut fields: Vec<(TokenStream, TokenStream)> = def
        .fields()
        .filter_map(|field| {
            if field.is_literal() {
                let field_name = gen_ident(field.name());
                let constant = field.constant().unwrap();
                let value = gen_constant_value(&constant.value());

                Some((field_name, value))
            } else {
                None
            }
        })
        .collect();

    if gen.minimal && fields.len() > 100 {
        fields.clear();
    }

    let mut tokens = if is_scoped {
        let fields = fields.iter().map(|(field_name, value)| {
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
        let fields = fields.iter().map(|(field_name, value)| {
            quote! {
                pub const #field_name: #name = #value;
            }
        });

        quote! {
            pub type #name = #underlying_type;
            #(#fields)*
        }
    };

    if !gen.sys {
        if is_scoped {
            tokens.combine(&quote! {
                unsafe impl ::windows::core::Abi for #name {
                    type Abi = Self;
                }
                impl ::core::cmp::PartialEq for #name {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::core::cmp::Eq for #name {}
            });
        }

        if def.is_winrt() {
            let signature = Literal::byte_string(def.type_signature().as_bytes());

            tokens.combine(&quote! {
                unsafe impl ::windows::core::RuntimeType for #name {
                    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(#signature);
                }
                impl ::windows::core::DefaultType for #name {
                    type DefaultType = Self;
                }
            });
        }
    }

    tokens
}
