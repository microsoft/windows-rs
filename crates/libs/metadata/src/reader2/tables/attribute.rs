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
}
