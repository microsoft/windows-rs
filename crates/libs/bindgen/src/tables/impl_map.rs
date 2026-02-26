use super::*;

pub trait ImplMapExt {
    fn scope(&self) -> ModuleRef;
}

impl ImplMapExt for ImplMap {
    fn scope(&self) -> ModuleRef {
        self.import_scope()
    }
}

