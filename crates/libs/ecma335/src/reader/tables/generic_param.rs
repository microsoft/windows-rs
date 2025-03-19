use super::*;

impl std::fmt::Debug for GenericParam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GenericParam").field(&self.name()).finish()
    }
}

impl GenericParam<'_> {
    pub fn sequence(&self) -> usize {
        self.usize(0)
    }

    pub fn flags(&self) -> usize {
        self.usize(1)
    }

    pub fn owner(&self) -> TypeOrMethodDef {
        self.decode(2)
    }

    pub fn name(&self) -> &str {
        self.str(3)
    }
}
