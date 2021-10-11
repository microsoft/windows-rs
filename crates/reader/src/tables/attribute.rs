use super::*;

#[derive(Clone)]
pub struct Attribute(pub Row);

impl Attribute {
    pub fn name(&self) -> &'static str {
        if let AttributeType::MemberRef(method) = self.0.decode(1) {
            return method.parent().name();
        }

        unimplemented!();
    }

    pub fn args(&self) -> Vec<(String, ConstantValue)> {
        let reader = TypeReader::get();

        let (mut sig, mut values) = match self.0.decode(1) {
            AttributeType::MethodDef(method) => (method.0.blob(4), self.0.blob(2)),
            AttributeType::MemberRef(method) => (method.0.blob(2), self.0.blob(2)),
        };

        let _prolog = values.read_u16();
        let _this_and_gen_param_count = sig.read_unsigned();
        let fixed_arg_count = sig.read_unsigned();
        let _ret_type = sig.read_unsigned();

        let mut args: Vec<(String, ConstantValue)> = Vec::with_capacity(fixed_arg_count as usize);

        for _ in 0..fixed_arg_count {
            let arg = match reader.type_from_blob(&mut sig, None, &[]) {
                ElementType::I8 => ConstantValue::I8(values.read_i8()),
                ElementType::U8 => ConstantValue::U8(values.read_u8()),
                ElementType::I16 => ConstantValue::I16(values.read_i16()),
                ElementType::U16 => ConstantValue::U16(values.read_u16()),
                ElementType::I32 => ConstantValue::I32(values.read_i32()),
                ElementType::U32 => ConstantValue::U32(values.read_u32()),
                ElementType::I64 => ConstantValue::I64(values.read_i64()),
                ElementType::U64 => ConstantValue::U64(values.read_u64()),
                ElementType::String => ConstantValue::String(values.read_str().to_string()),
                ElementType::TypeName => {
                    let name = values.read_str();
                    ConstantValue::TypeDef(reader.expect_type_def(TypeName::parse(name)).clone())
                }
                ElementType::TypeDef(def) => match def.underlying_type() {
                    ElementType::I8 => ConstantValue::I8(values.read_i8()),
                    ElementType::U8 => ConstantValue::U8(values.read_u8()),
                    ElementType::I16 => ConstantValue::I16(values.read_i16()),
                    ElementType::U16 => ConstantValue::U16(values.read_u16()),
                    ElementType::I32 => ConstantValue::I32(values.read_i32()),
                    ElementType::U32 => ConstantValue::U32(values.read_u32()),
                    ElementType::I64 => ConstantValue::I64(values.read_i64()),
                    ElementType::U64 => ConstantValue::U64(values.read_u64()),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            };

            args.push((String::new(), arg));
        }

        let named_arg_count = values.read_u16();
        args.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let _id = values.read_u8();
            let arg_type = values.read_u8();
            let name = values.read_str().to_string();
            let arg = match arg_type {
                0x02 => ConstantValue::Bool(values.read_u8() != 0),
                0x08 => ConstantValue::I32(values.read_i32()),
                0x09 => ConstantValue::U32(values.read_u32()),
                0x0E => ConstantValue::String(values.read_str().to_string()),
                0x50 => {
                    let name = values.read_str();
                    ConstantValue::TypeDef(reader.expect_type_def(TypeName::parse(name)).clone())
                }
                _ => unimplemented!(),
            };
            args.push((name, arg));
        }

        args
    }

    // Extracts the public type, if any, of a ComposableAttribute blob.
    pub fn composable_type(&self) -> Option<TypeDef> {
        let mut public = false;
        let mut def = None;

        // One of the arguments is a CompositionType enum and the Public variant
        // has a value of 2 as a signed 32-bit integer.

        for (_, arg) in self.args() {
            match arg {
                ConstantValue::I32(2) => public = true,
                ConstantValue::TypeDef(value) => def = Some(value),
                _ => {}
            }
        }

        if public {
            def
        } else {
            None
        }
    }
}
