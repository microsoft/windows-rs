use super::*;

pub fn writer(writer: &Writer, def: TypeDef) -> TokenStream {
    let type_name = writer.reader.type_def_type_name(def);
    let ident = to_ident(type_name.name);
    let underlying_type = writer.reader.type_def_underlying_type(def);
    let underlying_type = writer.type_name(&underlying_type);

    // TODO: unscoped enums should be removed from metadata
    let is_scoped = writer.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) || writer.reader.has_attribute(def, "ScopedEnumAttribute");

    let cfg = type_def_cfg(writer.reader, def, &[]);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);

    let fields: Vec<(TokenStream, TokenStream)> = writer
        .reader
        .type_def_fields(def)
        .filter_map(|field| {
            if writer.reader.field_flags(field).contains(FieldAttributes::Literal) {
                let field_name = to_ident(writer.reader.field_name(field));
                let constant = writer.reader.field_constant(field).unwrap();
                let value = writer.value(&writer.reader.constant_value(constant));

                Some((field_name, value))
            } else {
                None
            }
        })
        .collect();

    let eq = if writer.sys {
        quote! {}
    } else {
        quote! {
            // Unfortunately, Rust requires these to be derived to allow constant patterns.
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
        }
    };

    let mut tokens = if is_scoped || !writer.sys {
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

    if is_scoped {
        let fields = fields.iter().map(|(field_name, value)| {
            quote! {
                pub const #field_name: Self = Self(#value);
            }
        });

        tokens.combine(&quote! {
            #features
            impl #ident {
                #(#fields)*
            }
        });
    }

    if is_scoped || !writer.sys {
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

    if !writer.sys {
        tokens.combine(&quote! {
            #features
            impl ::core::default::Default for #ident {
                fn default() -> Self {
                    Self(0)
                }
            }
        });
    }

    if !writer.sys {
        let name = type_name.name;
        tokens.combine(&quote! {
            #features
            impl ::windows_core::TypeKind for #ident {
                type TypeKind = ::windows_core::CopyType;
            }
            #features
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_tuple(#name).field(&self.0).finish()
                }
            }
        });

        // Win32 enums use the Flags attribute. WinRT enums don't have the Flags attribute but are paritioned merely based
        // on whether they are signed.
        // TODO: Win32 metadata should just follow WinRT's example here.
        let type_def_is_flags = writer.reader.has_attribute(def, "FlagsAttribute") || (writer.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) && writer.reader.type_def_underlying_type(def) == Type::U32);

        if type_def_is_flags {
            tokens.combine(&quote! {
                #features
                impl #ident {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                #features
                impl ::core::ops::BitOr for #ident {
                    type Output = Self;

                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                #features
                impl ::core::ops::BitAnd for #ident {
                    type Output = Self;

                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                #features
                impl ::core::ops::BitOrAssign for #ident {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                #features
                impl ::core::ops::BitAndAssign for #ident {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                #features
                impl ::core::ops::Not for #ident {
                    type Output = Self;

                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
            });
        }

        if writer.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
            let signature = Literal::byte_string(type_def_signature(writer.reader, def, &[]).as_bytes());

            tokens.combine(&quote! {
                #features
                impl ::windows_core::RuntimeType for #ident {
                    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(#signature);
                }
            });
        }
    }

    tokens
}
