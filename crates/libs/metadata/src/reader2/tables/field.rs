use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field(pub ScopeKey);

impl Field {
    pub fn flags(&self, scope: &Scope) -> FieldAttributes {
        FieldAttributes(scope.usize(self.0, 0))
    }
    pub fn name<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 1)
    }
    pub fn signature<'a>(&self, scope: &'a Scope) -> Blob<'a> {
        scope.blob(self.0, 2)
    }
}
