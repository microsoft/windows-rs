use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct InterfaceImpl<'a>(pub Row<'a>);

impl<'a> InterfaceImpl<'a> {
    pub fn interface(&self) -> TypeDefOrRef {
        self.0.decode(1)
    }
    fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0.attributes(HasAttribute::InterfaceImpl(self.clone()))
    }
}
