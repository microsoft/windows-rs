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
    let signature = gen_signature(def, gen);

    quote! {
        pub type #ident = #signature;
    }
}

pub fn gen_win_handle(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = def.name();
    let ident = gen_ident(def.name());
    let signature = gen_signature(def, gen);

    let mut tokens = quote! {
        #[repr(transparent)]
        // Unfortunately, Rust requires these to be derived to allow constant patterns.
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
        pub struct #ident(pub #signature);
        impl #ident {
            pub fn is_invalid(&self) -> bool {
                *self == unsafe { ::core::mem::zeroed() }
            }

            pub fn ok(self) -> ::windows::core::Result<Self> {
                if !self.is_invalid() {
                    Ok(self)
                } else {
                    Err(::windows::core::Error::from_win32())
                }
            }
        }
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

fn gen_signature(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.type_name() == TypeName::HANDLE {
        quote! { *mut ::core::ffi::c_void }
    } else {
        let signature = def.fields().next().map(|field| field.signature(Some(def))).unwrap();
        gen_sig(&signature, gen)
    }
}
