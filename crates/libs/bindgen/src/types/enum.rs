use super::*;

// The literal-variant constants of an enum (`pub const X: Self = Self(value);`),
// honoring any per-variant filter (`Enum::{A, B}`). Identical for WinRT (`Enum`) and
// Win32/COM (`CppEnum`) enums; the callers differ only in how they wrap the result in an
// `impl` block.
pub fn write_enum_constants(def: TypeDef, config: &Config) -> Vec<TokenStream> {
    let tn = def.type_name();
    def.fields()
        .filter(|field| field.flags().contains(FieldAttributes::Literal))
        .filter(|field| {
            // When the filter lists specific variants for this enum, emit only those.
            if let Some(variant_set) = config.filter.enum_variant_filter(tn.namespace(), tn.name())
            {
                return variant_set.includes(field.name());
            }
            true
        })
        .map(|field| {
            let name = to_ident(field.name());
            let value = field.constant().unwrap().value().write();

            quote! {
                pub const #name: Self = Self(#value);
            }
        })
        .collect()
}

// The bitwise-operator impls shared by flag enums (`BitOr`/`BitAnd`/`BitOrAssign`/
// `BitAndAssign`/`Not` plus `contains`). The guard deciding *whether* to emit differs
// between generators (WinRT keys on a `u32` underlying type, COM on `FlagsAttribute`),
// so the callers gate the call; the emitted tokens are identical.
pub fn write_enum_flags(name: &TokenStream) -> TokenStream {
    quote! {
        impl #name {
            pub const fn contains(&self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }
        }
        impl core::ops::BitOr for #name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl core::ops::BitAnd for #name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
        impl core::ops::BitOrAssign for #name {
            fn bitor_assign(&mut self, other: Self) {
                self.0.bitor_assign(other.0);
            }
        }
        impl core::ops::BitAndAssign for #name {
            fn bitand_assign(&mut self, other: Self) {
                self.0.bitand_assign(other.0);
            }
        }
        impl core::ops::Not for #name {
            type Output = Self;
            fn not(self) -> Self {
                Self(self.0.not())
            }
        }

    }
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Enum {
    pub def: TypeDef,
}

impl Enum {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        let name = to_ident(self.def.name());
        let underlying_type = self.def.underlying_type_ext(config.reader);

        let mut derive = DeriveWriter::new(config, self.type_name());
        derive.extend(["Copy", "Clone"]);

        if config.bindgen.style.derive_std_traits() {
            derive.extend(["Default", "Debug", "PartialEq", "Eq"]);
        }

        let fields = write_enum_constants(self.def, config);

        let flags = if config.bindgen.style.is_sys() || underlying_type != Type::U32 {
            quote! {}
        } else {
            write_enum_flags(&name)
        };

        let underlying_type = underlying_type.write_name(config);

        let win_traits = if !config.bindgen.style.emit_core_traits() {
            quote! {}
        } else {
            let signature = Literal::byte_string(self.runtime_signature(config.reader).as_bytes());

            let name_const = config.write_value_name_const(self.def.type_name());

            quote! {
                impl windows_core::TypeKind for #name {
                    type TypeKind = windows_core::CopyType;
                }
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#signature);
                    #name_const
                }
            }
        };

        quote! {
            #[repr(transparent)]
            #derive
            pub struct #name(pub #underlying_type);
            impl #name {
                #(#fields)*
            }
            #win_traits
            #flags
        }
    }

    pub fn runtime_signature(&self, reader: &Reader) -> String {
        format!(
            "enum({};{})",
            self.def.type_name(),
            self.def
                .underlying_type_ext(reader)
                .runtime_signature(reader)
        )
    }
}
