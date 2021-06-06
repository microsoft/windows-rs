use super::*;

#[derive(Clone)]
pub struct ImplMap(pub Row);

impl ImplMap {
    pub fn scope(&self) -> ModuleRef {
        ModuleRef(Row::new(
            self.0.u32(3) - 1,
            TableIndex::ModuleRef,
            self.0.file,
        ))
    }
}

impl std::fmt::Debug for ImplMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scope().name())
    }
}
