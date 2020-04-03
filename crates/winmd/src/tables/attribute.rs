use super::TypeDef;
use crate::codes::{AttributeType, HasAttribute, MemberRefParent};
use crate::reader::Reader;
use crate::row::Row;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute(pub Row);

impl Attribute {
    pub fn parent(self, reader: &Reader) -> HasAttribute {
        reader.decode(self.0, 0)
    }

    pub fn constructor(self, reader: &Reader) -> AttributeType {
        reader.decode(self.0, 1)
    }

    pub fn name(self, reader: &Reader) -> (&str, &str) {
        match self.constructor(reader) {
            AttributeType::MethodDef(method) => method.parent(reader).name(reader),

            AttributeType::MemberRef(method) => match method.parent(reader) {
                MemberRefParent::TypeDef(parent) => parent.name(reader),
                MemberRefParent::TypeRef(parent) => parent.name(reader),
                _ => panic!(),
            },
        }
    }

    pub fn arguments(&self, reader: &Reader) -> Vec<(String, AttributeArgument)> {
        let (mut sig, mut values) = match self.constructor(reader) {
            AttributeType::MethodDef(method) => (reader.blob(method.0, 4), reader.blob(self.0, 2)),
            AttributeType::MemberRef(method) => (reader.blob(method.0, 2), reader.blob(self.0, 2)),
        };

        values.read_u16();
        sig.read_unsigned();
        let count = sig.read_unsigned();
        sig.read_unsigned();

        let mut args: Vec<(String, AttributeArgument)> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let arg = match sig.read_unsigned() {
                0x04 => AttributeArgument::I8(values.read_i8()),
                0x05 => AttributeArgument::U8(values.read_u8()),
                0x06 => AttributeArgument::I16(values.read_i16()),
                0x07 => AttributeArgument::U16(values.read_u16()),
                0x08 => AttributeArgument::I32(values.read_i32()),
                0x09 => AttributeArgument::U32(values.read_u32()),
                0x0A => AttributeArgument::I64(values.read_i64()),
                0x0B => AttributeArgument::U64(values.read_u64()),
                0x0E => AttributeArgument::String(values.read_str().to_string()),
                0x11 | 0x12 => {
                    sig.read_unsigned();
                    let name = values.read_str();
                    let index = name.rfind('.').unwrap();
                    AttributeArgument::TypeDef(
                        reader.resolve((&name[0..index], &name[index + 1..])),
                    )
                }
                _ => panic!(),
            };

            args.push((String::new(), arg));
        }

        println!("{:?}", args);

        let count = values.read_unsigned();
        args.reserve(count as usize);

        for _ in 0..count {
            let name = values.read_str().to_string();
            let arg = match values.read_unsigned() {
                0x02 => AttributeArgument::Bool(values.read_u8() != 0),
                0x08 => AttributeArgument::I32(values.read_i32()),
                0x0E => AttributeArgument::String(values.read_str().to_string()),
                0x50 => {
                    let name = values.read_str();
                    let index = name.rfind('.').unwrap();
                    AttributeArgument::TypeDef(
                        reader.resolve((&name[0..index], &name[index + 1..])),
                    )
                }
                // 0x55 => {
                //     let name = values.read_str();
                //     let index = name.rfind('.').unwrap();
                //     let def = reader.resolve((&name[0..index], &name[index + 1..]));
                //     def.fields(reader).next().unwrap().
                // }
                _ => panic!(),
            };
            args.push((name, arg));
        }

        args
    }
}

#[derive(Debug)]
pub enum AttributeArgument {
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
