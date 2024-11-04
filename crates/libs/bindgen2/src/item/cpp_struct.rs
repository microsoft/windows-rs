use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CppStruct {
    pub def: TypeDef,
    pub name: String,
    pub nested: BTreeMap<&'static str, Item>,
}

impl Ord for CppStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: need to do the same for other Cpp types that may have multiple arches
        (self.def.name(), self.def).cmp(&(other.def.name(), other.def))
    }
}

impl PartialOrd for CppStruct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppStruct {
    pub fn name(&self) -> &str {
        if self.name.is_empty() {
            self.def.name()
        } else {
            &self.name
        }
    }

    pub fn is_handle(&self) -> bool {
        self.def.has_attribute("NativeTypedefAttribute")
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        // TODO: do we need to ass cfg into this?
        if self.is_handle() {
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
            self.dependencies(&mut dependencies);
        }

        let cfg = writer.write_cfg(self.def, self.def.namespace(), &dependencies, false);
        self.write_with_cfg(writer, &cfg)
    }

    fn write_with_cfg(&self, writer: &Writer, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(self.name());
        let flags = self.def.flags();
        let is_union = flags.contains(TypeAttributes::ExplicitLayout);
        let has_explicit_layout = self.has_explicit_layout();
        let has_packing = self.has_packing();

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

        if !writer.config.sys && !has_explicit_layout && !has_packing {
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

        let struct_or_union = if is_union {
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

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        let namespace = self.def.namespace();
        if namespace.is_empty() || dependencies.insert(namespace, self.def.name()) {
            for field in self.def.fields() {
                field.ty(Some(self)).dependencies(dependencies);
            }
        }
    }

    pub fn is_blittable(&self) -> bool {
        // TODO: for compat may need to return true for VARIANT and PROPVARIANT
        !matches!(TypeName(self.def.namespace(), self.def.name()), TypeName::VARIANT | TypeName::PROPVARIANT)
    }

    pub fn has_explicit_layout(&self) -> bool {
        self.def.flags().contains(TypeAttributes::ExplicitLayout)
            || self
                .def
                .fields()
                .any(|field| field.ty(Some(self)).has_explicit_layout())
    }

    pub fn has_packing(&self) -> bool {
        self.def.class_layout().is_some()
            || self
                .def
                .fields()
                .any(|field| field.ty(Some(self)).has_packing())
    }

    pub fn size(&self) -> usize {
        if self.def.flags().contains(TypeAttributes::ExplicitLayout) {
            self.def
                .fields()
                .map(|field| field.ty(Some(self)).size())
                .max()
                .unwrap_or(1)
        } else {
            let mut sum = 0;
            for field in self.def.fields() {
                let ty = field.ty(Some(self));
                let size = ty.size();
                let align = ty.align();
                sum = (sum + (align - 1)) & !(align - 1);
                sum += size;
            }
            sum
        }
    }

    pub fn align(&self) -> usize {
        self.def
            .fields()
            .map(|field| field.ty(Some(self)).align())
            .max()
            .unwrap_or(1)
    }
}
