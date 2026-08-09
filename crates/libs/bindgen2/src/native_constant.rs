use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

/// An owned Win32 constant projection.
pub struct Constant {
    namespace: String,
    name: String,
    ty: native::Type,
    value: Value,
}

enum Value {
    Fixed {
        underlying: native::Type,
        value: ConstantValue,
        ansi: bool,
    },
    Guid(guid::Guid),
    PropertyKey {
        guid: guid::Guid,
        fields: [String; 2],
        pid: ConstantValue,
    },
}

impl Constant {
    pub(super) fn lower(
        database: &Database,
        field: windows_metadata2::FieldDefinition<'_>,
        namespace: &str,
        name: &str,
    ) -> Result<Self, Error> {
        let full_name = format!("{namespace}.{name}");
        let signature = field.signature()?;
        let ty = native::Type::lower(
            database,
            field.entity().file(),
            &full_name,
            signature.clone(),
        )?;
        let guid = guid::Guid::from_field(field, &full_name)?;
        let constant = field.constant()?;

        if let Some(guid) = guid {
            let value = if let Some(constant) = constant {
                Value::PropertyKey {
                    guid,
                    fields: property_fields(
                        database,
                        field.entity().file(),
                        &signature,
                        &full_name,
                    )?,
                    pid: constant.value()?,
                }
            } else {
                Value::Guid(guid)
            };
            return Ok(Self {
                namespace: namespace.to_string(),
                name: name.to_string(),
                ty,
                value,
            });
        }

        let value = constant
            .ok_or_else(|| Error::InvalidValue {
                name: full_name.clone(),
                message: "constant field has no Constant row",
            })?
            .value()?;
        let underlying = if ty.matches(&value) {
            ty.clone()
        } else {
            native::Type::constant_underlying(
                database,
                field.entity().file(),
                &full_name,
                &signature,
            )?
            .ok_or_else(|| Error::UnsupportedType {
                name: full_name.clone(),
                shape: format!("typed constant {ty:?} <- {value:?}"),
            })?
        };
        if !underlying.accepts_converted(&value) {
            return Err(Error::UnsupportedType {
                name: full_name,
                shape: format!("typed constant {ty:?} <- {value:?}"),
            });
        }
        Ok(Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            ty,
            value: Value::Fixed {
                underlying,
                value,
                ansi: is_ansi(field)?,
            },
        })
    }

    /// Renders a flat Win32 sys constant.
    pub fn write_sys(&self) -> TokenStream {
        let name = tokens::ident(&self.name);
        match &self.value {
            Value::Guid(guid) => {
                let guid = guid.write_u128();
                quote! {
                    pub const #name: windows_sys::core::GUID =
                        windows_sys::core::GUID::from_u128(#guid);
                }
            }
            Value::PropertyKey { guid, fields, pid } => {
                let ty = self.ty.write(&self.namespace);
                let guid = guid.write_u128();
                let guid_field = tokens::ident(&fields[0]);
                let pid_field = tokens::ident(&fields[1]);
                let pid = write_value(&Self::native_value_type(pid), pid);
                quote! {
                    pub const #name: #ty = #ty {
                        #guid_field: windows_sys::core::GUID::from_u128(#guid),
                        #pid_field: #pid,
                    };
                }
            }
            Value::Fixed {
                value: ConstantValue::String(value),
                ansi: true,
                ..
            } => {
                let bytes = value
                    .bytes()
                    .chain(std::iter::once(0))
                    .map(Literal::u8_unsuffixed);
                quote! {
                    pub const #name: PCSTR = [#(#bytes),*].as_ptr();
                }
            }
            Value::Fixed {
                value: ConstantValue::String(value),
                ..
            } => {
                let units = value
                    .encode_utf16()
                    .chain(std::iter::once(0))
                    .map(Literal::u16_unsuffixed);
                quote! {
                    pub const #name: PCWSTR = [#(#units),*].as_ptr();
                }
            }
            Value::Fixed {
                underlying, value, ..
            } => {
                let ty = self.ty.write(&self.namespace);
                let value = if self.ty == *underlying {
                    write_value(underlying, value)
                } else {
                    Self::write_converted(underlying, value)
                };
                quote! { pub const #name: #ty = #value; }
            }
        }
    }

    fn write_converted(underlying: &native::Type, value: &ConstantValue) -> TokenStream {
        if matches!(underlying, native::Type::Boolean) {
            return match value {
                ConstantValue::U8(0) => quote! { false },
                ConstantValue::U8(1) => quote! { true },
                _ => unreachable!(),
            };
        }
        if underlying.signed_i32()
            && let ConstantValue::I32(value) = value
        {
            return format!("0x{:X}_u32 as _", *value as u32).parse().unwrap();
        }
        if underlying.matches(value) {
            return write_value(underlying, value);
        }
        let value = write_value(&Self::native_value_type(value), value);
        quote! { #value as _ }
    }

    fn native_value_type(value: &ConstantValue) -> native::Type {
        match value {
            ConstantValue::Boolean(_) => native::Type::Boolean,
            ConstantValue::Char(_) => native::Type::Char,
            ConstantValue::I8(_) => native::Type::I8,
            ConstantValue::U8(_) => native::Type::U8,
            ConstantValue::I16(_) => native::Type::I16,
            ConstantValue::U16(_) => native::Type::U16,
            ConstantValue::I32(_) => native::Type::I32,
            ConstantValue::U32(_) => native::Type::U32,
            ConstantValue::I64(_) => native::Type::I64,
            ConstantValue::U64(_) => native::Type::U64,
            ConstantValue::ISize(_) => native::Type::ISize,
            ConstantValue::USize(_) => native::Type::USize,
            ConstantValue::F32(_) => native::Type::F32,
            ConstantValue::F64(_) => native::Type::F64,
            ConstantValue::String(_) => native::Type::String,
            ConstantValue::Null => unreachable!(),
        }
    }
}

fn property_fields(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
    owner: &str,
) -> Result<[String; 2], Error> {
    let (TypeKind::Value(id) | TypeKind::Class(id)) = &ty.kind else {
        return Err(Error::InvalidValue {
            name: owner.to_string(),
            message: "GUID-backed constant is not a named struct",
        });
    };
    let Some((namespace, name)) = database.type_name(file, *id)? else {
        return Err(Error::InvalidValue {
            name: owner.to_string(),
            message: "GUID-backed constant type has no name",
        });
    };
    let definitions = database.type_definitions(namespace, name);
    if definitions.len() != 1 {
        return Err(Error::InvalidValue {
            name: owner.to_string(),
            message: "GUID-backed constant type does not have one definition",
        });
    }
    let fields = database
        .definition(definitions[0])
        .unwrap()
        .fields()?
        .map(|field| field.name().map(str::to_string))
        .collect::<Result<Vec<_>, _>>()?;
    let [first, second] = fields.try_into().map_err(|_| Error::InvalidValue {
        name: owner.to_string(),
        message: "GUID-backed constant struct does not have two fields",
    })?;
    Ok([first, second])
}

fn write_value(ty: &native::Type, value: &ConstantValue) -> TokenStream {
    match (ty, value) {
        (native::Type::USize, ConstantValue::USize(value)) if *value > u32::MAX as u64 => {
            let value = Literal::u64_suffixed(*value);
            return quote! { #value as usize };
        }
        (native::Type::ISize, ConstantValue::ISize(value))
            if !(i32::MIN as i64..=i32::MAX as i64).contains(value) =>
        {
            let value = Literal::i64_suffixed(*value);
            return quote! { #value as isize };
        }
        _ => {}
    }
    let literal = match value {
        ConstantValue::Boolean(value) => return quote! { #value },
        ConstantValue::Char(value) | ConstantValue::U16(value) => Literal::u16_unsuffixed(*value),
        ConstantValue::I8(value) => Literal::i8_unsuffixed(*value),
        ConstantValue::U8(value) => Literal::u8_unsuffixed(*value),
        ConstantValue::I16(value) => Literal::i16_unsuffixed(*value),
        ConstantValue::I32(value) => Literal::i32_unsuffixed(*value),
        ConstantValue::U32(value) => Literal::u32_unsuffixed(*value),
        ConstantValue::I64(value) | ConstantValue::ISize(value) => Literal::i64_unsuffixed(*value),
        ConstantValue::U64(value) | ConstantValue::USize(value) => Literal::u64_unsuffixed(*value),
        ConstantValue::F32(value) => Literal::f32_unsuffixed(*value),
        ConstantValue::F64(value) => Literal::f64_unsuffixed(*value),
        ConstantValue::String(_) | ConstantValue::Null => unreachable!(),
    };
    quote! { #literal }
}

fn is_ansi(field: windows_metadata2::FieldDefinition<'_>) -> Result<bool, Error> {
    let Some(attribute) = field.find_attribute("NativeEncodingAttribute")? else {
        return Ok(false);
    };
    Ok(matches!(
        attribute.arguments(&())?.first(),
        Some(AttributeArgument::Fixed {
            value: AttributeValue::String(value),
            ..
        }) if value == "ansi"
    ))
}
