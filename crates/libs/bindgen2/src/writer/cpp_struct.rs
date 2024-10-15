use super::*;

impl Writer {
    pub fn write_cpp_struct(&self, item: &'static CppStruct) -> TokenStream {
        // TODO: do we need to ass cfg into this?
        if item.def.has_attribute("NativeTypedefAttribute") {
            return self.write_cpp_handle(item.def);
        }

        // TODO: there are actually structs with fields and GUIDs like LOGGING_PARAMETERS
        if item.def.fields().next().is_none() {
            if let Some(guid) = item.def.guid_attribute() {
                return self.write_cpp_const_guid(to_ident(item.name()), &guid);
            }
        }

        let mut dependencies = Dependencies::new();

        if self.config.package {
            item.dependencies(&mut dependencies, &self.config);
        }

        let cfg = self.write_cfg(item.def, item.def.namespace(), dependencies, false);
        self.write_with_cfg(item, &cfg)
    }

    fn write_with_cfg(&self, item: &'static CppStruct, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(item.name());
        let flags = item.def.flags();

        let fields: Vec<_> = item
            .def
            .fields()
            .filter(|field| !field.flags().contains(FieldAttributes::Literal))
            .map(|field| (field.name(), field.ty(Some(item))))
            .collect();

        let is_copyable = fields.iter().all(|(_, ty)| ty.is_copyable());

        let fields = {
            let fields = fields.iter().map(|(name, ty)| {
                let name = to_ident(name);
                let ty = self.write_default_name(ty);
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

        if !self.config.sys {
            derive.combine(quote! { Debug, PartialEq, });
        }

        // TODO: add any user-defined derive names

        let type_kind = if self.config.sys {
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

        let default = if self.config.sys {
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

        let repr = if let Some(layout) = item.def.class_layout() {
            let packing = Literal::usize_unsuffixed(layout.packing_size());
            quote! { #[repr(C, packed(#packing))] }
        } else {
            quote! { #[repr(C)] }
        };

        let constants = {
            let constants = item.def.fields().filter_map(|f| {
                if f.flags().contains(FieldAttributes::Literal) {
                    if let Some(constant) = f.constant() {
                        let name = to_ident(f.name());
                        let ty = self.write_name(&constant.ty());
                        let value = self.write_value(&constant.value());

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
