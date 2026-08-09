use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

/// An owned Win32 constant projection.
pub struct Constant {
    namespace: String,
    name: String,
    ty: native::Type,
    value: ConstantValue,
    ansi: bool,
}

impl Constant {
    pub(super) fn lower(
        database: &Database,
        field: windows_metadata2::FieldDefinition<'_>,
        namespace: &str,
        name: &str,
    ) -> Result<Self, Error> {
        let full_name = format!("{namespace}.{name}");
        let ty = native::Type::lower(
            database,
            field.entity().file(),
            &full_name,
            field.signature()?,
        )?;
        let value = field
            .constant()?
            .ok_or_else(|| Error::InvalidValue {
                name: full_name.clone(),
                message: "constant field has no Constant row",
            })?
            .value()?;
        if !ty.matches(&value) {
            return Err(Error::UnsupportedType {
                name: full_name,
                shape: format!("typed constant {ty:?} <- {value:?}"),
            });
        }
        Ok(Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            ty,
            value,
            ansi: is_ansi(field)?,
        })
    }

    /// Renders a flat Win32 constant.
    pub fn write(&self) -> TokenStream {
        let name = tokens::ident(&self.name);
        if let ConstantValue::String(value) = &self.value {
            if self.ansi {
                let bytes = value
                    .bytes()
                    .chain(std::iter::once(0))
                    .map(Literal::u8_unsuffixed);
                return quote! {
                    pub const #name: PCSTR = [#(#bytes),*].as_ptr();
                };
            }
            let units = value
                .encode_utf16()
                .chain(std::iter::once(0))
                .map(Literal::u16_unsuffixed);
            return quote! {
                pub const #name: PCWSTR = [#(#units),*].as_ptr();
            };
        }
        let ty = self.ty.write(&self.namespace);
        let value = write_value(&self.ty, &self.value);
        quote! { pub const #name: #ty = #value; }
    }
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
