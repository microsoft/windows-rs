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
}
