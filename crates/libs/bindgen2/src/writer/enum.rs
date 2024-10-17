use super::*;

impl Enum {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());
        let underlying_type = self.def.underlying_type().write(writer);

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

        let signature = Literal::byte_string(&self.runtime_signature());

        quote! {
            #[repr(transparent)]
            #[derive(Clone, Debug, Default, PartialEq)]
            pub struct #name(pub #underlying_type);
            impl #name {
                #(#fields)*
            }
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#signature);
            }
        }
    }
}
