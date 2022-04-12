use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeSpec(pub ScopeKey);

impl TypeSpec {
    pub fn signature<'a>(&self, scope: &'a Scope) -> Blob<'a> {
        scope.blob(self.0, 0)
    }
}
