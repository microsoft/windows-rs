use super::*;

impl Writer {
    pub fn write_struct(&self, item: &Struct) -> TokenStream {
        let name = to_ident(item.def.name());

        let fields: Vec<_> = item
            .def
            .fields()
            .map(|field| (field.name(), field.ty(None)))
            .collect();

        let is_copyable = fields.iter().all(|(_, ty)| ty.is_copyable());

        let derive = quote! { Clone, Debug, Default, PartialEq };
        // TODO: add any user-defined derive names

        let fields = fields.iter().map(|(name, ty)| {
            let name = to_ident(name);
            let ty = self.write_default_name(ty);
            quote! { pub #name: #ty, }
        });

        let type_kind = if is_copyable {
            quote! { CopyType }
        } else {
            quote! { CloneType }
        };

        let signature = Literal::byte_string(&item.signature());

        quote! {
            #[repr(C)]
            #[derive(#derive)]
            pub struct #name {
                #(#fields)*
            }
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::#type_kind;
            }
            impl windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#signature);
            }
        }
    }
}
