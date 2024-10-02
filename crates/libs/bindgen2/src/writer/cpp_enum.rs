use super::*;

impl Writer {
    pub fn write_cpp_enum(&self, item: &CppEnum) -> TokenStream {
        let name = to_ident(item.def.name());
        let underlying_type = self.write_name(&item.def.underlying_type());

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
            #[repr(transparent)]
            #[derive(Clone, Debug, Default, PartialEq)]
            pub struct #name(pub #underlying_type);
            impl #name {
                #(#fields)*
            }
            // TODO: is TypeKind really needed on all these types?
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::CopyType;
            }
        }
    }
}
