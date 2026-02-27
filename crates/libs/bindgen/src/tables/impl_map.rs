use super::*;

pub trait ImplMapExt {
    /// Returns the scope (ModuleRef) for this import map entry.
    fn scope(&self) -> ModuleRef;
}

impl ImplMapExt for ImplMap {
    fn scope(&self) -> ModuleRef {
        self.import_scope()
    }
}
