use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

/// An owned Win32 constant projection.
pub struct Constant {
    architectures: i32,
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
        let architectures = field.architectures()?;
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
                architectures,
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
            architectures,
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
        let architectures = tokens::architectures(self.architectures);
        let name = tokens::ident(&self.name);
        let value = match &self.value {
            Value::Guid(guid) => {
                let guid = guid.write_value();
                quote! {
                    pub const #name: GUID = #guid;
                }
            }
            Value::PropertyKey { guid, fields, pid } => {
                let ty = self.ty.write(&self.namespace);
                let guid = guid.write_value();
                let guid_field = tokens::ident(&fields[0]);
                let pid_field = tokens::ident(&fields[1]);
                let pid = native::write_value(&native::Type::from_constant(pid), pid);
                quote! {
                    pub const #name: #ty = #ty {
                        #guid_field: #guid,
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
                    native::write_value(underlying, value)
                } else {
                    Self::write_converted(underlying, value)
                };
                quote! { pub const #name: #ty = #value; }
            }
        };
        quote! { #architectures #value }
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
            return native::write_value(underlying, value);
        }
        let value = native::write_value(&native::Type::from_constant(value), value);
        quote! { #value as _ }
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
