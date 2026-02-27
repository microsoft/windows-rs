use super::*;

pub trait AttributeExt {
    fn args(&self) -> Vec<(&'static str, Value)>;
}

impl AttributeExt for Attribute {
    fn args(&self) -> Vec<(&'static str, Value)> {
        // Get the method signature to know parameter types.
        let signature = self.ctor().signature(&[]);

        let mut values = self.blob(2);
        let prolog = values.read_u16();
        debug_assert_eq!(prolog, 1);

        let mut args = Vec::with_capacity(signature.types.len());

        for ty in &signature.types {
            let arg = match ty {
                windows_metadata::Type::Bool => Value::Bool(values.read_bool()),
                windows_metadata::Type::I8 => Value::I8(values.read_i8()),
                windows_metadata::Type::U8 => Value::U8(values.read_u8()),
                windows_metadata::Type::I16 => Value::I16(values.read_i16()),
                windows_metadata::Type::U16 => Value::U16(values.read_u16()),
                windows_metadata::Type::I32 => Value::I32(values.read_i32()),
                windows_metadata::Type::U32 => Value::U32(values.read_u32()),
                windows_metadata::Type::I64 => Value::I64(values.read_i64()),
                windows_metadata::Type::U64 => Value::U64(values.read_u64()),
                windows_metadata::Type::String => Value::Str(values.read_str()),
                windows_metadata::Type::Name(tn)
                    if tn.namespace == "System" && tn.name == "Type" =>
                {
                    Value::TypeName(TypeName::parse(values.read_str()))
                }
                windows_metadata::Type::Name(tn) => {
                    // Enum type: look up underlying type then read the value.
                    let def = current_reader().unwrap_full_name(&tn.namespace, &tn.name);
                    let underlying_type = def.underlying_type();
                    values.read_integer(underlying_type)
                }
                rest => panic!("{rest:?}"),
            };

            args.push(("", arg));
        }

        let named_arg_count = values.read_u16();
        args.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let _id = values.read_u8();
            let arg_type = values.read_u8();
            let mut name = values.read_str();
            let arg = match arg_type {
                ELEMENT_TYPE_BOOLEAN => Value::Bool(values.read_bool()),
                ELEMENT_TYPE_I2 => Value::I16(values.read_i16()),
                ELEMENT_TYPE_I4 => Value::I32(values.read_i32()),
                ELEMENT_TYPE_U4 => Value::U32(values.read_u32()),
                ELEMENT_TYPE_STRING => Value::Str(values.read_str()),
                0x50 => Value::TypeName(TypeName::parse(values.read_str())),
                0x55 => {
                    let tn = TypeName::parse(name);
                    let def = current_reader().unwrap_full_name(tn.namespace(), tn.name());
                    name = values.read_str();
                    let underlying_type = def.underlying_type();
                    values.read_integer(underlying_type)
                }
                rest => panic!("{rest:?}"),
            };
            args.push((name, arg));
        }

        debug_assert_eq!(values.len(), 0);
        args
    }
}
