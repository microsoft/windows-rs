use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.sys {
        gen_sys_handle(gen, def)
    } else {
        gen_win_handle(gen, def)
    }
}

pub fn gen_sys_handle(gen: &Gen, def: TypeDef) -> TokenStream {
    let ident = to_ident(gen.reader.type_def_name(def));
    let signature = gen.type_default_name(&gen.reader.type_def_underlying_type(def));

    quote! {
        pub type #ident = #signature;
    }
}

pub fn gen_win_handle(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = gen.reader.type_def_name(def);
    let ident = to_ident(name);
    let underlying_type = gen.reader.type_def_underlying_type(def);
    let signature = gen.type_default_name(&underlying_type);
    let check = if underlying_type.is_pointer() {
        quote! {
            impl #ident {
                pub fn is_invalid(&self) -> bool {
                    self.0.is_null()
                }
            }
        }
    } else {
        let invalid = gen.reader.type_def_invalid_values(def);

        if !invalid.is_empty() {
            let invalid = invalid.iter().map(|value| {
                let literal = Literal::i64_unsuffixed(*value);

                if *value < 0 && underlying_type.is_unsigned() {
                    quote! { self.0 == #literal as _ }
                } else {
                    quote! { self.0 == #literal }
                }
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

    if let Some(dependency) = gen.reader.type_def_usable_for(def) {
        let type_name = gen.reader.type_def_type_name(dependency);
        let mut dependency = gen.namespace(type_name.namespace);
        dependency.push_str(type_name.name);

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
