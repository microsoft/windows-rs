use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericParam(pub ScopeKey);

impl GenericParam {
    pub fn name<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 3)
    }
}
