use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Param(pub ScopeKey);

impl Param {
    pub fn flags(&self, scope: &Scope) -> ParamAttributes {
        ParamAttributes(scope.usize(self.0, 0))
    }
    pub fn sequence(&self, scope: &Scope) -> usize {
        scope.usize(self.0, 1)
    }
    pub fn name<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 2)
    }
}
