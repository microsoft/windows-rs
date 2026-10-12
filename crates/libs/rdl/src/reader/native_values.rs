use super::*;
use metadata::reader::{NativeStringEncoding, NativeValue};

impl Encoder<'_> {
    pub fn encode_native_value(
        &self,
        expr: &syn::Expr,
        depth: usize,
    ) -> Result<NativeValue, Error> {
        if depth > 64 {
            return self.err(expr, "native value nesting exceeds 64");
        }
        match expr {
            syn::Expr::Array(array) => Ok(NativeValue::Array(
                array
                    .elems
                    .iter()
                    .map(|value| self.encode_native_value(value, depth + 1))
                    .collect::<Result<_, _>>()?,
            )),
            syn::Expr::Struct(record) if record.path.is_ident("record") => {
                if record.rest.is_some() {
                    return self.err(expr, "native records require every member value");
                }
                let mut fields = Vec::new();
                for field in &record.fields {
                    let syn::Member::Named(name) = &field.member else {
                        return self.err(field, "native records require named members");
                    };
                    if !field.attrs.is_empty() {
                        return self
                            .err(field, "native initializer members cannot have attributes");
                    }
                    fields.push((
                        name.unraw_to_string(),
                        self.encode_native_value(&field.expr, depth + 1)?,
                    ));
                }
                Ok(NativeValue::Record(fields))
            }
            syn::Expr::Call(call) => {
                let syn::Expr::Path(function) = call.func.as_ref() else {
                    return self.err(expr, "unsupported native value expression");
                };
                if call.args.len() != 1 {
                    return self.err(expr, "native value constructors require one argument");
                }
                let argument = &call.args[0];
                if function.path.is_ident("f32bits") {
                    let bits = self.encode_lit_int::<u32>(argument)?;
                    return Ok(NativeValue::Scalar(metadata::Value::F32(f32::from_bits(
                        bits,
                    ))));
                }
                if function.path.is_ident("f64bits") {
                    let bits = self.encode_lit_int::<u64>(argument)?;
                    return Ok(NativeValue::Scalar(metadata::Value::F64(f64::from_bits(
                        bits,
                    ))));
                }
                let (encoding, width) = if function.path.is_ident("narrow") {
                    (NativeStringEncoding::Narrow, 8)
                } else if function.path.is_ident("utf8") {
                    (NativeStringEncoding::Utf8, 8)
                } else if function.path.is_ident("utf16") {
                    (NativeStringEncoding::Utf16, 16)
                } else if function.path.is_ident("utf32") {
                    (NativeStringEncoding::Utf32, 32)
                } else {
                    return self.err(expr, "unsupported native value constructor");
                };
                let syn::Expr::Array(array) = argument else {
                    return self.err(
                        argument,
                        "native string requires an explicit code-unit array",
                    );
                };
                let units = array
                    .elems
                    .iter()
                    .map(|unit| match width {
                        8 => self.encode_lit_int::<u8>(unit).map(u32::from),
                        16 => self.encode_lit_int::<u16>(unit).map(u32::from),
                        _ => self.encode_lit_int::<u32>(unit),
                    })
                    .collect::<Result<_, _>>()?;
                Ok(NativeValue::String { encoding, units })
            }
            _ => {
                let literal = if let syn::Expr::Unary(unary) = expr {
                    unary.expr.as_ref()
                } else {
                    expr
                };
                let ty = match literal {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Bool(_),
                        ..
                    }) => metadata::Type::Bool,
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(value),
                        ..
                    }) => match value.suffix() {
                        "u8" => metadata::Type::U8,
                        "i8" => metadata::Type::I8,
                        "u16" => metadata::Type::U16,
                        "i16" => metadata::Type::I16,
                        "u32" => metadata::Type::U32,
                        "i32" => metadata::Type::I32,
                        "u64" => metadata::Type::U64,
                        "i64" => metadata::Type::I64,
                        "usize" => metadata::Type::USize,
                        "isize" => metadata::Type::ISize,
                        _ => return self.err(expr, "native value integers require a type suffix"),
                    },
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Float(value),
                        ..
                    }) => match value.suffix() {
                        "f32" => metadata::Type::F32,
                        "f64" => metadata::Type::F64,
                        _ => return self.err(expr, "native value floats require a type suffix"),
                    },
                    _ => return self.err(expr, "unsupported native value expression"),
                };
                let value = match ty {
                    metadata::Type::I8 => metadata::Value::I8(self.encode_neg_lit_int(expr)?),
                    metadata::Type::U8 => metadata::Value::U8(self.encode_lit_int(expr)?),
                    metadata::Type::I16 => metadata::Value::I16(self.encode_neg_lit_int(expr)?),
                    metadata::Type::U16 => metadata::Value::U16(self.encode_lit_int(expr)?),
                    metadata::Type::I32 => metadata::Value::I32(self.encode_neg_lit_int(expr)?),
                    metadata::Type::U32 => metadata::Value::U32(self.encode_lit_int(expr)?),
                    metadata::Type::I64 => metadata::Value::I64(self.encode_neg_lit_int(expr)?),
                    metadata::Type::U64 => metadata::Value::U64(self.encode_lit_int(expr)?),
                    metadata::Type::ISize => metadata::Value::ISize(self.encode_neg_lit_int(expr)?),
                    metadata::Type::USize => metadata::Value::USize(self.encode_lit_int(expr)?),
                    _ => self.encode_value(&ty, expr)?,
                };
                if matches!(value, metadata::Value::F32(value) if !value.is_finite())
                    || matches!(value, metadata::Value::F64(value) if !value.is_finite())
                {
                    return self.err(
                        expr,
                        "nonfinite native floats require explicit bit constructors",
                    );
                }
                Ok(NativeValue::Scalar(value))
            }
        }
    }

    pub fn encode_native_const(&mut self, ty: &metadata::Type, item: &Const) -> Result<(), Error> {
        if item.winrt {
            return self.err(&item.name, "native values require a native Apis container");
        }
        let expr = item.expr.as_ref().unwrap();
        let value = self.encode_native_value(expr, 0)?;
        let payload = value
            .to_payload()
            .map_err(|error| self.error(expr, &error.to_string()))?;
        let field = self.output.Field(
            &item.name.to_string(),
            ty,
            metadata::FieldAttributes::Public | metadata::FieldAttributes::Static,
        );
        self.encode_named_attribute(
            metadata::writer::HasAttribute::Field(field),
            &attribute_ref::AttributeRef {
                type_name: metadata::TypeName::named(METADATA_NAMESPACE, "NativeConstantAttribute"),
                args: vec![
                    (String::new(), metadata::Value::I32(1)),
                    (String::new(), metadata::Value::Utf8(payload)),
                ],
            },
        );
        self.encode_attrs(
            metadata::writer::HasAttribute::Field(field),
            &item.attrs,
            &[],
        )?;
        if let Some(arches) = self.read_arch(&item.attrs)? {
            self.emit_arch_attribute(metadata::writer::HasAttribute::Field(field), arches);
        }
        Ok(())
    }
}
