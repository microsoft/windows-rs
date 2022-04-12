use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub ScopeKey);

impl Constant {
    pub fn ty(&self, _scope: &Scope) -> Type {
        todo!()
    }
    pub fn value<'a>(&self, scope: &'a Scope) -> Blob<'a> {
        scope.blob(self.0, 2)
    }
}
