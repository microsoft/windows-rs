use super::*;

impl Writer {
    pub fn write_cpp_struct(&self, item: &'static CppStruct) -> TokenStream {
        let name = to_ident(item.name());

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

        let mut dependencies = Dependencies::new();

        if self.package {
            item.dependencies(&mut dependencies, self.minimal);
        }

        let cfg = self.write_cfg(item.def, item.def.namespace(), dependencies, false);

        let mut tokens = quote! {
            #[repr(C)]
            #cfg
            #[derive(#derive)]
            pub struct #name {
                #(#fields)*
            }
            #cfg
            impl Default for #name {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            #cfg
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::#type_kind;
            }
        };

        for nested in item.nested.values() {
            tokens.combine(self.write_item(nested));
        }

        tokens
    }
}
