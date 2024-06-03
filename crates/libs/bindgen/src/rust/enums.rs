use super::*;
use metadata::HasAttributes;

pub fn writer(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    let type_name = def.type_name();
    let ident = to_ident(type_name.name());
    let underlying_type = def.underlying_type();
    let underlying_type = writer.type_name(&underlying_type);

    // TODO: unscoped enums should be removed from metadata
    let is_scoped = def
        .flags()
        .contains(metadata::TypeAttributes::WindowsRuntime)
        || def.has_attribute("ScopedEnumAttribute");

    let cfg = cfg::type_def_cfg(writer, def, &[]);
    let features = writer.cfg_features(&cfg);

    let fields: Vec<(TokenStream, TokenStream)> = def
        .fields()
        .filter_map(|field| {
            if field.flags().contains(metadata::FieldAttributes::Literal) {
                let field_name = to_ident(field.name());
                let constant = field.constant().unwrap();
                let value = writer.value(&constant.value());

                Some((field_name, value))
            } else {
                None
            }
        })
        .collect();

    let derive = if writer.sys {
        if is_scoped {
            quote! {
                #[derive(Copy, Clone)]
            }
        } else {
            quote! {}
        }
    } else {
        quote! {
            #[derive(PartialEq, Eq, Copy, Clone, Default)]
        }
    };

    let mut tokens = if is_scoped || !writer.sys {
        quote! {
            #features
            #[repr(transparent)]
            #derive
            pub struct #ident(pub #underlying_type);
        }
    } else {
        quote! {
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

    if !writer.sys {
        let name = type_name.name();
        tokens.combine(&quote! {
            #features
            impl windows_core::TypeKind for #ident {
                type TypeKind = windows_core::CopyType;
            }
            #features
            impl core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_tuple(#name).field(&self.0).finish()
                }
            }
        });

        // Win32 enums use the Flags attribute. WinRT enums don't have the Flags attribute but are paritioned merely based
        // on whether they are signed.
        // TODO: Win32 metadata should just follow WinRT's example here.
        let type_def_is_flags = def.has_attribute("FlagsAttribute")
            || (def
                .flags()
                .contains(metadata::TypeAttributes::WindowsRuntime)
                && def.underlying_type() == metadata::Type::U32);

        if type_def_is_flags {
            tokens.combine(&quote! {
                #features
                impl #ident {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                #features
                impl core::ops::BitOr for #ident {
                    type Output = Self;

                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                #features
                impl core::ops::BitAnd for #ident {
                    type Output = Self;

                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                #features
                impl core::ops::BitOrAssign for #ident {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                #features
                impl core::ops::BitAndAssign for #ident {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                #features
                impl core::ops::Not for #ident {
                    type Output = Self;

                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
            });
        }

        if def
            .flags()
            .contains(metadata::TypeAttributes::WindowsRuntime)
        {
            let signature = Literal::byte_string(metadata::type_def_signature(def, &[]).as_bytes());

            tokens.combine(&quote! {
                #features
                impl windows_core::RuntimeType for #ident {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#signature);
                }
            });
        }
    }

    tokens
}
