use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute(pub Row);

impl Attribute {
    pub fn constructor(&self) -> AttributeType {
        self.0.decode(1)
    }

    pub fn name(&self) -> &'static str {
        if let AttributeType::MemberRef(method) = self.constructor() {
            return method.parent().name();
        }

        unexpected!();
    }

    pub fn args(&self) -> Vec<(String, ConstantValue)> {
        let (mut sig, mut values) = match self.constructor() {
            AttributeType::MethodDef(method) => (method.0.blob(4), self.0.blob(2)),
            AttributeType::MemberRef(method) => (method.0.blob(2), self.0.blob(2)),
        };

        let prolog = values.read_u16();
        debug_assert!(prolog == 0x0001, "CustomAttribute Prolog must be 0x0001"); // Required by spec.

        let _this_and_gen_param_count = sig.read_unsigned();
        let fixed_arg_count = sig.read_unsigned();
        let _ret_type = sig.read_unsigned();

        let mut args: Vec<(String, ConstantValue)> = Vec::with_capacity(fixed_arg_count as usize);

        for _ in 0..fixed_arg_count {
            let arg = match ElementType::from_blob(&mut sig, &[]) {
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
                    let index = name.rfind('.').unwrap();
                    ConstantValue::TypeDef(
                        TypeReader::get().resolve_type_def(&name[0..index], &name[index + 1..]),
                    )
                }
                ElementType::Enum(def) => {
                    let underlying_type = def.underlying_type();
                    read_enum(&underlying_type, &mut values)
                }
                _ => unexpected!(),
            };

            args.push((String::new(), arg));
        }

        let named_arg_count = values.read_u16();
        args.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let id = values.read_u8();
            debug_assert!(
                id == 0x53 || id == 0x54,
                "A NamedArg must start with an id of 0x53 (Field) or 0x54 (Property)"
            );
            let arg_type = values.read_u8();
            let name = values.read_str().to_string();
            let arg = match arg_type {
                0x02 => ConstantValue::Bool(values.read_u8() != 0),
                0x08 => ConstantValue::I32(values.read_i32()),
                0x09 => ConstantValue::U32(values.read_u32()),
                0x0E => ConstantValue::String(values.read_str().to_string()),
                0x50 => {
                    let name = values.read_str();
                    let index = name.rfind('.').unwrap();
                    ConstantValue::TypeDef(
                        TypeReader::get().resolve_type_def(&name[0..index], &name[index + 1..]),
                    )
                }
                _ => unexpected!(),
            };
            args.push((name, arg));
        }

        args
    }
}

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn read_enum(element_type: &ElementType, blob: &mut Blob) -> ConstantValue {
    match element_type {
        ElementType::I8 => ConstantValue::I8(blob.read_i8()),
        ElementType::U8 => ConstantValue::U8(blob.read_u8()),
        ElementType::I16 => ConstantValue::I16(blob.read_i16()),
        ElementType::U16 => ConstantValue::U16(blob.read_u16()),
        ElementType::I32 => ConstantValue::I32(blob.read_i32()),
        ElementType::U32 => ConstantValue::U32(blob.read_u32()),
        ElementType::I64 => ConstantValue::I64(blob.read_i64()),
        ElementType::U64 => ConstantValue::U64(blob.read_u64()),
        _ => unexpected!(),
    }
}
