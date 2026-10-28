use super::*;

#[derive(Clone, Debug)]
pub struct CppStruct {
    pub def: TypeDef,
    pub name: &'static str,
    pub enclosing: Option<TypeName>,
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
    pub fn has_interface_bases(&self, reader: &Reader) -> bool {
        self.def.fields().any(|field| {
            field.has_attribute("NativeBaseAttribute")
                && field
                    .field_type(Some(self), reader)
                    .is_interface_with_reader(reader)
        })
    }

    pub fn nested_type(&self, name: &str) -> Option<Type> {
        self.nested.get(name).cloned().map(Type::CppStruct)
    }

    pub fn definition_types(&self, reader: &Reader) -> Vec<Type> {
        let mut types: Vec<_> = self
            .def
            .fields()
            .map(|field| field.field_type(Some(self), reader))
            .collect();
        types.extend(self.bitfield_types(reader));
        if self.has_mixed_layout(reader) {
            types.push(Type::UnalignedHelper);
        }
        for nested in self.nested.values() {
            types.extend(nested.definition_types(reader));
        }
        types
    }

    pub fn type_name(&self) -> TypeName {
        TypeName(
            self.enclosing
                .map_or(self.def.namespace(), |owner| owner.namespace()),
            self.name,
        )
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        if let Some(owner) = self.enclosing
            && config.self_ty != Some(self.type_name())
        {
            let namespace = config.write_namespace(owner);
            let name = to_ident(self.name);
            return quote! { #namespace #name };
        }
        self.type_name().write(config, &[])
    }

    // Handle-like typedefs have one primitive instance field named Value and no nested types.
    pub fn is_handle(&self, reader: &Reader) -> bool {
        if !self.nested.is_empty() {
            return false;
        }
        let mut fields = self.def.fields();

        let Some(field) = fields.next() else {
            return false;
        };

        if field.name() != "Value"
            || field.flags().contains(FieldAttributes::Literal)
            || member_alignment(field).is_some()
            || !field.field_type(Some(self), reader).is_primitive(reader)
        {
            return false;
        }

        fields.next().is_none()
    }

    // Single-field `Value: void` structs are typedef-void aliases, not real C structs.
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

    // Native typedefs over named types render as transparent aliases, not wrapper structs.
    pub fn is_native_typedef(&self) -> bool {
        if self.def.find_attribute("NativeTypedefAttribute").is_none() {
            return false;
        }

        let mut fields = self.def.fields();

        let Some(field) = fields.next() else {
            return false;
        };

        field.name() == "Value" && fields.next().is_none()
    }

    // Native typedef chains to fixed arrays prevent deriving `Default`.
    fn resolves_to_fixed_array(&self, reader: &Reader) -> bool {
        if !self.is_native_typedef() {
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

    fn write_native_object(&self, config: &Config) -> TokenStream {
        assert!(
            !self.def.flags().contains(TypeAttributes::ExplicitLayout)
                && self.nested.is_empty()
                && self.def.methods().next().is_none()
                && self.forced_align().is_none(),
            "unsupported native object definition: {}",
            self.name
        );
        let name = to_ident(self.name);
        let cfg = self.write_cfg(config);
        let arches = config.write_reached_arches(&Type::CppStruct(self.clone()), self.def);
        let packing = self.def.class_layout().map_or_else(
            || quote! { #[repr(C)] },
            |layout| {
                assert_eq!(
                    layout.class_size(),
                    0,
                    "unsupported native object size: {}",
                    self.name
                );
                if layout.packing_size() == 0 {
                    quote! { #[repr(C)] }
                } else {
                    let packing = Literal::u16_unsuffixed(layout.packing_size());
                    quote! { #[repr(C, packed(#packing))] }
                }
            },
        );
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        for field in self.def.fields() {
            let ty = field.field_type(Some(self), config.reader);
            assert!(
                field.has_attribute("NativeBaseAttribute")
                    && ty.is_interface_with_reader(config.reader)
                    && !field.flags().contains(FieldAttributes::Literal)
                    && !field.flags().contains(FieldAttributes::Static)
                    && field.attributes().count() == 1,
                "native object `{}` requires only interface base storage",
                self.name
            );
            let field = to_ident(field.name());
            fields.push(quote! { #field: *const core::ffi::c_void, });
            let pointer = quote! { core::ptr::addr_of!(self.#field).cast_mut().cast() };
            if config.bindgen.style.is_sys() {
                methods.push(quote! {
                    pub fn #field(&self) -> *mut core::ffi::c_void { #pointer }
                });
            } else {
                let ty = ty.write_name(config);
                let core = config.write_core();
                methods.push(quote! {
                        pub fn #field(&self) -> #core InterfaceRef<'_, #ty> {
                            unsafe {
                                #core InterfaceRef::from_raw(core::ptr::NonNull::new_unchecked(#pointer))
                            }
                        }
                    });
            }
        }
        assert!(
            fields.len() > 1,
            "native object `{}` requires multiple bases",
            self.name
        );
        quote! {
            #arches
            #cfg
            #packing
            pub struct #name { #(#fields)* }
            #arches
            #cfg
            impl #name { #(#methods)* }
        }
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        if self.has_interface_bases(config.reader) {
            return self.write_native_object(config);
        }
        if self.is_handle(config.reader) {
            let cfg = self.write_cfg(config);
            return config.write_cpp_handle(&Type::CppStruct(self.clone()), self.def, &cfg);
        }
        if self.is_void_typedef(config.reader) {
            let name = to_ident(self.name);
            let field = self.def.fields().next().unwrap();
            let ty = match field.field_type(Some(self), config.reader) {
                Type::Void => quote! { core::ffi::c_void },
                ty => ty.write_name(config),
            };
            let arches = config.write_reached_arches(&Type::CppStruct(self.clone()), self.def);
            let cfg = self.write_cfg(config);
            return quote! {
                #arches
                #cfg
                pub type #name = #ty;
            };
        }

        if self.is_native_typedef() {
            let name = to_ident(self.name);
            let field = self.def.fields().next().unwrap();
            let ty = field
                .field_type(Some(self), config.reader)
                .write_name(config);
            let arches = config.write_reached_arches(&Type::CppStruct(self.clone()), self.def);
            let cfg = self.write_cfg(config);
            return quote! {
                #arches
                #cfg
                pub type #name = #ty;
            };
        }

        if self.def.fields().next().is_none()
            && let Some(guid) = self.def.guid_attribute()
        {
            return config.write_cpp_const_guid(to_ident(self.name), &guid);
        }

        let arches = config.write_reached_arches(&Type::CppStruct(self.clone()), self.def);
        let cfg = self.write_cfg(config);
        self.write_with_cfg(config, &quote! { #arches #cfg }, self.def.arches())
    }

    fn write_with_cfg(&self, config: &Config, cfg: &TokenStream, arches: i32) -> TokenStream {
        if self.has_mixed_layout(config.reader) {
            return self.write_mixed_layout(config, cfg, arches);
        }
        let name = to_ident(self.name);
        let flags = self.def.flags();
        let is_union = flags.contains(TypeAttributes::ExplicitLayout);
        let has_explicit_layout = self.has_explicit_layout(config.reader);
        let has_packing = self.has_packing(config.reader);

        let fields: Vec<_> = self
            .def
            .fields()
            .filter(|field| !field.flags().contains(FieldAttributes::Literal))
            .map(|field| {
                (
                    field.name(),
                    field.field_type(Some(self), config.reader),
                    member_alignment(field),
                )
            })
            .collect();

        let is_copyable = self.is_copyable(config.reader);
        let is_eq = fields.iter().all(|(_, ty, _)| ty.is_eq(config.reader));
        let aligned_members = fields.iter().any(|(_, _, alignment)| alignment.is_some());
        assert!(
            !aligned_members || self.def.class_layout().is_none(),
            "member alignment combined with packing is unsupported: {}",
            self.name
        );

        let field_config = &config.with_self_ty(self.type_name(), &[]);

        let fields = {
            let mut names: BTreeSet<String> = if aligned_members {
                fields.iter().map(|(name, _, _)| (*name).into()).collect()
            } else {
                BTreeSet::new()
            };
            let mut layout = TokenStream::new();
            let fields = fields.iter().enumerate().map(|(position, (name, ty, alignment))| {
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

                let padding = if let Some(alignment) = alignment.filter(|_| !is_union) {
                    let mut padding = format!("__padding_{position}");
                    while !names.insert(padding.clone()) {
                        padding.push('_');
                    }
                    let padding = to_ident(&padding);
                    let alignment = Literal::usize_unsuffixed(alignment);
                    // Native padding may be uninitialized even when every real field is valid.
                    quote! {
                        #[doc(hidden)]
                        pub #padding: [core::mem::MaybeUninit<u8>; {
                            let offset = 0usize;
                            #layout
                            let natural = ::core::mem::align_of::<#ty>();
                            let required = if natural < #alignment { #alignment } else { natural };
                            ((offset + required - 1) & !(required - 1))
                                - ((offset + natural - 1) & !(natural - 1))
                        }],
                    }
                } else {
                    TokenStream::new()
                };
                if aligned_members && !is_union {
                    let requested = alignment.map(|alignment| {
                        let alignment = Literal::usize_unsuffixed(alignment);
                        quote! { let alignment = if alignment < #alignment { #alignment } else { alignment }; }
                    });
                    layout.extend(quote! {
                        let alignment = ::core::mem::align_of::<#ty>();
                        #requested
                        let offset = ((offset + alignment - 1) & !(alignment - 1))
                            + ::core::mem::size_of::<#ty>();
                    });
                }
                quote! { #padding pub #name: #ty, }
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
        } else if has_explicit_layout {
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

        if !config.bindgen.style.is_sys()
            && !has_explicit_layout
            && !has_packing
            && !self.has_member_alignment(config.reader)
        {
            derive.extend(["Debug"]);

            if !self.has_cpp_delegate(config.reader) {
                derive.extend(["PartialEq"]);

                if is_eq {
                    derive.extend(["Eq"]);
                }
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
            // alignment; mixed packing/alignment uses field-local storage instead.
            let align = Literal::usize_unsuffixed(align);
            quote! { #[repr(C, align(#align))] }
        } else if let Some(layout) = self.def.class_layout() {
            let packing = Literal::usize_unsuffixed(layout.packing_size() as usize);
            quote! { #[repr(C, packed(#packing))] }
        } else {
            quote! { #[repr(C)] }
        };

        let constants = self.write_constants(config, cfg);

        let bitfields = self.write_bitfield_accessors(config, cfg, arches);

        let mut tokens = quote! {
            #repr
            #cfg
            #derive
            pub #struct_or_union #name
            #fields
            #constants
            #bitfields
            #manual_clone
            #default
        };

        let mut nested_sorted: Vec<_> = self.nested.values().collect();
        nested_sorted.sort_by_key(|s| s.name);

        for nested in nested_sorted {
            tokens.combine(nested.write_with_cfg(config, cfg, arches));
        }

        tokens
    }

    fn write_constants(&self, config: &Config, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(self.name);
        let constants = self
            .def
            .fields()
            .filter(|field| {
                field.flags().contains(FieldAttributes::Literal) && field.constant().is_some()
            })
            .map(|field| {
                CppConst {
                    namespace: self.type_name().namespace(),
                    field,
                    arches: field.arches(),
                    is_enum_member: false,
                }
                .write_with_enclosing(config, Some(self))
            });
        let constants = quote! { #(#constants)* };
        if constants.is_empty() {
            constants
        } else {
            quote! { #cfg impl #name { #constants } }
        }
    }

    fn can_derive_default(&self, config: &Config) -> bool {
        !self.has_explicit_layout(config.reader)
            && !self.has_member_alignment(config.reader)
            && !self.def.fields().any(|field| {
                let ty = field.field_type(Some(self), config.reader);

                // Resolve transparent native-typedef aliases to a fixed-size array,
                // which has no derivable `Default`; treat like a directly-spelled array.
                if let Type::CppStruct(inner) = &ty
                    && inner.resolves_to_fixed_array(config.reader)
                {
                    return true;
                }

                if config.bindgen.style.is_sys()
                    && let Type::CppEnum(inner) = &ty
                    && inner.def.has_attribute("ScopedEnumAttribute")
                {
                    return true;
                }
                matches!(&ty, Type::ArrayFixed(..))
            })
    }

    pub(super) fn has_mixed_layout(&self, reader: &Reader) -> bool {
        self.def
            .class_layout()
            .is_some_and(|layout| layout.packing_size() != 0)
            && self.has_required_alignment(reader)
    }

    fn has_required_alignment(&self, reader: &Reader) -> bool {
        self.forced_align().is_some()
            || self
                .multi_struct_fields(reader)
                .any(|field| field.has_required_alignment(reader))
    }

    fn write_mixed_layout(&self, config: &Config, cfg: &TokenStream, arches: i32) -> TokenStream {
        let arches = if arches == 0 { 7 } else { arches };
        let name = to_ident(self.name);
        let field_config = &config.with_self_ty(self.type_name(), &[]);
        let is_union = self.def.flags().contains(TypeAttributes::ExplicitLayout);
        let keyword = if is_union {
            quote! { union }
        } else {
            quote! { struct }
        };
        let mut variants: Vec<(String, TokenStream, i32)> = Vec::new();
        for arch in [1, 2, 4].into_iter().filter(|arch| arches & arch != 0) {
            let record_layout = native_layout(&Type::CppStruct(self.clone()), config.reader, arch);
            let alignment = Literal::usize_unsuffixed(record_layout.alignment);
            let mut names: BTreeSet<String> =
                self.def.fields().map(|field| field.name().into()).collect();
            let mut fields = TokenStream::new();
            let mut end = 0;
            for (position, field) in self
                .def
                .fields()
                .filter(|field| !field.flags().contains(FieldAttributes::Literal))
                .enumerate()
            {
                let ty = field.field_type(Some(self), config.reader);
                let layout = native_layout(&ty, config.reader, arch);
                let offset = record_layout.offsets[position];
                if offset > end {
                    let mut padding = format!("__padding_{position}");
                    while !names.insert(padding.clone()) {
                        padding.push('_');
                    }
                    let padding = to_ident(&padding);
                    let size = Literal::usize_unsuffixed(offset - end);
                    fields.extend(quote! {
                        #[doc(hidden)]
                        pub #padding: [core::mem::MaybeUninit<u8>; #size],
                    });
                }
                let field_name = to_ident(field.name());
                let mut storage = ty.write_default(field_config);
                if record_layout.alignment < layout.alignment
                    || !offset.is_multiple_of(layout.alignment)
                {
                    let helper = Type::UnalignedHelper.type_name().write(config, &[]);
                    storage = quote! { #helper<#storage, { ::core::mem::size_of::<#storage>() }> };
                }
                if !config.bindgen.style.is_sys() && !ty.is_copyable(config.reader) {
                    storage = quote! { core::mem::ManuallyDrop<#storage> };
                }
                fields.extend(quote! { pub #field_name: #storage, });
                end = end.max(offset + layout.size);
            }
            let copy = (config.bindgen.style.is_sys() || self.is_copyable(config.reader))
                .then(|| quote! { #[derive(Clone, Copy)] });
            let body = quote! {
                #[repr(C, align(#alignment))]
                #copy
                pub #keyword #name { #fields }
            };
            let key = body.to_string();
            if let Some((_, _, bits)) = variants.iter_mut().find(|(text, _, _)| *text == key) {
                *bits |= arch;
            } else {
                variants.push((key, body, arch));
            }
        }
        let mut output = TokenStream::new();
        for (_, body, bits) in variants {
            let gate = (bits != arches).then(|| write_arch_bits(bits));
            output.extend(quote! { #cfg #gate #body });
        }
        output.extend(quote! {
            #cfg
            impl Default for #name {
                fn default() -> Self { unsafe { core::mem::zeroed() } }
            }
        });
        output.extend(self.write_constants(config, cfg));
        output.extend(self.write_bitfield_accessors(config, cfg, arches));
        for nested in self.nested.values() {
            output.extend(nested.write_with_cfg(config, cfg, arches));
        }
        output
    }
}

struct NativeLayout {
    size: usize,
    alignment: usize,
    required: usize,
    offsets: Vec<usize>,
}

fn align_up(size: usize, alignment: usize) -> usize {
    size.div_ceil(alignment) * alignment
}

fn native_layout(ty: &Type, reader: &Reader, arch: i32) -> NativeLayout {
    let pointer = if arch == 1 { 4 } else { 8 };
    let scalar = |size, alignment| NativeLayout {
        size,
        alignment,
        required: 0,
        offsets: Vec::new(),
    };
    match ty {
        Type::I8 | Type::U8 | Type::Bool => scalar(1, 1),
        Type::I16 | Type::U16 | Type::Char => scalar(2, 2),
        Type::I32
        | Type::U32
        | Type::F32
        | Type::HRESULT
        | Type::BOOL
        | Type::NTSTATUS
        | Type::RPC_STATUS => scalar(4, 4),
        Type::I64 | Type::U64 | Type::F64 | Type::Ptr64(..) => scalar(8, 8),
        Type::GUID => scalar(16, 4),
        Type::PtrMut(..)
        | Type::PtrConst(..)
        | Type::ISize
        | Type::USize
        | Type::FnPtr(..)
        | Type::CppDelegate(..)
        | Type::CppInterface(..)
        | Type::PSTR
        | Type::PCSTR
        | Type::PWSTR
        | Type::PCWSTR
        | Type::IUnknown
        | Type::BSTR => scalar(pointer, pointer),
        Type::ArrayFixed(element, count) => {
            let mut layout = native_layout(element, reader, arch);
            layout.size *= count;
            layout
        }
        Type::CppStruct(record) => {
            let full_name = record.def.full_name();
            let variants: Vec<_> = reader
                .with_full_name(&full_name.namespace, &full_name.name)
                .filter_map(|ty| match ty {
                    Type::CppStruct(record)
                        if record.def.arches() == 0 || record.def.arches() & arch != 0 =>
                    {
                        Some(record)
                    }
                    _ => None,
                })
                .collect();
            assert!(
                variants.len() <= 1,
                "ambiguous native layout for {full_name:?}"
            );
            let record = variants.first().unwrap_or(record);
            assert!(
                record.def.arches() == 0 || record.def.arches() & arch != 0,
                "unavailable native layout for {full_name:?}"
            );
            if record.is_native_typedef() {
                return native_layout(
                    &record
                        .def
                        .fields()
                        .next()
                        .unwrap()
                        .field_type(Some(record), reader),
                    reader,
                    arch,
                );
            }
            let packing = record
                .def
                .class_layout()
                .map(|layout| layout.packing_size() as usize)
                .filter(|packing| *packing != 0)
                .unwrap_or(usize::MAX);
            let mut result = NativeLayout {
                size: 0,
                alignment: 1,
                required: record.forced_align().unwrap_or(0),
                offsets: Vec::new(),
            };
            let union = record.def.flags().contains(TypeAttributes::ExplicitLayout);
            for field in record
                .def
                .fields()
                .filter(|field| !field.flags().contains(FieldAttributes::Literal))
            {
                let field_layout =
                    native_layout(&field.field_type(Some(record), reader), reader, arch);
                let required = field_layout
                    .required
                    .max(member_alignment(field).unwrap_or(0));
                let alignment = field_layout.alignment.min(packing).max(required);
                let offset = if union {
                    0
                } else {
                    align_up(result.size, alignment)
                };
                result.offsets.push(offset);
                result.size = result.size.max(offset + field_layout.size);
                result.alignment = result.alignment.max(alignment);
                result.required = result.required.max(required);
            }
            result.alignment = result.alignment.max(result.required);
            result.size = align_up(result.size.max(1), result.alignment);
            result
        }
        Type::CppEnum(enumeration) => {
            let full_name = enumeration.def.full_name();
            let variants: Vec<_> = reader
                .with_full_name(&full_name.namespace, &full_name.name)
                .filter_map(|ty| match ty {
                    Type::CppEnum(enumeration)
                        if enumeration.def.arches() == 0
                            || enumeration.def.arches() & arch != 0 =>
                    {
                        Some(enumeration)
                    }
                    _ => None,
                })
                .collect();
            assert!(
                variants.len() <= 1,
                "ambiguous native enum layout for {full_name:?}"
            );
            let enumeration = variants.first().unwrap_or(enumeration);
            native_layout(&enumeration.def.underlying_type_ext(reader), reader, arch)
        }
        _ => panic!("unsupported native layout: {ty:?}"),
    }
}
impl CppStruct {
    pub fn has_cpp_delegate(&self, reader: &Reader) -> bool {
        self.def.fields().any(|field| {
            let ty = field.field_type(Some(self), reader);
            ty.has_cpp_delegate(reader)
        })
    }

    pub fn is_copyable(&self, reader: &Reader) -> bool {
        self.def
            .fields()
            .all(|field| field.field_type(Some(self), reader).is_copyable(reader))
    }

    pub fn has_explicit_layout(&self, reader: &Reader) -> bool {
        self.def.flags().contains(TypeAttributes::ExplicitLayout)
            || self
                .multi_struct_fields(reader)
                .chain(self.nested.values().cloned())
                .any(|ty| ty.has_explicit_layout(reader))
    }

    pub fn has_packing(&self, reader: &Reader) -> bool {
        self.def.class_layout().is_some()
            || self
                .multi_struct_fields(reader)
                .chain(self.nested.values().cloned())
                .any(|ty| ty.has_packing(reader))
    }

    fn has_member_alignment(&self, reader: &Reader) -> bool {
        self.def
            .fields()
            .any(|field| member_alignment(field).is_some())
            || self
                .multi_struct_fields(reader)
                .any(|ty| ty.has_member_alignment(reader))
    }

    // Include arch-specific overloads when checking layout and packing.
    fn multi_struct_fields<'a>(&'a self, reader: &'a Reader) -> impl Iterator<Item = Self> + 'a {
        self.def
            .fields()
            .map(move |field| field.field_type(Some(self), reader))
            .filter_map(|mut ty| {
                loop {
                    match ty {
                        Type::CppStruct(ty) => return Some(ty),
                        Type::ArrayFixed(inner, _) => ty = *inner,
                        _ => return None,
                    }
                }
            })
            .flat_map(|ty| {
                let name = ty.def.full_name();
                reader.with_full_name(&name.namespace, &name.name)
            })
            .filter_map(|ty| {
                if let Type::CppStruct(ty) = ty {
                    Some(ty)
                } else {
                    None
                }
            })
    }

    pub fn size(&self, reader: &Reader) -> usize {
        let size = if self.def.flags().contains(TypeAttributes::ExplicitLayout) {
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
                let align = ty.align(reader).max(member_alignment(field).unwrap_or(1));
                sum = (sum + (align - 1)) & !(align - 1);
                sum += size;
            }
            sum
        };
        if self
            .def
            .fields()
            .any(|field| member_alignment(field).is_some())
        {
            let align = self.align(reader);
            (size + align - 1) & !(align - 1)
        } else {
            size
        }
    }

    pub fn align(&self, reader: &Reader) -> usize {
        let derived = self
            .def
            .fields()
            .map(|field| field.field_type(Some(self), reader).align(reader))
            .max()
            .unwrap_or(1);

        // Forced over-alignment can exceed the field-derived alignment.
        self.forced_align()
            .map_or(derived, |forced| forced.max(derived))
    }

    /// Required over-alignment from record and member attributes.
    pub fn forced_align(&self) -> Option<usize> {
        let record = match self
            .def
            .find_attribute("AlignmentAttribute")
            .and_then(|attribute| attribute.value().into_iter().next())
        {
            Some((_, Value::I32(n))) if n > 0 => Some(n as usize),
            _ => None,
        };
        record
            .into_iter()
            .chain(self.def.fields().filter_map(member_alignment))
            .max()
    }

    pub fn bitfield_types(&self, reader: &Reader) -> impl Iterator<Item = Type> {
        self.def.fields().flat_map(move |field| {
            field
                .attributes()
                .filter(|attribute| attribute.name() == "NativeBitfieldAttribute")
                .filter_map(move |attribute| bitfield_member_type(&attribute.value(), self, reader))
        })
    }

    /// Generates typed accessors for coalesced C bit-fields in non-sys projections.
    fn write_bitfield_accessors(
        &self,
        config: &Config,
        cfg: &TokenStream,
        arches: i32,
    ) -> TokenStream {
        if config.bindgen.style.is_sys()
            || !self
                .def
                .fields()
                .any(|field| field.has_attribute("NativeBitfieldAttribute"))
        {
            return quote! {};
        }

        let arches = if arches == 0 { 7 } else { arches };
        let mut variants: Vec<(String, TokenStream, i32)> = Vec::new();
        for arch in [1, 2, 4].into_iter().filter(|arch| arches & arch != 0) {
            let tokens = self.write_bitfield_accessors_for_arch(config, arch);
            if tokens.is_empty() {
                continue;
            }
            let text = tokens.to_string();
            if let Some((_, _, bits)) = variants.iter_mut().find(|(key, _, _)| *key == text) {
                *bits |= arch;
            } else {
                variants.push((text, tokens, arch));
            }
        }
        let name = to_ident(self.name);
        let variants = variants.into_iter().map(|(_, tokens, bits)| {
            let gate = (bits != arches).then(|| write_arch_bits(bits));
            quote! {
                #cfg
                #gate
                impl #name {
                    #tokens
                }
            }
        });
        quote! { #(#variants)* }
    }

    fn write_bitfield_accessors_for_arch(&self, config: &Config, arch: i32) -> TokenStream {
        let mut accessors = Vec::new();
        let record_layout = self
            .has_mixed_layout(config.reader)
            .then(|| native_layout(&Type::CppStruct(self.clone()), config.reader, arch));
        for (position, field) in self
            .def
            .fields()
            .filter(|field| !field.flags().contains(FieldAttributes::Literal))
            .enumerate()
        {
            let bitfields: Vec<_> = field
                .attributes()
                .filter(|attr| attr.name() == "NativeBitfieldAttribute")
                .collect();

            if bitfields.is_empty() {
                continue;
            }

            let field_ident = to_ident(field.name());
            let storage = field.field_type(Some(self), config.reader);
            let (scalar, wrappers) = bitfield_scalar(&storage, config, arch);
            let backing = BitfieldBacking::from_type(&scalar).unwrap_or_else(|| {
                panic!(
                    "bitfield storage `{}.{}.{}` is not an integer: {storage:?}",
                    self.def.namespace(),
                    self.def.full_name().name,
                    field.name()
                )
            });
            let is_union = self.def.flags().contains(TypeAttributes::ExplicitLayout);
            let unaligned = record_layout.as_ref().is_some_and(|record| {
                let layout = native_layout(&storage, config.reader, arch);
                record.alignment < layout.alignment
                    || !record.offsets[position].is_multiple_of(layout.alignment)
            });
            let mut read = if unaligned {
                quote! { (unsafe { self.#field_ident.read() }) }
            } else if is_union {
                quote! { (unsafe { self.#field_ident }) }
            } else {
                quote! { self.#field_ident }
            };
            for _ in &wrappers {
                read = quote! { #read.0 };
            }

            for attr in bitfields {
                let values = attr.value();
                let (
                    Some((_, Value::Utf8(member))),
                    Some((_, Value::I64(offset))),
                    Some((_, Value::I64(length))),
                ) = (values.first(), values.get(1), values.get(2))
                else {
                    panic!("invalid NativeBitfieldAttribute on `{}`", field.name());
                };

                let offset = u32::try_from(*offset).unwrap();
                let width = u32::try_from(*length).unwrap();
                assert!(
                    width > 0
                        && offset
                            .checked_add(width)
                            .is_some_and(|end| end <= backing.bits),
                    "bitfield `{member}` exceeds its backing type"
                );
                let member_type = bitfield_member_type(&values, self, config.reader);
                if member_type.is_some()
                    || !wrappers.is_empty()
                    || is_union
                    || backing.pointer
                    || unaligned
                {
                    let ty = member_type.unwrap_or_else(|| {
                        if width == 1 {
                            Type::Bool
                        } else {
                            scalar.clone()
                        }
                    });
                    accessors.push(backing.write_typed_accessor(
                        config,
                        &field_ident,
                        (&read, &wrappers, unaligned),
                        (member, &ty),
                        (offset, width),
                        arch,
                    ));
                } else {
                    accessors.push(backing.write_accessor(&field_ident, member, offset, width));
                }
            }
        }

        quote! { #(#accessors)* }
    }
}

fn member_alignment(field: Field) -> Option<usize> {
    let attribute = field.find_attribute("AlignmentAttribute")?;
    let values = attribute.value();
    let [(_, Value::I32(alignment))] = values.as_slice() else {
        panic!("invalid member alignment on `{}`", field.name());
    };
    let alignment = usize::try_from(*alignment).unwrap();
    assert!(
        alignment.is_power_of_two() && alignment <= u16::MAX as usize,
        "unsupported member alignment on `{}`",
        field.name()
    );
    Some(alignment)
}

fn bitfield_member_type(
    values: &[(String, Value)],
    enclosing: &CppStruct,
    reader: &Reader,
) -> Option<Type> {
    let (_, value) = values.iter().find(|(name, _)| name == "MemberType")?;
    let Value::TypeName(name) = value else {
        panic!("NativeBitfieldAttribute.MemberType must be System.Type");
    };
    let ty = name
        .primitive()
        .unwrap_or_else(|| windows_metadata::Type::ValueName(name.clone()));
    Some(Type::from_metadata_type(&ty, Some(enclosing), &[], reader))
}

fn bitfield_scalar(ty: &Type, config: &Config, arch: i32) -> (Type, Vec<TokenStream>) {
    let mut scalar = ty.clone();
    let mut wrappers = Vec::new();
    let mut seen = HashSet::new();
    loop {
        let def = match &scalar {
            Type::CppEnum(ty) => Some(ty.def),
            Type::CppStruct(ty) => Some(ty.def),
            _ => None,
        };
        if let Some(def) = def {
            let mut variants = config
                .reader
                .with_full_name(def.namespace(), def.name())
                .filter(|candidate| {
                    let bits = match candidate {
                        Type::CppEnum(ty) => ty.def.arches(),
                        Type::CppStruct(ty) => ty.def.arches(),
                        _ => return false,
                    };
                    bits == 0 || bits & arch != 0
                });
            if let Some(variant) = variants.next() {
                assert!(
                    variants.next().is_none(),
                    "ambiguous bitfield type for architecture {arch}: {scalar:?}"
                );
                scalar = variant;
            } else {
                assert!(
                    def.arches() == 0 || def.arches() & arch != 0,
                    "bitfield type unavailable for architecture {arch}: {scalar:?}"
                );
            }
        }
        assert!(seen.insert(scalar.clone()), "cyclic bitfield type");
        let wrap = match &scalar {
            Type::CppEnum(ty) => ty.def.has_attribute("ScopedEnumAttribute"),
            Type::CppStruct(ty) if ty.is_handle(config.reader) || ty.is_native_typedef() => {
                !config.typedef_emits_bare(ty.def)
            }
            Type::BOOL | Type::HRESULT | Type::NTSTATUS | Type::RPC_STATUS => {
                !config.bindgen.uses_inline_core_types()
            }
            _ => return (scalar, wrappers),
        };
        if wrap {
            wrappers.push(scalar.write_name(config));
        }
        scalar = scalar.underlying_type(config.reader);
    }
}

/// Integer type backing a coalesced bit-field.
struct BitfieldBacking {
    prim: TokenStream,
    uprim: TokenStream,
    // Pointer widths are checked by generated code on the consumer target.
    bits: u32,
    signed: bool,
    pointer: bool,
}

impl BitfieldBacking {
    fn from_type(ty: &Type) -> Option<Self> {
        let (prim, uprim, bits, signed) = match ty {
            Type::U8 => (quote! { u8 }, quote! { u8 }, 8, false),
            Type::I8 => (quote! { i8 }, quote! { u8 }, 8, true),
            Type::U16 => (quote! { u16 }, quote! { u16 }, 16, false),
            Type::I16 => (quote! { i16 }, quote! { u16 }, 16, true),
            Type::U32 => (quote! { u32 }, quote! { u32 }, 32, false),
            Type::I32 => (quote! { i32 }, quote! { u32 }, 32, true),
            Type::U64 => (quote! { u64 }, quote! { u64 }, 64, false),
            Type::I64 => (quote! { i64 }, quote! { u64 }, 64, true),
            Type::USize => (quote! { usize }, quote! { usize }, 64, false),
            Type::ISize => (quote! { isize }, quote! { usize }, 64, true),
            _ => return None,
        };
        Some(Self {
            prim,
            uprim,
            bits,
            signed,
            pointer: matches!(ty, Type::USize | Type::ISize),
        })
    }

    fn same_width(&self, other: &Self) -> bool {
        self.bits == other.bits && self.pointer == other.pointer
    }

    /// Emits the get/set pair for one logical bit-field member.
    fn write_accessor(
        &self,
        field: &TokenStream,
        member: &str,
        offset: u32,
        width: u32,
    ) -> TokenStream {
        let getter = to_ident(member);
        let setter = to_ident(&format!("set_{member}"));
        let prim = &self.prim;

        if width == 1 {
            // Single-bit members project as `bool`.
            let get_body = if offset == 0 {
                quote! { self.#field & 1 != 0 }
            } else {
                let o = Literal::u32_unsuffixed(offset);
                quote! { (self.#field >> #o) & 1 != 0 }
            };
            let (clear, place) = if offset == 0 {
                (quote! { !1 }, quote! { value as #prim })
            } else {
                let o = Literal::u32_unsuffixed(offset);
                (quote! { !(1 << #o) }, quote! { (value as #prim) << #o })
            };
            return quote! {
                pub fn #getter(&self) -> bool {
                    #get_body
                }
                pub fn #setter(&mut self, value: bool) {
                    self.#field = (self.#field & #clear) | (#place);
                }
            };
        }

        // Signed backings use arithmetic shifts to sign-extend the isolated member bits.
        let hi = self.bits - offset - width;
        let lo = self.bits - width;
        let get_body = match (hi, lo) {
            (0, 0) => quote! { self.#field },
            (0, _) => {
                let lo = Literal::u32_unsuffixed(lo);
                quote! { self.#field >> #lo }
            }
            _ => {
                let hi = Literal::u32_unsuffixed(hi);
                let lo = Literal::u32_unsuffixed(lo);
                quote! { (self.#field << #hi) >> #lo }
            }
        };

        let mask_val: u64 = if width >= 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };
        let mask = Literal::u64_unsuffixed(mask_val);

        // Signed backings use unsigned masks so literals never overflow.
        let clear = if offset == 0 {
            quote! { !#mask }
        } else {
            let o = Literal::u32_unsuffixed(offset);
            quote! { !(#mask << #o) }
        };
        let setter_body = if self.signed {
            let uprim = &self.uprim;
            let place = if offset == 0 {
                quote! { value as #uprim & #mask }
            } else {
                let o = Literal::u32_unsuffixed(offset);
                quote! { (value as #uprim & #mask) << #o }
            };
            quote! {
                self.#field = ((self.#field as #uprim & #clear) | (#place)) as #prim;
            }
        } else {
            let place = if offset == 0 {
                quote! { value & #mask }
            } else {
                let o = Literal::u32_unsuffixed(offset);
                quote! { (value & #mask) << #o }
            };
            quote! {
                self.#field = (self.#field & #clear) | (#place);
            }
        };

        quote! {
            pub fn #getter(&self) -> #prim {
                #get_body
            }
            pub fn #setter(&mut self, value: #prim) {
                #setter_body
            }
        }
    }

    fn write_typed_accessor(
        &self,
        config: &Config,
        field: &TokenStream,
        storage: (&TokenStream, &[TokenStream], bool),
        member: (&str, &Type),
        range: (u32, u32),
        arch: i32,
    ) -> TokenStream {
        let (member, ty) = member;
        let (offset, width) = range;
        let (read, storage_wrappers, unaligned) = storage;
        let (scalar, wrappers) = bitfield_scalar(ty, config, arch);
        let prim = &self.prim;
        let uprim = &self.uprim;
        let shift_offset = Literal::u32_unsuffixed(offset);
        let mask = Literal::u64_unsuffixed(u64::MAX >> (64 - width));
        let raw = if self.signed {
            quote! { (#read as #uprim) }
        } else {
            read.clone()
        };
        let mut bits = if offset == 0 {
            raw.clone()
        } else {
            quote! { #raw >> #shift_offset }
        };
        if offset + width < self.bits {
            bits = quote! { (#bits) & #mask };
        }
        let member_backing = (scalar != Type::Bool).then(|| {
            let member_backing = Self::from_type(&scalar)
                .unwrap_or_else(|| panic!("unsupported bitfield member type: {ty:?}"));
            assert!(
                width <= member_backing.bits,
                "bitfield `{member}` exceeds its member type"
            );
            member_backing
        });
        let bounds = self.pointer.then(|| {
            let end = Literal::u32_unsuffixed(offset + width);
            quote! { const _: () = assert!(#prim::BITS >= #end, "bitfield exceeds target storage width"); }
        });
        let member_bounds = member_backing.as_ref().filter(|member| member.pointer && !self.pointer).map(|member| {
            let prim = &member.prim;
            let width = Literal::u32_unsuffixed(width);
            quote! { const _: () = assert!(#prim::BITS >= #width, "bitfield exceeds target member width"); }
        });
        let mut get = if let Some(member_backing) = &member_backing {
            let member_prim = &member_backing.prim;
            if member_backing.signed && width < member_backing.bits {
                let shift = if member_backing.pointer {
                    let width = Literal::u32_unsuffixed(width);
                    quote! { (#member_prim::BITS - #width) }
                } else {
                    let shift = Literal::u32_unsuffixed(member_backing.bits - width);
                    quote! { #shift }
                };
                quote! { (((#bits) as #member_prim) << #shift) >> #shift }
            } else if width == self.bits
                && member_backing.same_width(self)
                && member_backing.signed == self.signed
            {
                read.clone()
            } else if !member_backing.signed && member_backing.same_width(self) {
                bits
            } else {
                quote! { (#bits) as #member_prim }
            }
        } else {
            quote! { (#bits) != 0 }
        };
        let mut value = quote! { value };
        for _ in &wrappers {
            value = quote! { #value.0 };
        }
        for wrapper in wrappers.iter().rev() {
            get = quote! { #wrapper(#get) };
        }
        let mut set = if width == self.bits {
            if member_backing
                .as_ref()
                .is_some_and(|member| member.same_width(self) && member.signed == self.signed)
            {
                value
            } else {
                quote! { #value as #prim }
            }
        } else {
            let value = if member_backing
                .as_ref()
                .is_some_and(|member| member.same_width(self) && !member.signed)
            {
                value
            } else {
                quote! { (#value as #uprim) }
            };
            let (clear, place) = if offset == 0 {
                (quote! { !#mask }, quote! { (#value & #mask) })
            } else {
                (
                    quote! { !(#mask << #shift_offset) },
                    quote! { ((#value & #mask) << #shift_offset) },
                )
            };
            let merged = quote! { (#raw & #clear) | #place };
            if self.signed {
                quote! { (#merged) as #prim }
            } else {
                merged
            }
        };
        for wrapper in storage_wrappers.iter().rev() {
            set = quote! { #wrapper(#set) };
        }
        if unaligned {
            let helper = Type::UnalignedHelper.type_name().write(config, &[]);
            set = quote! { #helper::new(#set) };
        }
        let safety = unaligned.then(|| {
            quote! {
                /// # Safety
                ///
                /// The backing storage must be initialized.
            }
        });
        let unsafety = unaligned.then(|| quote! { unsafe });
        let getter = to_ident(member);
        let setter = to_ident(&format!("set_{member}"));
        let name = ty.write_name(config);
        quote! {
            #safety
            pub #unsafety fn #getter(&self) -> #name {
                #bounds
                #member_bounds
                #get
            }
            #safety
            pub #unsafety fn #setter(&mut self, value: #name) {
                #bounds
                #member_bounds
                self.#field = #set;
            }
        }
    }
}

impl Dependencies for CppStruct {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        for ty in self.definition_types(reader) {
            ty.combine(dependencies, reader);
        }
    }
}
