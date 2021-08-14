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
