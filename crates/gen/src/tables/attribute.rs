use super::*;
macros::table!(Attribute);

// TODO: replace with ElementType
#[derive(Debug)]
pub enum AttributeArg {
    Bool(bool),
    Char(char),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    TypeDef(TypeDef),
}

impl Attribute {
    pub fn constructor(&self) -> AttributeType {
        self.reader.decode(self.row, 1)
    }

    pub fn full_name(&self) -> (&'static str, &'static str) {
        if let AttributeType::MemberRef(method) = self.constructor() {
            return method.parent().full_name();
        }

        unexpected!();
    }

    pub fn args(&self) -> Vec<(String, AttributeArg)> {
        let (mut sig, mut values) = match self.constructor() {
            AttributeType::MethodDef(method) => (
                self.reader.blob(method.row, 4),
                self.reader.blob(self.row, 2),
            ),
            AttributeType::MemberRef(method) => (
                self.reader.blob(method.row, 2),
                self.reader.blob(self.row, 2),
            ),
        };

        let prolog = values.read_u16();
        debug_assert!(prolog == 0x0001, "CustomAttribute Prolog must be 0x0001"); // Required by spec.

        let _this_and_gen_param_count = sig.read_unsigned();
        let fixed_arg_count = sig.read_unsigned();
        let _ret_type = sig.read_unsigned();

        let mut args: Vec<(String, AttributeArg)> = Vec::with_capacity(fixed_arg_count as usize);

        for _ in 0..fixed_arg_count {
            let arg = match ElementType::from_blob(&mut sig, &[]) {
                ElementType::I8 => AttributeArg::I8(values.read_i8()),
                ElementType::U8 => AttributeArg::U8(values.read_u8()),
                ElementType::I16 => AttributeArg::I16(values.read_i16()),
                ElementType::U16 => AttributeArg::U16(values.read_u16()),
                ElementType::I32 => AttributeArg::I32(values.read_i32()),
                ElementType::U32 => AttributeArg::U32(values.read_u32()),
                ElementType::I64 => AttributeArg::I64(values.read_i64()),
                ElementType::U64 => AttributeArg::U64(values.read_u64()),
                ElementType::String => AttributeArg::String(values.read_str().to_string()),
                ElementType::TypeName => {
                    let name = values.read_str();
                    let index = name.rfind('.').unwrap();
                    AttributeArg::TypeDef(
                        self.reader
                            .resolve_type_def(&name[0..index], &name[index + 1..]),
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
                0x02 => AttributeArg::Bool(values.read_u8() != 0),
                0x08 => AttributeArg::I32(values.read_i32()),
                0x0E => AttributeArg::String(values.read_str().to_string()),
                0x50 => {
                    let name = values.read_str();
                    let index = name.rfind('.').unwrap();
                    AttributeArg::TypeDef(
                        self.reader
                            .resolve_type_def(&name[0..index], &name[index + 1..]),
                    )
                }
                _ => unexpected!(),
            };
            args.push((name, arg));
        }

        args
    }
}

fn read_enum(element_type: &ElementType, blob: &mut Blob) -> AttributeArg {
    match element_type {
        ElementType::I8 => AttributeArg::I8(blob.read_i8()),
        ElementType::U8 => AttributeArg::U8(blob.read_u8()),
        ElementType::I16 => AttributeArg::I16(blob.read_i16()),
        ElementType::U16 => AttributeArg::U16(blob.read_u16()),
        ElementType::I32 => AttributeArg::I32(blob.read_i32()),
        ElementType::U32 => AttributeArg::U32(blob.read_u32()),
        ElementType::I64 => AttributeArg::I64(blob.read_i64()),
        ElementType::U64 => AttributeArg::U64(blob.read_u64()),
        _ => unexpected!(),
    }
}
