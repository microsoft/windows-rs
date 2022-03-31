use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        gen_sys_handle(def, gen)
    } else {
        gen_win_handle(def, gen)
    }
}

pub fn gen_sys_handle(def: &TypeDef, gen: &Gen) -> TokenStream {
    let ident = gen_ident(def.name());
    let signature = gen_default_type(&underlying_type(def), gen);

    quote! {
        pub type #ident = #signature;
    }
}

pub fn gen_win_handle(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = def.name();
    let ident = gen_ident(def.name());
    let underlying_type = underlying_type(def);
    let signature = gen_default_type(&underlying_type, gen);
    let check = if underlying_type.is_pointer() {
        quote! {
            impl #ident {
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
        }
    } else {
        let invalid = def.invalid_values();

        if !invalid.is_empty() {
            let invalid = invalid.iter().map(|value| {
                let value = Literal::i64_unsuffixed(*value);
                quote! { self.0 == #value as _ }
            });
            quote! {
                impl #ident {
                    pub fn is_invalid(&self) -> bool {
                        #(#invalid)||*
                    }
                }
            }
        } else {
            quote! {}
        }
    };

    let mut tokens = quote! {
        #[repr(transparent)]
        // Unfortunately, Rust requires these to be derived to allow constant patterns.
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
        pub struct #ident(pub #signature);
        #check
        impl ::core::default::Default for #ident {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
        impl ::core::clone::Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for #ident {}
        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(#name).field(&self.0).finish()
            }
        }
        unsafe impl ::windows::core::Abi for #ident {
            type Abi = Self;
        }
    };

    if let Some(dependency) = def.is_convertible_to() {
        let type_name = dependency.type_name();
        let mut dependency = gen.namespace(type_name.namespace());
        dependency.push_str(type_name.name());

        tokens.combine(&quote! {
            impl<'a> ::windows::core::IntoParam<'a, #dependency> for #ident {
                fn into_param(self) -> ::windows::core::Param<'a, #dependency> {
                    ::windows::core::Param::Owned(#dependency(self.0))
                }
            }
        });
    }

    tokens
}

fn underlying_type(def: &TypeDef) -> Type {
    def.fields().next().map(|field| field.get_type(Some(def))).unwrap()
}
