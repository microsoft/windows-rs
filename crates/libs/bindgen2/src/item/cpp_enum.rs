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
    pub fn write(&self, writer: &Writer) -> TokenStream {
        let is_scoped = self.def.has_attribute("ScopedEnumAttribute");

        if !is_scoped && writer.config.sys {
            return writer.write_cpp_handle(self.def);
        }

        let name = to_ident(self.def.name());
        let underlying_type = self.def.underlying_type().write(writer);

        let mut derive = quote! { Copy, Clone, };

        if !writer.config.sys {
            derive.combine(quote! { Debug, PartialEq, });
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

        quote! {
            #[repr(transparent)]
            #[derive(#derive)]
            pub struct #name(pub #underlying_type);
            #fields
            #type_kind
        }
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        dependencies.insert(self.def.namespace(), self.def.name());
    }
}
