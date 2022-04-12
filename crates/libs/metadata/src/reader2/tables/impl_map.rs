use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ImplMap(pub ScopeKey);

impl ImplMap {
    pub fn flags(&self, scope: &Scope) -> PInvokeAttributes {
        PInvokeAttributes(scope.usize(self.0, 0))
    }
    pub fn scope(&self, scope: &Scope) -> ModuleRef {
        ModuleRef(ScopeKey::new(scope.usize(self.0, 3) - 1, TABLE_MODULEREF, self.0.file as _))
    }
}
