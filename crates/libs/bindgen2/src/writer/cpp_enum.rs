use super::*;

impl Writer {
    pub fn write_cpp_enum(&self, item: &CppEnum) -> TokenStream {
        let is_scoped = item.def.has_attribute("ScopedEnumAttribute");

        if !is_scoped && self.config.sys {
            return self.write_cpp_handle(item.def);
        }

        let name = to_ident(item.def.name());
        let underlying_type = self.write_name(&item.def.underlying_type());

        let mut derive = quote! { Copy, Clone, };

        if !self.config.sys {
            derive.combine(quote! { Debug, PartialEq, });
        }

        let fields = if is_scoped {
            let fields = item
                .def
                .fields()
                .filter(|field| field.flags().contains(FieldAttributes::Literal))
                .map(|field| {
                    let name = to_ident(field.name());
                    let value = self.write_value(&field.constant().unwrap().value());

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
        let type_kind = if self.config.sys {
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
}
