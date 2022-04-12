use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct InterfaceImpl(pub ScopeKey);

impl InterfaceImpl {
    pub fn interface(&self, scope: &Scope) -> TypeDefOrRef {
        scope.decode(self.0, 1)
    }
    fn attributes(&self, scope: &Scope) -> impl Iterator<Item = Attribute> {
        scope.attributes(self.0, HasAttribute::InterfaceImpl(*self))
    }
}
