use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ClassLayout(pub ScopeKey);

impl ClassLayout {
    pub fn packing_size(&self, scope: &Scope) -> usize {
        scope.usize(self.0, 0)
    }
}
