use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MethodDef(pub ScopeKey);

impl MethodDef {
    pub fn impl_flags(&self, scope: &Scope) -> MethodImplAttributes {
        MethodImplAttributes(scope.usize(self.0, 0))
    }
    pub fn flags(&self, scope: &Scope) -> MethodAttributes {
        MethodAttributes(scope.usize(self.0, 1))
    }
    pub fn name<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 3)
    }
    pub fn signature<'a>(&self, scope: &'a Scope) -> Blob<'a> {
        scope.blob(self.0, 4)
    }
    pub fn params(&self, scope: &Scope) -> impl Iterator<Item = Param> {
        scope.list(self.0, TABLE_PARAM, 4).map(Param)
    }
}
