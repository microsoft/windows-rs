use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute(pub winmd::Row);

impl Attribute {
    pub fn parent(self, reader: &TypeReader) -> winmd::HasAttribute {
        reader.decode(self.0, 0)
    }

    pub fn constructor(self, reader: &TypeReader) -> winmd::AttributeType {
        reader.decode(self.0, 1)
    }

    pub fn name(self, reader: &TypeReader) -> (&str, &str) {
        match self.constructor(reader) {
            winmd::AttributeType::MethodDef(method) => method.parent(reader).name(reader),

            winmd::AttributeType::MemberRef(method) => match method.parent(reader) {
                winmd::MemberRefParent::TypeDef(parent) => parent.name(reader),
                winmd::MemberRefParent::TypeRef(parent) => parent.name(reader),
                _ => panic!("Expected a TypeDef or TypeRef"),
            },
        }
    }

    pub fn args(self, reader: &TypeReader) -> Vec<(String, winmd::AttributeArg)> {
        let (mut sig, mut values) = match self.constructor(reader) {
            winmd::AttributeType::MethodDef(method) => {
                (reader.blob(method.0, 4), reader.blob(self.0, 2))
            }
            winmd::AttributeType::MemberRef(method) => {
                (reader.blob(method.0, 2), reader.blob(self.0, 2))
            }
        };

        let prolog = values.read_u16();
        debug_assert!(prolog == 0x0001, "CustomAttribute Prolog must be 0x0001"); // Required by spec.

        let _this_and_gen_param_count = sig.read_unsigned();
        let fixed_arg_count = sig.read_unsigned();
        let _ret_type = sig.read_unsigned();

        let mut args: Vec<(String, winmd::AttributeArg)> =
            Vec::with_capacity(fixed_arg_count as usize);

        for _ in 0..fixed_arg_count {
            let arg = match winmd::ElementType::from_blob(&mut sig) {
                winmd::ElementType::I1 => winmd::AttributeArg::I8(values.read_i8()),
                winmd::ElementType::U1 => winmd::AttributeArg::U8(values.read_u8()),
                winmd::ElementType::I2 => winmd::AttributeArg::I16(values.read_i16()),
                winmd::ElementType::U2 => winmd::AttributeArg::U16(values.read_u16()),
                winmd::ElementType::I4 => winmd::AttributeArg::I32(values.read_i32()),
                winmd::ElementType::U4 => winmd::AttributeArg::U32(values.read_u32()),
                winmd::ElementType::I8 => winmd::AttributeArg::I64(values.read_i64()),
                winmd::ElementType::U8 => winmd::AttributeArg::U64(values.read_u64()),
                winmd::ElementType::String => {
                    winmd::AttributeArg::String(values.read_str().to_string())
                }
                winmd::ElementType::ValueType(type_def_or_ref)
                | winmd::ElementType::Class(type_def_or_ref) => {
                    let (namespace, type_name) = match type_def_or_ref {
                        winmd::TypeDefOrRef::TypeDef(type_def) => type_def.name(reader),
                        winmd::TypeDefOrRef::TypeRef(type_ref) => type_ref.name(reader),
                        _ => panic!("Expected a TypeDef or TypeRef"),
                    };

                    if namespace == "System" && type_name == "Type" {
                        let name = values.read_str();
                        let index = name.rfind('.').unwrap();
                        winmd::AttributeArg::TypeDef(
                            reader.resolve_type_def((&name[0..index], &name[index + 1..])),
                        )
                    } else {
                        // Can't resolve it until we know it's an enum, because System.Type won't actually resolve.
                        let type_name = gen::TypeName::from_type_def_or_ref(
                            reader,
                            type_def_or_ref,
                            &Vec::new(),
                            "",
                        );

                        let e = gen::Enum::from_type_name(reader, type_name);
                        read_enum(&e.underlying_type, &mut values)
                    }
                }
                _ => panic!("Unexpected fixed attribute argument type"),
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
                0x02 => winmd::AttributeArg::Bool(values.read_u8() != 0),
                0x08 => winmd::AttributeArg::I32(values.read_i32()),
                0x0E => winmd::AttributeArg::String(values.read_str().to_string()),
                0x50 => {
                    let name = values.read_str();
                    let index = name.rfind('.').unwrap();
                    winmd::AttributeArg::TypeDef(
                        reader.resolve_type_def((&name[0..index], &name[index + 1..])),
                    )
                }
                _ => panic!("Unexpected named attribute argument type"),
            };
            args.push((name, arg));
        }

        args
    }
}

fn read_enum(element_type: &winmd::ElementType, blob: &mut winmd::Blob) -> winmd::AttributeArg {
    match element_type {
        winmd::ElementType::I1 => winmd::AttributeArg::I8(blob.read_i8()),
        winmd::ElementType::U1 => winmd::AttributeArg::U8(blob.read_u8()),
        winmd::ElementType::I2 => winmd::AttributeArg::I16(blob.read_i16()),
        winmd::ElementType::U2 => winmd::AttributeArg::U16(blob.read_u16()),
        winmd::ElementType::I4 => winmd::AttributeArg::I32(blob.read_i32()),
        winmd::ElementType::U4 => winmd::AttributeArg::U32(blob.read_u32()),
        winmd::ElementType::I8 => winmd::AttributeArg::I64(blob.read_i64()),
        winmd::ElementType::U8 => winmd::AttributeArg::U64(blob.read_u64()),
        _ => panic!("Invalid underlying enum type encountered!"),
    }
}
