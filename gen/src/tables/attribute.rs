use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute(pub Row);

impl Attribute {
    pub fn parent(&self, reader: &Reader) -> HasAttribute {
        reader.decode(self.0, 0)
    }

    pub fn constructor(&self, reader: &Reader) -> AttributeType {
        reader.decode(self.0, 1)
    }

    pub fn name<'a>(&self, reader: &'a Reader) -> (&'a str, &'a str) {
        match self.constructor(reader) {
            AttributeType::MethodDef(method) => method.parent(reader).name(reader),

            AttributeType::MemberRef(method) => match method.parent(reader) {
                MemberRefParent::TypeDef(parent) => parent.name(reader),
                MemberRefParent::TypeRef(parent) => parent.name(reader),
                _ => panic!(),
            },
        }
    }

    pub fn arguments<'a>(&self, reader: &'a Reader) -> Vec<(String, Argument)> {
        let (sig, values) = match self.constructor(reader) {
            AttributeType::MethodDef(method) => (reader.blob(method.0, 4), reader.blob(self.0, 2)),
            AttributeType::MemberRef(method) => (reader.blob(method.0, 2), reader.blob(self.0, 2)),
        };

        Vec::new()
    }
}

pub enum Argument {
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
