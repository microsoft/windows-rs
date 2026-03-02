use super::*;

impl std::fmt::Debug for ModuleRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ModuleRef").field(&self.name()).finish()
    }
}

impl<'a> ModuleRef<'a> {
    pub fn name(&self) -> &'a str {
        self.str(0)
    }
}
