use super::*;

#[derive(Clone, Debug)]
pub struct CppStruct {
    pub def: TypeDef,
    pub name: &'static str,
    pub nested: BTreeMap<&'static str, Self>,
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

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &[])
    }

    // A "handle" type is any struct with a single field called "Value" of some primitive type.
    pub fn is_handle(&self, reader: &Reader) -> bool {
        let mut fields = self.def.fields();

        let Some(field) = fields.next() else {
            return false;
        };

        if field.name() != "Value" || !field.field_type(Some(self), reader).is_primitive(reader) {
            return false;
        }

        fields.next().is_none()
    }

    // A `typedef void X;` is represented in metadata as a single-field `Value: void`
    // struct. A C struct can never have a bare `void` field, so this shape is
    // unambiguously a void typedef (possibly chained through another void typedef,
    // e.g. `MENUTEMPLATE -> MENUTEMPLATEA -> void`). It is only ever used behind a
    // pointer, so it is rendered as a plain alias rather than a struct (which could
    // not derive `Copy`/`Default`).
    pub fn is_void_typedef(&self, reader: &Reader) -> bool {
        let mut fields = self.def.fields();

        let Some(field) = fields.next() else {
            return false;
        };

        if field.name() != "Value" {
            return false;
        }

        if fields.next().is_some() {
            return false;
        }

        match field.field_type(Some(self), reader) {
            Type::Void => true,
            Type::CppStruct(ty) => ty.is_void_typedef(reader),
            _ => false,
        }
    }

    pub fn write_cfg(&self, config: &Config) -> TokenStream {
        write_simple_cfg(self, config)
    }

    // A `typedef Y X;` whose target `Y` is a named struct or interface — e.g.
    // `typedef GUID IID;` or `typedef IUnknown *LPUNKNOWN;` — is represented in
    // metadata as a single-field `Value: Y` struct carrying `NativeTypedefAttribute`.
    // Primitive- and void-typed typedefs are already collapsed by `is_handle` and
    // `is_void_typedef`; this covers the remaining non-primitive case, which must
    // render as a transparent alias rather than a (broken) wrapper struct.
    pub fn is_native_typedef(&self, _reader: &Reader) -> bool {
        if self.def.find_attribute("NativeTypedefAttribute").is_none() {
            return false;
        }

        let mut fields = self.def.fields();

        let Some(field) = fields.next() else {
            return false;
        };

        field.name() == "Value" && fields.next().is_none()
    }

    // A native typedef whose underlying type is (possibly through a chain of further
    // native typedefs) a fixed-size array — e.g. `WINBIO_STRING = [u16; 256]`. Such an
    // alias has no `Default` impl, so a struct field of this type cannot derive it.
    fn resolves_to_fixed_array(&self, reader: &Reader) -> bool {
        if !self.is_native_typedef(reader) {
            return false;
        }

        match self
            .def
            .fields()
            .next()
            .unwrap()
            .field_type(Some(self), reader)
        {
            Type::ArrayFixed(..) => true,
            Type::CppStruct(inner) => inner.resolves_to_fixed_array(reader),
            _ => false,
        }
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        if self.is_handle(config.reader) {
            return config.write_cpp_handle(self.def);
        }

        if self.is_void_typedef(config.reader) {
            let name = to_ident(self.name);
            let field = self.def.fields().next().unwrap();
            let ty = match field.field_type(Some(self), config.reader) {
                Type::Void => quote! { core::ffi::c_void },
                ty => ty.write_name(config),
            };
            let arches = write_arches(self.def);
            let cfg = self.write_cfg(config);
            return quote! {
                #arches
                #cfg
                pub type #name = #ty;
            };
        }

        if self.is_native_typedef(config.reader) {
            let name = to_ident(self.name);
            let field = self.def.fields().next().unwrap();
            let ty = field
                .field_type(Some(self), config.reader)
                .write_name(config);
            let arches = write_arches(self.def);
            let cfg = self.write_cfg(config);
            return quote! {
                #arches
                #cfg
                pub type #name = #ty;
            };
        }

        if self.def.fields().next().is_none() {
            if let Some(guid) = self.def.guid_attribute() {
                return config.write_cpp_const_guid(to_ident(self.name), &guid);
            }
        }

        let arches = write_arches(self.def);
        let cfg = self.write_cfg(config);
        self.write_with_cfg(config, &quote! { #arches #cfg })
    }

    fn write_with_cfg(&self, config: &Config, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(self.name);
        let flags = self.def.flags();
        let is_union = flags.contains(TypeAttributes::ExplicitLayout);
        let has_explicit_layout = self.has_explicit_layout(config.reader);
        let has_packing = self.has_packing(config.reader);

        let fields: Vec<_> = self
            .def
            .fields()
            .filter(|field| !field.flags().contains(FieldAttributes::Literal))
            .map(|field| (field.name(), field.field_type(Some(self), config.reader)))
            .collect();

        let is_copyable = self.is_copyable(config.reader);

        let field_config = &config.with_self_ty(self.type_name(), &[]);

        let fields = {
            let fields = fields.iter().map(|(name, ty)| {
                let name = to_ident(name);

                let ty =
                    if !config.bindgen.style.is_sys() && is_union && !ty.is_copyable(config.reader)
                    {
                        let ty = ty.write_default(field_config);
                        quote! { core::mem::ManuallyDrop<#ty> }
                    } else if !config.bindgen.style.is_sys() && ty.is_dropped(config.reader) {
                        if let Type::ArrayFixed(ty, len) = ty {
                            let ty = ty.write_default(field_config);
                            let len = Literal::usize_unsuffixed(*len);
                            quote! { [core::mem::ManuallyDrop<#ty>; #len] }
                        } else {
                            let ty = ty.write_default(field_config);
                            quote! { core::mem::ManuallyDrop<#ty> }
                        }
                    } else {
                        ty.write_default(field_config)
                    };

                quote! { pub #name: #ty, }
            });

            let fields = quote! { #(#fields)* };

            if fields.is_empty() {
                if is_union {
                    quote! {
                        { pub value: u8 }
                    }
                } else {
                    quote! {
                        (pub u8);
                    }
                }
            } else {
                quote! {
                    { #fields }
                }
            }
        };

        let mut derive = DeriveWriter::new(config, self.type_name());
        let mut manual_clone = None;

        if config.bindgen.style.is_sys() || is_copyable {
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

        if !config.bindgen.style.is_sys() && !has_explicit_layout && !has_packing {
            derive.extend(["Debug"]);

            if !self.has_cpp_delegate(config.reader) {
                derive.extend(["PartialEq"]);
            }
        }

        let default = if self.can_derive_default(config) {
            derive.extend(["Default"]);
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

        let repr = if let Some(align) = self.forced_align() {
            // `__declspec(align(N))` raises alignment above the natural field
            // alignment; Rust expresses this as `repr(align(N))`. Forced
            // over-alignment and packing are mutually exclusive by construction.
            let align = Literal::usize_unsuffixed(align);
            quote! { #[repr(C, align(#align))] }
        } else if let Some(layout) = self.def.class_layout() {
            let packing = Literal::usize_unsuffixed(layout.packing_size() as usize);
            quote! { #[repr(C, packed(#packing))] }
        } else {
            quote! { #[repr(C)] }
        };

        let constants = {
            let constants = self.def.fields().filter_map(|f| {
                if f.flags().contains(FieldAttributes::Literal) {
                    if let Some(constant) = f.constant() {
                        let name = to_ident(f.name());
                        let ty = constant.constant_type(config.reader).write_name(config);
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

        let mut nested_sorted: Vec<_> = self.nested.values().collect();
        nested_sorted.sort_by_key(|s| s.name);

        for nested in nested_sorted {
            tokens.combine(nested.write_with_cfg(config, cfg));
        }

        tokens
    }

    fn can_derive_default(&self, config: &Config) -> bool {
        !self.has_explicit_layout(config.reader)
            && !self.def.fields().any(|field| {
                let ty = field.field_type(Some(self), config.reader);

                // Resolve transparent native-typedef aliases to a fixed-size array,
                // which has no derivable `Default`; treat like a directly-spelled array.
                if let Type::CppStruct(inner) = &ty {
                    if inner.resolves_to_fixed_array(config.reader) {
                        return true;
                    }
                }

                if config.bindgen.style.is_sys() {
                    if let Type::CppStruct(ty) = &ty {
                        if ty.is_handle(config.reader)
                            && ty.def.underlying_type_ext(config.reader).is_pointer()
                        {
                            return true;
                        }
                    }

                    // A scoped C++ enum projects to a newtype that does not derive
                    // `Default` in sys mode (matching the published `windows-sys`), so a
                    // struct embedding one cannot derive `Default` either; fall back to
                    // the zeroed impl. Non-scoped enums are bare integer typedefs and
                    // remain fine.
                    if let Type::CppEnum(inner) = &ty {
                        if inner.def.has_attribute("ScopedEnumAttribute") {
                            return true;
                        }
                    }

                    matches!(
                        &ty,
                        Type::ArrayFixed(..)
                            | Type::BSTR
                            | Type::Class(..)
                            | Type::CppInterface(..)
                            | Type::Delegate(..)
                            | Type::Interface(..)
                            | Type::IUnknown
                            | Type::Object
                            | Type::PCSTR
                            | Type::PCWSTR
                            | Type::PSTR
                            | Type::PtrConst(..)
                            | Type::PtrMut(..)
                            | Type::PWSTR
                    )
                } else {
                    matches!(
                        &ty,
                        Type::ArrayFixed(..) | Type::PtrConst(..) | Type::PtrMut(..)
                    )
                }
            })
    }

    pub fn has_cpp_delegate(&self, reader: &Reader) -> bool {
        self.def.fields().any(|field| {
            let ty = field.field_type(Some(self), reader);
            ty.has_cpp_delegate(reader)
        })
    }

    pub fn is_copyable(&self, reader: &Reader) -> bool {
        if matches!(
            self.def.type_name(),
            TypeName::VARIANT | TypeName::PROPVARIANT
        ) {
            return false;
        }

        self.def
            .fields()
            .all(|field| field.field_type(Some(self), reader).is_copyable(reader))
    }

    pub fn has_explicit_layout(&self, reader: &Reader) -> bool {
        self.def.flags().contains(TypeAttributes::ExplicitLayout)
            || self
                .multi_struct_fields(reader)
                .any(|ty| ty.has_explicit_layout(reader))
    }

    pub fn has_packing(&self, reader: &Reader) -> bool {
        self.def.class_layout().is_some()
            || self
                .multi_struct_fields(reader)
                .any(|ty| ty.has_packing(reader))
    }

    // Returns all possible struct field types including arch-specific overloads.
    // This avoids skipping arch-specific definitions of structs that may have
    // different layout or packing requirements.
    fn multi_struct_fields<'a>(&'a self, reader: &'a Reader) -> impl Iterator<Item = Self> + 'a {
        self.def
            .fields()
            .map(move |field| field.field_type(Some(self), reader))
            .filter_map(|ty| match ty {
                Type::CppStruct(ty) => Some(ty),
                Type::ArrayFixed(ty, _) => {
                    if let Type::CppStruct(ty) = *ty {
                        Some(ty)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flat_map(|ty| reader.with_full_name(ty.def.namespace(), ty.def.name()))
            .filter_map(|ty| {
                if let Type::CppStruct(ty) = ty {
                    Some(ty)
                } else {
                    None
                }
            })
            .chain(self.nested.values().cloned())
    }

    pub fn size(&self, reader: &Reader) -> usize {
        if self.def.flags().contains(TypeAttributes::ExplicitLayout) {
            self.def
                .fields()
                .map(|field| field.field_type(Some(self), reader).size(reader))
                .max()
                .unwrap_or(1)
        } else {
            let mut sum = 0;
            for field in self.def.fields() {
                let ty = field.field_type(Some(self), reader);
                let size = ty.size(reader);
                let align = ty.align(reader);
                sum = (sum + (align - 1)) & !(align - 1);
                sum += size;
            }
            sum
        }
    }

    pub fn align(&self, reader: &Reader) -> usize {
        let derived = self
            .def
            .fields()
            .map(|field| field.field_type(Some(self), reader).align(reader))
            .max()
            .unwrap_or(1);

        // Forced over-alignment (`__declspec(align(N))`) can raise the type's
        // alignment above what its fields imply, so it must win when larger.
        self.forced_align()
            .map_or(derived, |forced| forced.max(derived))
    }

    /// The forced over-alignment in bytes recorded by `[AlignmentAttribute(N)]`
    /// (from `__declspec(align(N))` / `alignas(N)`), or `None` when the type uses
    /// its natural field alignment. The winmd `ClassLayout` can only lower
    /// alignment via its packing size, so raised alignment is carried by this
    /// custom attribute instead.
    pub fn forced_align(&self) -> Option<usize> {
        match self
            .def
            .find_attribute("AlignmentAttribute")?
            .value()
            .first()
        {
            Some((_, Value::I32(n))) if *n > 0 => Some(*n as usize),
            _ => None,
        }
    }
}

impl Dependencies for CppStruct {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        for field in self.def.fields() {
            field
                .field_type(Some(self), reader)
                .combine(dependencies, reader);
        }

        if let Some(attribute) = self.def.find_attribute("AlsoUsableForAttribute") {
            if let Some((_, Value::Utf8(type_name))) = attribute.value().first() {
                reader
                    .unwrap_full_name(self.def.namespace(), type_name)
                    .combine(dependencies, reader);
            }
        }
    }
}
