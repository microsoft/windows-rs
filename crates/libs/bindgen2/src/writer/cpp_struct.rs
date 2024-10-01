use super::*;

impl Writer {
    pub fn write_cpp_struct(&self, item: &'static CppStruct) -> TokenStream {
        let name = to_ident(item.def.name());

        let fields: Vec<_> = item
            .def
            .fields()
            .map(|field| (field.name(), field.ty(Some(item))))
            .collect();

        let is_copyable = fields.iter().all(|(_, ty)| ty.is_copyable());

        let derive = quote! { Clone, Debug, PartialEq };
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

        quote! {
            #[repr(C)]
            #[derive(#derive)]
            pub struct #name {
                #(#fields)*
            }
            impl Default for #name {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::#type_kind;
            }
        }
    }
}
