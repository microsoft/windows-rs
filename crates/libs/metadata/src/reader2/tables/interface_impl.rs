use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct InterfaceImpl(pub Row);

// impl InterfaceImpl {
//     pub fn interface(&self, scope: &Reader) -> TypeDefOrRef {
//         scope.decode(self.0, 1)
//     }
//     fn attributes(&self, scope: &Reader) -> impl Iterator<Item = Attribute> {
//         scope.attributes(self.0, HasAttribute::InterfaceImpl(*self))
//     }
// }
