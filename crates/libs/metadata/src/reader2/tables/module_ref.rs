use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ModuleRef(pub ScopeKey);

impl ModuleRef {
    pub fn name<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 0)
    }
}
