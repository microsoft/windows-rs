use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppConst {
    pub namespace: &'static str,
    pub field: Field,
}

impl Ord for CppConst {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.field.name(), self.field).cmp(&(other.field.name(), other.field))
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

        if let Some(guid) = self.field.guid_attribute() {
            return config.write_cpp_const_guid(name, &guid);
        }

        let cfg = self.write_cfg(config);

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
                    let value = constant.value().write();

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
                // In `--sys` mode, every wrapper struct/enum is emitted as a
                // bare type alias, so constants must drop the `Self(value)`
                // newtype constructor. In `--minimal` mode handle structs and
                // unscoped enums also become bare aliases (see cpp_handle and
                // cpp_enum), so we drop the wrapper for those too; scoped
                // enums and other wrapper types still get the constructor.
                let emit_alias_const = config.bindgen.style.is_sys()
                    || (config.bindgen.style.is_minimal()
                        && (matches!(&field_ty, Type::CppStruct(ty) if ty.is_handle(config.reader))
                            || matches!(&field_ty, Type::CppEnum(e) if !e.def.has_attribute("ScopedEnumAttribute"))));
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
        } else if let Some(attribute) = self.field.find_attribute("ConstantAttribute") {
            let args = attribute.value();
            let Some((_, Value::Utf8(input_str))) = args.first() else {
                panic!()
            };
            let mut input = input_str.as_str();

            let Type::CppStruct(ty) = &field_ty else {
                panic!()
            };

            let mut tokens = quote! {};

            for field in ty.def.fields() {
                let (value, rest) = config.field_initializer(field, input);
                input = rest;
                tokens.combine(value);
            }

            let ty = field_ty.write_name(config);

            quote! {
                #cfg
                pub const #name: #ty = #ty { #tokens };
            }
        } else {
            panic!()
        }
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
