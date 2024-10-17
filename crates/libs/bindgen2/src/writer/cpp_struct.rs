use super::*;

impl CppStruct {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        // TODO: do we need to ass cfg into this?
        if self.def.has_attribute("NativeTypedefAttribute") {
            return writer.write_cpp_handle(self.def);
        }

        // TODO: there are actually structs with fields and GUIDs like LOGGING_PARAMETERS
        if self.def.fields().next().is_none() {
            if let Some(guid) = self.def.guid_attribute() {
                return writer.write_cpp_const_guid(to_ident(self.name()), &guid);
            }
        }

        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies, &writer.config);
        }

        let cfg = writer.write_cfg(self.def, self.def.namespace(), dependencies, false);
        self.write_with_cfg(writer, &cfg)
    }

    fn write_with_cfg(&self, writer: &Writer, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(self.name());
        let flags = self.def.flags();

        let fields: Vec<_> = self
            .def
            .fields()
            .filter(|field| !field.flags().contains(FieldAttributes::Literal))
            .map(|field| (field.name(), field.ty(Some(self))))
            .collect();

        let is_copyable = fields.iter().all(|(_, ty)| ty.is_copyable());

        let fields = {
            let fields = fields.iter().map(|(name, ty)| {
                let name = to_ident(name);
                let ty = ty.write_default(writer);
                quote! { pub #name: #ty, }
            });

            let fields = quote! { #(#fields)* };

            if fields.is_empty() {
                quote! {
                    (pub u8);
                }
            } else {
                quote! {
                    { #fields }
                }
            }
        };

        let mut derive = quote! { Clone, Copy, };

        if !writer.config.sys {
            derive.combine(quote! { Debug, PartialEq, });
        }

        // TODO: add any user-defined derive names

        let type_kind = if writer.config.sys {
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

        let default = if writer.config.sys {
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

        let struct_or_union = if flags.contains(TypeAttributes::ExplicitLayout) {
            quote! { union }
        } else {
            quote! { struct }
        };

        let repr = if let Some(layout) = self.def.class_layout() {
            let packing = Literal::usize_unsuffixed(layout.packing_size());
            quote! { #[repr(C, packed(#packing))] }
        } else {
            quote! { #[repr(C)] }
        };

        let constants = {
            let constants = self.def.fields().filter_map(|f| {
                if f.flags().contains(FieldAttributes::Literal) {
                    if let Some(constant) = f.constant() {
                        let name = to_ident(f.name());
                        let ty = constant.ty().write(writer);
                        let value = constant.value().write();

                        return Some(quote! {
                            pub const #name: #ty = #value;
                        });
                    }
                }

                None
            });

            let mut constants = quote! { #(#constants)* };

            if !constants.is_empty() {
                constants = quote! {
                    #cfg
                    impl #name {
                        #constants
                    }
                };
            }

            constants
        };

        let mut tokens = quote! {
            #repr
            #cfg
            #[derive(#derive)]
            pub #struct_or_union #name
            #fields
            #constants
            #default
            #type_kind
        };

        for nested in self.nested.values() {
            if let Item::CppStruct(item) = nested {
                tokens.combine(item.write_with_cfg(writer, cfg));
            } else {
                panic!();
            }
        }

        tokens
    }
}
