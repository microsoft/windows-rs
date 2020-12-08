use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct Attribute {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl Attribute {
    pub fn parent(&self) -> HasAttribute {
        self.reader.decode(self.row, 0)
    }

    pub fn constructor(&self) -> AttributeType {
        self.reader.decode(self.row, 1)
    }

    pub fn name(&self) -> (&'static str, &'static str) {
        match self.constructor() {
            AttributeType::MethodDef(method) => method.parent().name(),

            AttributeType::MemberRef(method) => match method.parent() {
                MemberRefParent::TypeDef(parent) => parent.name(),
                MemberRefParent::TypeRef(parent) => parent.name(),
                _ => panic!("Expected a TypeDef or TypeRef"),
            },
        }
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
            let arg = match ElementType::from_blob(&mut sig) {
                ElementType::I8 => AttributeArg::I8(values.read_i8()),
                ElementType::U8 => AttributeArg::U8(values.read_u8()),
                ElementType::I16 => AttributeArg::I16(values.read_i16()),
                ElementType::U16 => AttributeArg::U16(values.read_u16()),
                ElementType::I32 => AttributeArg::I32(values.read_i32()),
                ElementType::U32 => AttributeArg::U32(values.read_u32()),
                ElementType::I64 => AttributeArg::I64(values.read_i64()),
                ElementType::U64 => AttributeArg::U64(values.read_u64()),
                ElementType::String => AttributeArg::String(values.read_str().to_string()),
                ElementType::Struct(type_def_or_ref) | ElementType::Class(type_def_or_ref) => {
                    let (namespace, type_name) = match type_def_or_ref {
                        TypeDefOrRef::TypeDef(type_def) => type_def.name(),
                        TypeDefOrRef::TypeRef(type_ref) => type_ref.name(),
                        _ => panic!("Expected a TypeDef or TypeRef"),
                    };

                    if namespace == "System" && type_name == "Type" {
                        let name = values.read_str();
                        let index = name.rfind('.').unwrap();
                        AttributeArg::TypeDef(
                            self.reader
                                .resolve_type_def((&name[0..index], &name[index + 1..])),
                        )
                    } else {
                        let def = match type_def_or_ref {
                            TypeDefOrRef::TypeRef(value) => {
                                self.reader.resolve_type_def(value.name())
                            }
                            TypeDefOrRef::TypeDef(value) => value,
                            TypeDefOrRef::TypeSpec(_) => panic!("Unsupported underlying type"),
                        };

                        let underlying_type = def.underlying_type();
                        read_enum(&underlying_type, &mut values)
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
                0x02 => AttributeArg::Bool(values.read_u8() != 0),
                0x08 => AttributeArg::I32(values.read_i32()),
                0x0E => AttributeArg::String(values.read_str().to_string()),
                0x50 => {
                    let name = values.read_str();
                    let index = name.rfind('.').unwrap();
                    AttributeArg::TypeDef(
                        self.reader
                            .resolve_type_def((&name[0..index], &name[index + 1..])),
                    )
                }
                _ => panic!("Unexpected named attribute argument type"),
            };
            args.push((name, arg));
        }

        args
    }
}

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attribute").field("row", &self.row).finish()
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
        _ => panic!("Invalid underlying enum type encountered!"),
    }
}
