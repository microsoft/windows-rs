use super::*;

impl std::fmt::Debug for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Attribute")
            .field(&self.ctor().parent().name())
            .finish()
    }
}

impl<'a> Attribute<'a> {
    pub fn name(&self) -> &'a str {
        self.ctor().parent().name()
    }

    pub fn parent(&self) -> HasAttribute<'a> {
        self.decode(0)
    }

    pub fn ctor(&self) -> AttributeType<'a> {
        self.decode(1)
    }

    pub fn value(&self) -> Vec<(String, Value)> {
        let signature = self.ctor().signature(&[]);
        debug_assert_eq!(signature.flags, MethodCallAttributes::HASTHIS);
        debug_assert_eq!(signature.return_type, Type::Void);

        let mut values = Vec::with_capacity(signature.types.len());
        let mut blob = self.blob(2);
        let prolog = blob.read_u16();
        debug_assert_eq!(prolog, 1);

        for ty in &signature.types {
            let value = read_value(&mut blob, ty);
            values.push((String::new(), value));
        }

        let named_arg_count = blob.read_u16();
        values.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let _id = blob.read_u8();
            // Per ECMA-335 §II.23.3, this byte is 0x53 (FIELD) or 0x54 (PROPERTY),
            // indicating whether the named argument targets a field or a property.
            let ty = blob.read_type_code(&[]);
            let name = blob.read_utf8();
            let value = read_value(&mut blob, &ty);
            values.push((name, value));
        }

        debug_assert_eq!(blob.len(), 0);
        values
    }
}

fn read_value(blob: &mut Blob, ty: &Type) -> Value {
    match ty {
        Type::Bool => Value::Bool(blob.read_bool()),
        Type::I8 => Value::I8(blob.read_i8()),
        Type::U8 => Value::U8(blob.read_u8()),
        Type::I16 => Value::I16(blob.read_i16()),
        Type::U16 => Value::U16(blob.read_u16()),
        Type::I32 => Value::I32(blob.read_i32()),
        Type::U32 => Value::U32(blob.read_u32()),
        Type::I64 => Value::I64(blob.read_i64()),
        Type::U64 => Value::U64(blob.read_u64()),
        Type::F32 => Value::F32(blob.read_f32()),
        Type::F64 => Value::F64(blob.read_f64()),
        Type::String => Value::Utf8(blob.read_utf8()),
        Type::Name(tn) => {
            if tn.namespace == "System" && tn.name == "Type" {
                let s = blob.read_utf8();
                if let Some(dot) = s.rfind('.') {
                    Value::TypeName(TypeName::named(&s[..dot], &s[dot + 1..]))
                } else {
                    Value::TypeName(TypeName::named("", &s))
                }
            } else {
                Value::EnumValue(tn.clone(), Box::new(Value::I32(blob.read_i32())))
            }
        }
        rest => panic!("{rest:?}"),
    }
}
