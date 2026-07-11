use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppConst {
    pub namespace: &'static str,
    pub field: Field,
    /// Architecture bits of the parent unscoped enum when this constant is an enum member;
    /// `0` for a free-standing constant. Enum-member fields are themselves arch-neutral (the
    /// `SupportedArchitecture` tag sits on the enum type), so an arch-divergent member must
    /// inherit the enum's arches — otherwise the neutral member collides with a same-named
    /// arch-specific macro constant on the complementary architecture.
    pub enum_arches: i32,
    /// `true` when this constant is a member of an unscoped enum. Such a member is always
    /// projected as a bare alias constant, even when its type resolves by name to a same-named
    /// non-enum sibling (e.g. `KSPIN_LOCK_QUEUE_NUMBER` is an enum on x86/arm64 but a pointer
    /// typedef on x64), which would otherwise wrap the value in an invalid tuple constructor.
    pub is_enum_member: bool,
}

impl Ord for CppConst {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.field.name(), self.field, self.enum_arches).cmp(&(
            other.field.name(),
            other.field,
            other.enum_arches,
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

    /// The architectures this constant is emitted for: the field's own `SupportedArchitecture`
    /// bits when present, otherwise the parent enum's (for enum members).
    pub fn effective_arches(&self) -> i32 {
        let field_arches = self.field.arches();
        if field_arches != 0 {
            field_arches
        } else {
            self.enum_arches
        }
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write_cfg(&self, config: &Config) -> TokenStream {
        write_simple_cfg(self, config)
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        if let windows_metadata::Type::ClassName(type_name)
        | windows_metadata::Type::ValueName(type_name) = self.field.ty()
        {
            if type_name.namespace.is_empty() {
                return quote! {};
            }
        }

        let field_ty = self.field.field_type(None, config.reader).to_const_type();
        let tn = field_ty.type_name().name();
        let name = if !tn.is_empty() && self.field.name() == tn {
            to_ident(&format!("{tn}_"))
        } else {
            to_ident(self.field.name())
        };

        let arches = write_arch_bits(self.effective_arches());
        let cfg = self.write_cfg(config);
        let cfg = quote! { #arches #cfg };

        if let Some(guid) = self.field.guid_attribute() {
            // A `GuidAttribute` on a struct-typed field is a property-key constant
            // (`PROPERTYKEY`/`DEVPROPKEY`): the attribute is the `fmtid` and the field's
            // `Constant` the `pid`. Reconstruct the struct literal from those two structured
            // pieces rather than parsing an initializer string.
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
                let mut value = constant.value().write();

                if underlying_ty == constant_ty {
                    if is_signed_error(&field_ty, config.reader) {
                        if let Value::I32(signed) = constant.value() {
                            value = format!("0x{signed:X}_u32 as _").parse().unwrap();
                        }
                    }
                } else if field_ty == Type::Bool {
                    value = match constant.value() {
                        Value::U8(1) => quote! { true },
                        Value::U8(0) => quote! { false },
                        _ => panic!(),
                    };
                } else {
                    value = wide_int_cast(&constant.value());
                }
                // Constants of a bare-alias type must drop the `Self(value)` newtype constructor.
                // Unscoped (C-style) enums are emitted as bare `pub type = <int>` aliases in every
                // style (see `cpp_enum`), so their constants are plain integers everywhere. In
                // `--sys` every wrapper is a bare alias; in `--minimal` handle structs are too.
                // Scoped enums and (in default/minimal) handle newtypes still get the constructor.
                let unscoped_enum_const = self.is_enum_member
                    || matches!(&field_ty, Type::CppEnum(e) if !e.def.has_attribute("ScopedEnumAttribute"));
                let emit_alias_const = config.bindgen.style.is_sys()
                    || unscoped_enum_const
                    || (config.bindgen.style.is_minimal()
                        && matches!(&field_ty, Type::CppStruct(ty) if ty.is_handle(config.reader)));
                if emit_alias_const || field_ty == Type::Bool {
                    quote! {
                        #cfg
                        pub const #name: #ty = #value;
                    }
                } else {
                    // A non-handle native typedef (e.g. `LPCTSTR = PCSTR`) is
                    // emitted as a transparent `pub type` alias, which cannot be
                    // used as a tuple-struct constructor (E0423). Construct the
                    // value through the underlying newtype instead.
                    let ctor = match &field_ty {
                        Type::CppStruct(s)
                            if !s.is_handle(config.reader)
                                && s.is_native_typedef(config.reader) =>
                        {
                            underlying_ty.write_name(config)
                        }
                        _ => ty.clone(),
                    };
                    // In full mode a handle may wrap another newtype handle (e.g.
                    // `HCERTCHAINENGINE(HANDLE)` or `JET_GRBIT(JET_UINT32)`). A bare
                    // `value as _` cannot cast to that wrapper, so build the argument
                    // through each intervening newtype layer.
                    let arg = match &field_ty {
                        Type::CppStruct(s) if s.is_handle(config.reader) => {
                            write_newtype_wrap(&underlying_ty, &value, config)
                        }
                        _ => value,
                    };
                    quote! {
                        #cfg
                        pub const #name: #ty = #ctor(#arg);
                    }
                }
            }
        } else {
            panic!()
        }
    }
}

/// Emits a `usize`/`isize` constant value portably across 32- and 64-bit targets.
/// A pointer-sized sentinel such as `#define ITSAT_DEFAULT_LPARAM ((DWORD_PTR)-1)` is
/// stored as the 64-bit two's-complement value (`0xFFFF_FFFF_FFFF_FFFF`); written as a
/// bare literal it overflows a 32-bit `usize` (`E0080`). Emitting `<value>u64 as usize`
/// truncates to the target's pointer width, reproducing the correct arch-specific value
/// (`0xFFFF_FFFF` on 32-bit, `0xFFFF_FFFF_FFFF_FFFF` on 64-bit). Values that already fit a
/// 32-bit target keep the bare literal, so existing bindings are unaffected.
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

/// Emits `<literal> as _` for a constant value that is cast to a native-typedef /
/// handle / pointer target. A bare unsuffixed literal defaults to `i32`, so a value
/// outside `i32`'s range (e.g. `0xFFFF_FFFF` for an unsigned `JET_DBID`, or a wide
/// handle sentinel) would overflow that default *before* the cast is applied. Such
/// values are given an explicit unsigned/wide suffix so the literal type holds them;
/// every value that already fits `i32` keeps the bare form, so published bindings are
/// unaffected.
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

impl Dependencies for CppConst {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        if let windows_metadata::Type::ClassName(type_name)
        | windows_metadata::Type::ValueName(type_name) = self.field.ty()
        {
            if type_name.namespace.is_empty() {
                return;
            }
        }

        let ty = self.field.field_type(None, reader).to_const_type();

        // String constants emit PCWSTR/PCSTR in the generated code, so we
        // need those types in the dependency closure (not Type::String).
        if ty == Type::String {
            if is_ansi_encoding(self.field) {
                Type::PCSTR.combine(dependencies, reader);
            } else {
                Type::PCWSTR.combine(dependencies, reader);
            }
        } else {
            ty.combine(dependencies, reader);
        }
    }
}

fn is_ansi_encoding(row: Field) -> bool {
    row.find_attribute("NativeEncodingAttribute").is_some_and(|attribute| matches!(attribute.value().first(), Some((_, Value::Utf8(encoding))) if encoding.as_str() == "ansi"))
}

fn is_signed_error(ty: &Type, reader: &Reader) -> bool {
    match ty {
        Type::HRESULT => true,
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
