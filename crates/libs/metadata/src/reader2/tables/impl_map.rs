use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ImplMap<'a>(pub Row<'a>);

impl<'a> ImplMap<'a> {
    pub fn flags(&self) -> PInvokeAttributes {
        PInvokeAttributes(self.0.usize(0))
    }
    pub fn scope(&self) -> ModuleRef {
        ModuleRef(Row::new(self.0.scope, ScopeKey::new(self.0.usize(3) - 1, TABLE_MODULEREF, self.0.key.file as _)))
    }
}
