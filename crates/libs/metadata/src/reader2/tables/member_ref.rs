use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MemberRef(pub ScopeKey);

impl MemberRef {
    pub fn parent(&self, scope: &Scope) -> MemberRefParent {
        scope.decode(self.0, 0)
    }
    pub fn signature<'a>(&self, scope: &'a Scope) -> Blob<'a> {
        scope.blob(self.0, 2)
    }
}
