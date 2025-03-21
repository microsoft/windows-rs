use super::*;

impl std::fmt::Debug for NestedClass<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedClass")
            .field("inner", &self.inner())
            .field("outer", &self.outer())
            .finish()
    }
}

impl NestedClass<'_> {
    pub fn inner(&self) -> TypeDef {
        self.row(0)
    }

    pub fn outer(&self) -> TypeDef {
        self.row(1)
    }
}
