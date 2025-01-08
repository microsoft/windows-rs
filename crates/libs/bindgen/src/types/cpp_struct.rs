use super::*;

#[derive(Clone, Debug)]
pub struct CppStruct {
    pub def: TypeDef,
    pub name: &'static str,
    pub nested: BTreeMap<&'static str, CppStruct>,
}

impl Ord for CppStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.name, self.def).cmp(&(other.name, other.def))
    }
}

impl PartialOrd for CppStruct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CppStruct {
    fn eq(&self, other: &Self) -> bool {
        self.def == other.def
    }
}

impl Eq for CppStruct {}

impl std::hash::Hash for CppStruct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.def.hash(state);
    }
}

impl CppStruct {
    pub fn type_name(&self) -> TypeName {
        TypeName(self.def.namespace(), self.name)
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        self.type_name().write(writer, &[])
    }

    pub fn is_handle(&self) -> bool {
        self.def.has_attribute("NativeTypedefAttribute")
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        if self.is_handle() {
            return writer.write_cpp_handle(self.def);
        }

        if self.def.fields().next().is_none() {
            if let Some(guid) = self.def.guid_attribute() {
                return writer.write_cpp_const_guid(to_ident(self.name), &guid);
            }
        }

        let mut dependencies = TypeMap::new();

        if config().package {
            self.dependencies(&mut dependencies);
        }

        let cfg = writer.write_cfg(self.def, self.def.namespace(), &dependencies, false);
        self.write_with_cfg(writer, &cfg)
    }

    fn write_with_cfg(&self, writer: &Writer, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(self.name);
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

        let is_copyable = self.is_copyable();

        let fields = {
            let fields = fields.iter().map(|(name, ty)| {
                let name = to_ident(name);

                let ty = if !config().sys && is_union && !ty.is_copyable() {
                    let ty = ty.write_default(writer);
                    quote! { core::mem::ManuallyDrop<#ty> }
                } else if !config().sys && ty.is_dropped() {
                    if let Type::ArrayFixed(ty, len) = ty {
                        let ty = ty.write_default(writer);
                        let len = Literal::usize_unsuffixed(*len);
                        quote! { [core::mem::ManuallyDrop<#ty>; #len] }
                    } else {
                        let ty = ty.write_default(writer);
                        quote! { core::mem::ManuallyDrop<#ty> }
                    }
                } else {
                    ty.write_default(writer)
                };

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

        let mut derive = DeriveWriter::new(self.type_name());
        let mut manual_clone = None;

        if config().sys || is_copyable {
            derive.extend(["Clone", "Copy"]);
        } else if !matches!(
            TypeName(self.def.namespace(), self.def.name()),
            TypeName::VARIANT | TypeName::PROPVARIANT
        ) {
            if has_explicit_layout {
                manual_clone = Some(quote! {
                    #cfg
                    impl Clone for #name {
                        fn clone(&self) -> Self {
                            unsafe { core::mem::transmute_copy(self) }
                        }
                    }
                });
            } else if !has_packing {
                derive.extend(["Clone"]);
            }
        }

        if !config().sys && !has_explicit_layout && !has_packing {
            derive.extend(["Debug", "PartialEq"]);
        }

        let default = if config().sys {
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
                        let ty = constant.ty().write_name(writer);
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
            #derive
            pub #struct_or_union #name
            #fields
            #constants
            #manual_clone
            #default
        };

        for nested in self.nested.values() {
            tokens.combine(nested.write_with_cfg(writer, cfg));
        }

        tokens
    }

    pub fn dependencies(&self, dependencies: &mut TypeMap) {
        for field in self.def.fields() {
            field.ty(Some(self)).dependencies(dependencies);
        }

        if let Some(attribute) = self.def.find_attribute("AlsoUsableForAttribute") {
            if let Some((_, Value::Str(type_name))) = attribute.args().first() {
                reader()
                    .unwrap_full_name(self.def.namespace(), type_name)
                    .dependencies(dependencies);
            }
        }
    }

    pub fn is_copyable(&self) -> bool {
        if matches!(
            self.def.type_name(),
            TypeName::VARIANT | TypeName::PROPVARIANT
        ) {
            return false;
        }

        self.def
            .fields()
            .all(|field| field.ty(Some(self)).is_copyable())
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
