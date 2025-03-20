use super::*;

impl std::fmt::Debug for ImplMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ImplMap").field(&self.import_name()).finish()
    }
}

impl ImplMap<'_> {
    pub fn flags(&self) -> PInvokeAttributes {
        PInvokeAttributes(self.usize(0))
    }

    pub fn scope(&self) -> ModuleRef {
        ModuleRef(self.row(3))
    }

    pub fn import_name(&self) -> &str {
        self.str(2)
    }
}
