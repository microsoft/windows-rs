use super::*;

impl Writer {
    pub fn write_cpp_enum(&self, item: &CppEnum) -> TokenStream {
        let is_scoped = item.def.has_attribute("ScopedEnumAttribute");

        if !is_scoped && self.sys {
            return self.write_cpp_handle(item.def);
        }

        let name = to_ident(item.def.name());
        let underlying_type = self.write_name(&item.def.underlying_type());

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

        quote! {
            #[repr(transparent)]
            #[derive(Clone, Debug, Default, PartialEq)]
            pub struct #name(pub #underlying_type);
            #fields
            // TODO: is TypeKind really needed on all these Win32 types?
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::CopyType;
            }
        }
    }
}
