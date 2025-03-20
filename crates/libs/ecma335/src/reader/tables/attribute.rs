use super::*;

impl std::fmt::Debug for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Attribute")
            .field(&self.ctor().parent().name())
            .finish()
    }
}

impl Attribute<'_> {
    pub fn parent(&self) -> HasAttribute {
        self.decode(0)
    }

    pub fn ctor(&self) -> AttributeType {
        self.decode(1)
    }

    pub fn values(&self) -> Vec<(String, Value)> {
        let signature = self.ctor().signature(&[]);
        debug_assert_eq!(signature.flags, MethodCallAttributes::HASTHIS);
        debug_assert_eq!(signature.return_type, Type::Void);

        let mut values = Vec::with_capacity(signature.types.len());

        let mut blob = self.blob(2);

        let prolog = blob.read_u16();
        debug_assert_eq!(prolog, 1);

        for ty in &signature.types {
            let value = match ty {
                Type::Bool => Value::Bool(blob.read_bool()),
                Type::I8 => Value::I8(blob.read_i8()),
                Type::U8 => Value::U8(blob.read_u8()),
                Type::I16 => Value::I16(blob.read_i16()),
                Type::U16 => Value::U16(blob.read_u16()),
                Type::I32 => Value::I32(blob.read_i32()),
                Type::U32 => Value::U32(blob.read_u32()),
                Type::I64 => Value::I64(blob.read_i64()),
                Type::U64 => Value::U64(blob.read_u64()),
                Type::String | Type::Type => Value::String(blob.read_utf8()),
                Type::Name(..) => Value::I32(blob.read_i32()),
                rest => panic!("{rest:?}"),
            };

            values.push((String::new(), value));
        }

        let named_arg_count = blob.read_u16();
        values.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let _id = blob.read_u8();
            // TODO: what's ID?
            let arg_type = blob.read_u8();
            let mut name = blob.read_utf8();

            let value = match arg_type {
                ELEMENT_TYPE_BOOLEAN => Value::Bool(blob.read_bool()),
                ELEMENT_TYPE_I2 => Value::I16(blob.read_i16()),
                ELEMENT_TYPE_I4 => Value::I32(blob.read_i32()),
                ELEMENT_TYPE_U4 => Value::U32(blob.read_u32()),
                ELEMENT_TYPE_STRING => Value::String(blob.read_utf8()),
                0x50 => Value::String(blob.read_utf8()),
                0x55 => {
                    // previous `name` is the enum type name so we have to "re-read" the name to get the field name
                    name = blob.read_utf8();
                    Value::I32(blob.read_i32())
                }
                rest => panic!("{rest:?}"),
            };

            values.push((name, value));
        }

        debug_assert_eq!(blob.len(), 0);
        values
    }
}
