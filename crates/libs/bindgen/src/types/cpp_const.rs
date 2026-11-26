use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppConst {
    pub namespace: &'static str,
    pub field: Field,
    /// Projected architecture bits; `0` means all architectures.
    pub arches: i32,
    /// `true` for unscoped enum members, which are always bare alias constants.
    pub is_enum_member: bool,
}

impl Ord for CppConst {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.field.name(), self.field, self.arches).cmp(&(
            other.field.name(),
            other.field,
            other.arches,
        ))
    }
}

impl PartialOrd for CppConst {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppConst {
    pub fn type_name(&self) -> TypeName {
        TypeName(self.namespace, self.field.name())
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write_cfg(&self, config: &Config) -> TokenStream {
        write_simple_cfg(self, config)
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        self.write_with_enclosing(config, None)
    }

    pub fn write_with_enclosing(
        &self,
        config: &Config,
        enclosing: Option<&CppStruct>,
    ) -> TokenStream {
        if let windows_metadata::Type::ClassName(type_name)
        | windows_metadata::Type::ValueName(type_name) = self.field.ty()
            && type_name.namespace.is_empty()
        {
            return quote! {};
        }

        let field_ty = self
            .field
            .field_type(enclosing, config.reader)
            .to_const_type();
        if self.is_enum_member
            && (config.plan.selection.excludes_type_name(self.type_name())
                || config
                    .plan
                    .selection
                    .enum_variant_filter(
                        field_ty.type_name().namespace(),
                        field_ty.type_name().name(),
                    )
                    .is_some_and(|variants| !variants.includes(self.field.name())))
        {
            return quote! {};
        }
        let tn = field_ty.type_name().name();
        let name = if enclosing.is_none() && !tn.is_empty() && self.field.name() == tn {
            to_ident(&format!("{tn}_"))
        } else {
            to_ident(self.field.name())
        };

        let arches = write_arch_bits(self.arches);
        let cfg = self.write_cfg(config);
        let cfg = quote! { #arches #cfg };

        if let Some(guid) = self.field.guid_attribute() {
            // Property-key constants store `fmtid` in `GuidAttribute` and `pid` in `Constant`.
            if let Type::CppStruct(ty) = &field_ty {
                let struct_ty = field_ty.write_name(config);
                let mut fields = quote! {};

                for field in ty.def.fields() {
                    let field_name = to_ident(field.name());
                    let member_ty = field.field_type(None, config.reader);
                    if resolves_to_guid(&member_ty, config.reader) {
                        let value = config.write_guid_value(&guid);
                        fields.combine(quote! { #field_name: #value, });
                    } else if let Some(constant) = self.field.constant() {
                        let value = constant.value().write();
                        let value = write_newtype_wrap(&member_ty, &value, config);
                        fields.combine(quote! { #field_name: #value, });
                    }
                }

                return quote! {
                    #cfg
                    pub const #name: #struct_ty = #struct_ty { #fields };
                };
            }

            return config.write_cpp_const_guid(name, &guid);
        }

        if let Some(constant) = self.field.constant() {
            let constant_ty = constant.constant_type(config.reader);
            if field_ty.is_pointer() {
                let ty = field_ty.write_name(config);
                let value = pointer_const_value(&field_ty, &constant.value(), config);
                return quote! {
                    #cfg
                    pub const #name: #ty = #value;
                };
            }

            if field_ty == constant_ty {
                if field_ty == Type::String {
                    if config.bindgen.uses_inline_core_types() {
                        // Sys bindings emit inline core types, so the w!/s!
                        // macros are unavailable.
                        // Emit an inline null-terminated array instead.
                        let (Value::Utf16(value_str) | Value::Utf8(value_str)) = constant.value()
                        else {
                            panic!("expected string constant")
                        };
                        if is_ansi_encoding(self.field) {
                            let bytes: Vec<u8> =
                                value_str.bytes().chain(std::iter::once(0)).collect();
                            let lit_bytes = bytes.iter().map(|b| Literal::u8_unsuffixed(*b));
                            let ty = Type::PCSTR.write_name(config);
                            quote! {
                                #cfg
                                pub const #name: #ty = [#(#lit_bytes),*].as_ptr();
                            }
                        } else {
                            let units: Vec<u16> =
                                value_str.encode_utf16().chain(std::iter::once(0)).collect();
                            let lit_units = units.iter().map(|u| Literal::u16_unsuffixed(*u));
                            let ty = Type::PCWSTR.write_name(config);
                            quote! {
                                #cfg
                                pub const #name: #ty = [#(#lit_units),*].as_ptr();
                            }
                        }
                    } else {
                        let crate_name = config.write_core();
                        let value = constant.value().write();

                        if is_ansi_encoding(self.field) {
                            quote! {
                                #cfg
                                pub const #name: #crate_name PCSTR = #crate_name s!(#value);
                            }
                        } else {
                            quote! {
                                #cfg
                                pub const #name: #crate_name PCWSTR = #crate_name w!(#value);
                            }
                        }
                    }
                } else {
                    let ty = field_ty.write_name(config);
                    let value = pointer_sized_const_value(&field_ty, &constant.value());

                    quote! {
                        #cfg
                        pub const #name: #ty = #value;
                    }
                }
            } else {
                let underlying_ty = field_ty.underlying_type(config.reader);
                let ty = field_ty.write_name(config);

                let value = if let Some(value) =
                    fixed_to_native_const_value(&underlying_ty, &constant.value())
                {
                    value
                } else if underlying_ty == constant_ty {
                    let mut value = pointer_sized_const_value(&underlying_ty, &constant.value());
                    if is_signed_error(&field_ty, config.reader)
                        && let Value::I32(signed) = constant.value()
                    {
                        value = format!("0x{signed:X}_u32 as _").parse().unwrap();
                    }
                    value
                } else if field_ty == Type::Bool {
                    match constant.value() {
                        Value::U8(1) => quote! { true },
                        Value::U8(0) => quote! { false },
                        _ => panic!(),
                    }
                } else {
                    wide_int_cast(&constant.value())
                };
                // Bare-alias constants cannot use tuple constructors; wrap only concrete newtype
                // layers.
                let unscoped_enum_const = match &field_ty {
                    Type::CppEnum(e) => !config.enum_is_newtype(e.def),
                    _ => self.is_enum_member,
                };
                let field_ty_bare_alias =
                    matches!(&field_ty, Type::CppStruct(s) if config.typedef_emits_bare(s.def));
                let emit_alias_const =
                    config.bindgen.style.is_sys() || unscoped_enum_const || field_ty_bare_alias;
                if emit_alias_const || matches!(field_ty, Type::Bool | Type::ISize | Type::USize) {
                    // Arch-blind lookup can find a same-name non-enum sibling; enum members stay
                    // integers.
                    let value = if unscoped_enum_const {
                        value
                    } else {
                        write_newtype_wrap_for_arches(&field_ty, &value, config, self.arches)
                    };
                    quote! {
                        #cfg
                        pub const #name: #ty = #value;
                    }
                } else {
                    // Transparent native typedef aliases cannot be tuple constructors.
                    let ctor = match &field_ty {
                        Type::CppStruct(s)
                            if !s.is_handle(config.reader)
                                && s.is_native_typedef(config.reader) =>
                        {
                            underlying_ty.write_name(config)
                        }
                        _ => ty.clone(),
                    };
                    // Full-mode handle constants must wrap through each nested newtype layer.
                    let arg = match &field_ty {
                        Type::CppStruct(s) if s.is_handle(config.reader) => {
                            write_newtype_wrap_for_arches(
                                &underlying_ty,
                                &value,
                                config,
                                self.arches,
                            )
                        }
                        _ => value,
                    };
                    quote! {
                        #cfg
                        pub const #name: #ty = #ctor(#arg);
                    }
                }
            }
        } else if let Some(input) = legacy_constant_initializer(self.field, config.reader) {
            let Type::CppStruct(record) = &field_ty else {
                panic!("legacy constant `{}` is not a struct", self.field.name());
            };
            let mut input = input.as_str();
            let mut fields = quote! {};
            for field in record.def.fields() {
                let (value, rest) = config.legacy_field_initializer(field, input);
                input = rest;
                fields.combine(value);
            }
            let ty = field_ty.write_name(config);
            quote! {
                #cfg
                pub const #name: #ty = #ty { #fields };
            }
        } else {
            panic!()
        }
    }
}

// Win32metadata encodes struct constants as `ConstantAttribute` initializer text.
fn legacy_constant_initializer(field: Field, reader: &Reader) -> Option<String> {
    if !reader.legacy() {
        return None;
    }
    match field
        .find_attribute("ConstantAttribute")?
        .value()
        .into_iter()
        .next()
    {
        Some((_, Value::Utf8(input))) => Some(input),
        _ => panic!("invalid constant initializer on `{}`", field.name()),
    }
}

impl Config<'_> {
    fn legacy_field_initializer<'a>(&self, field: Field, input: &'a str) -> (TokenStream, &'a str) {
        let name = to_ident(field.name());

        match field.field_type(None, self.reader) {
            Type::GUID => {
                let (literals, rest) = read_literal_array(input, 11);
                let value = self.write_guid_value(&GUID(
                    literals[0].parse().unwrap(),
                    literals[1].parse().unwrap(),
                    literals[2].parse().unwrap(),
                    literals[3].parse().unwrap(),
                    literals[4].parse().unwrap(),
                    literals[5].parse().unwrap(),
                    literals[6].parse().unwrap(),
                    literals[7].parse().unwrap(),
                    literals[8].parse().unwrap(),
                    literals[9].parse().unwrap(),
                    literals[10].parse().unwrap(),
                ));
                (quote! { #name: #value, }, rest)
            }
            Type::ArrayFixed(_, len) => {
                let (literals, rest) = read_literal_array(input, len);
                let literals = literals
                    .iter()
                    .map(|literal| literal.parse::<TokenStream>().unwrap());
                (quote! { #name: [#(#literals,)*], }, rest)
            }
            Type::PtrMut(..) => {
                // Win32metadata initializes pointers with integers; only null is representable.
                let (_, rest) = read_literal(input);
                (quote! { #name: core::ptr::null_mut(), }, rest)
            }
            ty => {
                let (literal, rest) = read_literal(input);
                let literal = literal.parse::<TokenStream>().unwrap();
                let value = write_newtype_wrap(&ty, &literal, self);
                (quote! { #name: #value, }, rest)
            }
        }
    }
}

fn read_literal_array(input: &str, len: usize) -> (Vec<&str>, &str) {
    let mut input = read_token(input, b'{');
    let mut result = vec![];
    for _ in 0..len {
        let (literal, rest) = read_literal(input);
        result.push(literal);
        input = rest;
    }
    (result, read_token(input, b'}'))
}

fn read_literal(input: &str) -> (&str, &str) {
    let mut start = None;
    let mut end = 0;
    for (pos, c) in input.bytes().enumerate() {
        if start.is_none() {
            if c != b' ' && c != b',' {
                start = Some(pos);
            }
        } else if c == b' ' || c == b',' || c == b'}' {
            break;
        }
        end += 1;
    }
    let start = start.unwrap_or_else(|| panic!("literal expected in `{input}`"));
    (&input[start..end], &input[end..])
}

fn read_token(input: &str, token: u8) -> &str {
    for (pos, c) in input.bytes().enumerate() {
        if c == token {
            return &input[pos + 1..];
        } else if c != b' ' && c != b',' {
            break;
        }
    }
    panic!("`{}` expected", token.escape_ascii());
}

fn pointer_const_value(field_ty: &Type, value: &Value, config: &Config) -> TokenStream {
    let address = match value {
        Value::I8(value) => signed_pointer_address(i64::from(*value), "i8"),
        Value::I16(value) => signed_pointer_address(i64::from(*value), "i16"),
        Value::I32(value) => signed_pointer_address(i64::from(*value), "i32"),
        Value::I64(value) | Value::ISize(value) => signed_pointer_address(*value, "i64"),
        Value::U8(value) => {
            let value = Literal::usize_suffixed((*value).into());
            quote! { #value }
        }
        Value::U16(value) => {
            let value = Literal::usize_suffixed((*value).into());
            quote! { #value }
        }
        Value::U32(value) => {
            let value = Literal::u32_suffixed(*value);
            quote! { #value as usize }
        }
        Value::U64(value) | Value::USize(value) => {
            let value = Literal::u64_suffixed(*value);
            quote! { #value as usize }
        }
        rest => panic!("pointer constant has unsupported value {rest:?}"),
    };
    match field_ty {
        Type::PtrMut(inner, pointers) => {
            let pointee = if *pointers == 1 {
                inner.write_default(config)
            } else {
                Type::PtrMut(inner.clone(), pointers - 1).write_name(config)
            };
            quote! { core::ptr::without_provenance_mut::<#pointee>(#address) }
        }
        Type::PtrConst(inner, pointers) => {
            let pointee = if *pointers == 1 {
                inner.write_default(config)
            } else {
                Type::PtrConst(inner.clone(), pointers - 1).write_name(config)
            };
            quote! { core::ptr::without_provenance::<#pointee>(#address) }
        }
        _ => unreachable!(),
    }
}

fn signed_pointer_address(value: i64, suffix: &str) -> TokenStream {
    if value >= 0 {
        let value = Literal::usize_suffixed(value.try_into().unwrap());
        quote! { #value }
    } else {
        let value: TokenStream = format!("{value}{suffix}").parse().unwrap();
        quote! { (#value) as usize }
    }
}

/// Emits pointer-sized constants without overflowing 32-bit targets.
fn pointer_sized_const_value(field_ty: &Type, value: &Value) -> TokenStream {
    match (field_ty, value) {
        (Type::USize, Value::USize(v)) if *v > u32::MAX as u64 => {
            let lit = Literal::u64_suffixed(*v);
            quote! { #lit as usize }
        }
        (Type::ISize, Value::ISize(v)) if !(i32::MIN as i64..=i32::MAX as i64).contains(v) => {
            let lit = Literal::i64_suffixed(*v);
            quote! { #lit as isize }
        }
        _ => value.write(),
    }
}

fn fixed_to_native_const_value(ty: &Type, value: &Value) -> Option<TokenStream> {
    Some(match (ty, value) {
        (Type::ISize, Value::I32(value)) => {
            let literal = Literal::i32_unsuffixed(*value);
            quote! { #literal }
        }
        (Type::USize, Value::U32(value)) => {
            let literal = Literal::u32_unsuffixed(*value);
            quote! { #literal }
        }
        (Type::ISize, Value::I64(value)) => {
            let literal = Literal::i64_suffixed(*value);
            quote! { #literal as isize }
        }
        (Type::ISize, Value::U32(value)) => {
            let literal = Literal::u32_suffixed(*value);
            quote! { #literal as isize }
        }
        (Type::ISize, Value::U64(value)) => {
            let literal = Literal::u64_suffixed(*value);
            quote! { #literal as isize }
        }
        (Type::USize, Value::U64(value)) => {
            let literal = Literal::u64_suffixed(*value);
            quote! { #literal as usize }
        }
        (Type::USize, Value::I32(value)) => {
            let literal = Literal::i32_suffixed(*value);
            quote! { #literal as usize }
        }
        (Type::USize, Value::I64(value)) => {
            let literal = Literal::i64_suffixed(*value);
            quote! { #literal as usize }
        }
        _ => return None,
    })
}

/// Emits wide integer casts with suffixes so values outside `i32` do not overflow first.
fn wide_int_cast(value: &Value) -> TokenStream {
    let fits_i32 = |v: i128| (i32::MIN as i128..=i32::MAX as i128).contains(&v);
    match value {
        Value::U32(v) if !fits_i32(*v as i128) => {
            let lit = Literal::u32_suffixed(*v);
            quote! { #lit as _ }
        }
        Value::U64(v) if !fits_i32(*v as i128) => {
            let lit = Literal::u64_suffixed(*v);
            quote! { #lit as _ }
        }
        Value::I64(v) if !fits_i32(*v as i128) => {
            let lit = Literal::i64_suffixed(*v);
            quote! { #lit as _ }
        }
        Value::USize(v) if !fits_i32(*v as i128) => {
            let lit = Literal::u64_suffixed(*v);
            quote! { #lit as _ }
        }
        Value::ISize(v) if !fits_i32(*v as i128) => {
            let lit = Literal::i64_suffixed(*v);
            quote! { #lit as _ }
        }
        _ => {
            let value = value.write();
            quote! { #value as _ }
        }
    }
}

impl CppConst {
    /// The type spelled by the generated constant, if it names one.
    pub fn value_type(&self, reader: &Reader) -> Option<Type> {
        if let windows_metadata::Type::ClassName(type_name)
        | windows_metadata::Type::ValueName(type_name) = self.field.ty()
            && type_name.namespace.is_empty()
        {
            return None;
        }

        let ty = self.field.field_type(None, reader).to_const_type();

        // String constants emit PCWSTR/PCSTR, not Type::String.
        Some(if ty != Type::String {
            ty
        } else if is_ansi_encoding(self.field) {
            Type::PCSTR
        } else {
            Type::PCWSTR
        })
    }
}

impl Dependencies for CppConst {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        if let Some(ty) = self.value_type(reader) {
            ty.combine(dependencies, reader);
        }
    }
}

fn is_ansi_encoding(row: Field) -> bool {
    row.attributes().any(|attribute| {
        matches!(
            (attribute.namespace(), attribute.name()),
            ("Windows.Win32.Metadata", "EncodingAttribute")
                | ("Windows.Win32.Foundation.Metadata", "NativeEncodingAttribute")
        ) && matches!(attribute.value().first(), Some((_, Value::Utf8(encoding))) if encoding.as_str() == "ansi")
    })
}

fn is_signed_error(ty: &Type, reader: &Reader) -> bool {
    match ty {
        Type::HRESULT | Type::NTSTATUS | Type::RPC_STATUS => true,
        Type::CppStruct(ty) => !ty.def.underlying_type_ext(reader).is_unsigned(),
        _ => false,
    }
}

// Resolves a field type through any chain of native typedefs (e.g. `DEVPROPGUID = GUID`)
// to decide whether it ultimately is the `GUID` carrying a property key's `fmtid`.
fn resolves_to_guid(ty: &Type, reader: &Reader) -> bool {
    let mut ty = ty.clone();
    loop {
        if ty == Type::GUID {
            return true;
        }
        match &ty {
            Type::CppStruct(s) if s.is_native_typedef(reader) => {
                ty = s.def.underlying_type_ext(reader);
            }
            _ => return false,
        }
    }
}
