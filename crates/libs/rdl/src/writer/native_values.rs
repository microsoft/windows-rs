use super::*;
use metadata::reader::{NativeStringEncoding, NativeValue};

pub fn write_native_value(value: &NativeValue) -> Result<TokenStream, Error> {
    Ok(match value {
        NativeValue::Scalar(value) => {
            macro_rules! scalar {
                ($method:ident, $value:expr) => {{
                    let value = Literal::$method(*$value);
                    quote! { #value }
                }};
            }
            match value {
                metadata::Value::Bool(value) => quote! { #value },
                metadata::Value::U8(value) => scalar!(u8_suffixed, value),
                metadata::Value::I8(value) => scalar!(i8_suffixed, value),
                metadata::Value::U16(value) => scalar!(u16_suffixed, value),
                metadata::Value::I16(value) => scalar!(i16_suffixed, value),
                metadata::Value::U32(value) => scalar!(u32_suffixed, value),
                metadata::Value::I32(value) => scalar!(i32_suffixed, value),
                metadata::Value::U64(value) => scalar!(u64_suffixed, value),
                metadata::Value::I64(value) => scalar!(i64_suffixed, value),
                metadata::Value::USize(value) => {
                    let value = syn::LitInt::new(&format!("{value}usize"), Span::call_site());
                    quote! { #value }
                }
                metadata::Value::ISize(value) => {
                    let value = syn::parse_str::<syn::Expr>(&format!("{value}isize"))
                        .map_err(|error| writer_err!("{error}"))?;
                    quote! { #value }
                }
                metadata::Value::F32(value) => {
                    let bits = Literal::u32_unsuffixed(value.to_bits());
                    quote! { f32bits(#bits) }
                }
                metadata::Value::F64(value) => {
                    let bits = Literal::u64_unsuffixed(value.to_bits());
                    quote! { f64bits(#bits) }
                }
                _ => return Err(writer_err!("unsupported native scalar value")),
            }
        }
        NativeValue::Array(values) => {
            let values = values
                .iter()
                .map(write_native_value)
                .collect::<Result<Vec<_>, _>>()?;
            quote! { [#(#values),*] }
        }
        NativeValue::Record(fields) => {
            let fields = fields
                .iter()
                .map(|(name, value)| {
                    let name = write_ident(name);
                    let value = write_native_value(value)?;
                    Ok(quote! { #name: #value, })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            quote! { record { #(#fields)* } }
        }
        NativeValue::String { encoding, units } => {
            let name = syn::Ident::new(
                match encoding {
                    NativeStringEncoding::Narrow => "narrow",
                    NativeStringEncoding::Utf8 => "utf8",
                    NativeStringEncoding::Utf16 => "utf16",
                    NativeStringEncoding::Utf32 => "utf32",
                },
                Span::call_site(),
            );
            let units = units.iter().map(|unit| Literal::u32_unsuffixed(*unit));
            quote! { #name([#(#units),*]) }
        }
    })
}
