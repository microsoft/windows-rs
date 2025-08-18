use super::*;

impl std::fmt::Debug for ModuleRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ModuleRef").field(&self.name()).finish()
    }
}

impl ModuleRef<'_> {
    pub fn name(&self) -> &str {
        self.str(0)
    }
}
