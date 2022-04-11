use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute<'a>(pub Row<'a>);

impl<'a> Attribute<'a> {
    pub fn ty(&self) -> TypeRef {
        let AttributeType::MemberRef(member) = self.0.decode(1);
        let MemberRefParent::TypeRef(type_ref) = member.parent();
        type_ref
    }
    pub fn value(&self) -> Blob {
        self.0.blob(2)
    }
    // pub fn args(&self) -> Vec<(&str, Value)> {

    // }
}
