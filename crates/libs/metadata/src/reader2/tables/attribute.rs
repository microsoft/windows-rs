use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute<'a>(pub Row<'a>);

impl<'a> Attribute<'a> {
    pub fn ty(&self) -> AttributeType {
        self.0.decode(1)
    }
    pub fn value(&self) -> Blob {
        self.0.blob(2)
    }

    pub fn get(attributes: impl Iterator<Item = Self>, name: &str) -> Option<Self> {
        for attribute in attributes {
            let AttributeType::MemberRef(member) = attribute.ty();
            let MemberRefParent::TypeRef(ty) = member.parent();
            if ty.name() == name {
                return Some(attribute);
            }
        }
        None
    }
    // pub fn args(&self) -> Vec<(&str, Value)> {

    // }
}
