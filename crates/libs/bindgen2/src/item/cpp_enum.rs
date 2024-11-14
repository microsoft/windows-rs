use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CppEnum {
    pub def: TypeDef,
}

impl Ord for CppEnum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.def.name().cmp(other.def.name())
    }
}

impl PartialOrd for CppEnum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppEnum {
    pub fn type_name(&self) -> TypeName<'_> {
        self.def.type_name()
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        self.type_name().write(writer, &[])
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        let is_scoped = self.def.has_attribute("ScopedEnumAttribute");

        if !is_scoped && writer.config.sys {
            return writer.write_cpp_handle(self.def);
        }

        let name = to_ident(self.def.name());
        let underlying_type = self.def.underlying_type().write(writer);

        let mut derive = DeriveWriter::from(["Copy", "Clone"]);

        if !writer.config.sys {
            derive.add(["Default", "Debug", "PartialEq", "Eq"]);
        }

        let fields = if is_scoped {
            let fields = self
                .def
                .fields()
                .filter(|field| field.flags().contains(FieldAttributes::Literal))
                .map(|field| {
                    let name = to_ident(field.name());
                    let value = field.constant().unwrap().value().write();

                    quote! {
                        pub const #name: Self = Self(#value);
                    }
                });

            quote! {
                impl #name {
                    #(#fields)*
                }
            }
        } else {
            quote! {}
        };

        // TODO: is TypeKind really needed on all these Win32 types?
        let type_kind = if writer.config.sys {
            quote! {}
        } else {
            quote! {
                impl windows_core::TypeKind for #name {
                    type TypeKind = windows_core::CopyType;
                }
            }
        };

        let flags = if writer.config.sys || !self.def.has_attribute("FlagsAttribute") {
            quote! {}
        } else {
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
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for #name {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for #name {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }

            }
        };

        quote! {
            #[repr(transparent)]
            #derive
            pub struct #name(pub #underlying_type);
            #fields
            #type_kind
            #flags
        }
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        dependencies.insert(self.def.namespace(), self.def.name());
    }

    pub fn size(&self) -> usize {
        self.def.underlying_type().size()
    }

    pub fn align(&self) -> usize {
        self.def.underlying_type().align()
    }
}
