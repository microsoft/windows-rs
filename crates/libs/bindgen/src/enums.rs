use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = def.name();
    let ident = gen_ident(name);
    let underlying_type = def.underlying_type();
    let underlying_type = gen_element_name(&underlying_type, gen);
    let is_scoped = def.is_scoped();
    let cfg = gen.type_cfg(def);
    let features = cfg.gen(gen);
    let doc = cfg.gen_doc(gen);

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

    if gen.min_enum && fields.len() > 100 {
        fields.clear();
    }

    let eq = if gen.sys {
        quote! {}
    } else {
        quote! {
            // Unfortunately, Rust requires these to be derived to allow constant patterns.
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
        }
    };

    let mut tokens = if is_scoped || !gen.sys {
        quote! {
            #doc
            #features
            #[repr(transparent)]
            #eq
            pub struct #ident(pub #underlying_type);
        }
    } else {
        quote! {
            #doc
            #features
            pub type #ident = #underlying_type;
        }
    };

    tokens.combine(&if is_scoped {
        let fields = fields.iter().map(|(field_name, value)| {
            quote! {
                pub const #field_name: Self = Self(#value);
            }
        });

        quote! {
            #features
            impl #ident {
                #(#fields)*
            }
        }
    } else if !gen.sys {
        let fields = fields.iter().map(|(field_name, value)| {
            quote! {
                #doc
                #features
                pub const #field_name: #ident = #ident(#value);
            }
        });

        quote! {
            #(#fields)*
        }
    } else {
        let fields = fields.iter().map(|(field_name, value)| {
            quote! {
                #doc
                #features
                pub const #field_name: #ident = #value;
            }
        });

        quote! {
            #(#fields)*
        }
    });

    if is_scoped || !gen.sys {
        tokens.combine(&quote! {
            #features
            impl ::core::marker::Copy for #ident {}
            #features
            impl ::core::clone::Clone for #ident {
                fn clone(&self) -> Self {
                    *self
                }
            }
        });
    }

    if !gen.sys {
        tokens.combine(&quote! {
            #features
            impl ::core::default::Default for #ident {
                fn default() -> Self {
                    Self(0)
                }
            }
        });
    }

    if !gen.sys {
        tokens.combine(&quote! {
            #features
            unsafe impl ::windows::core::Abi for #ident {
                type Abi = Self;
            }
            #features
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_tuple(#name).field(&self.0).finish()
                }
            }
        });

        if def.has_flags() {
            tokens.combine(&quote! {
                impl ::core::ops::BitOr for #ident {
                    type Output = Self;

                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl ::core::ops::BitAnd for #ident {
                    type Output = Self;

                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl ::core::ops::BitOrAssign for #ident {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl ::core::ops::BitAndAssign for #ident {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl ::core::ops::Not for #ident {
                    type Output = Self;

                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
            });
        }

        if def.is_winrt() {
            let signature = Literal::byte_string(def.type_signature().as_bytes());

            tokens.combine(&quote! {
                #features
                unsafe impl ::windows::core::RuntimeType for #ident {
                    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(#signature);
                    type DefaultType = Self;
                }
            });
        }

        tokens.combine(&extensions::gen(def));
    }

    tokens
}
