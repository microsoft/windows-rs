use super::*;

impl Writer {
    pub fn write_cpp_struct(&self, item: &'static CppStruct) -> TokenStream {
        let mut dependencies = Dependencies::new();

        if self.package {
            item.dependencies(&mut dependencies, self.minimal);
        }

        let cfg = self.write_cfg(item.def, item.def.namespace(), dependencies, false);
        self.write_with_cfg(item, &cfg)
    }

    fn write_with_cfg(&self, item: &'static CppStruct, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(item.name());

        let fields: Vec<_> = item
            .def
            .fields()
            .map(|field| (field.name(), field.ty(Some(item))))
            .collect();

        let is_copyable = fields.iter().all(|(_, ty)| ty.is_copyable());

        let mut derive = quote! { Clone, Copy, };

        if !self.sys {
            derive.combine(quote! { Debug, PartialEq, });
        }

        // TODO: add any user-defined derive names

        let fields = fields.iter().map(|(name, ty)| {
            let name = to_ident(name);
            let ty = self.write_default_name(ty);
            quote! { pub #name: #ty, }
        });

        let type_kind = if self.sys {
            quote! {}
        } else if is_copyable {
            quote! {
               #cfg
               impl windows_core::TypeKind for #name {
                   type TypeKind = windows_core::CopyType;
               }
            }
        } else {
            quote! {
               #cfg
               impl windows_core::TypeKind for #name {
                   type TypeKind = windows_core::CloneType;
               }
            }
        };

        let default = if self.sys {
            quote! {}
        } else {
            quote! {
                #cfg
                impl Default for #name {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
            }
        };

        let mut tokens = quote! {
            #[repr(C)]
            #cfg
            #[derive(#derive)]
            pub struct #name {
                #(#fields)*
            }
            #default
            #type_kind
        };

        for nested in item.nested.values() {
            if let Item::CppStruct(item) = nested {
                tokens.combine(self.write_with_cfg(item, cfg));
            } else {
                panic!();
            }
        }

        tokens
    }
}
