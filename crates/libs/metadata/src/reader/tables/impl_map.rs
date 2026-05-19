use super::*;

impl std::fmt::Debug for ImplMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ImplMap").field(&self.import_name()).finish()
    }
}

impl ImplMap {
    pub fn flags(&self) -> PInvokeAttributes {
        PInvokeAttributes(self.usize(0).try_into().unwrap())
    }

    pub fn import_name(&self) -> &str {
        self.str(2)
    }

    pub fn import_scope(&self) -> ModuleRef {
        self.row_at(3)
    }
}
