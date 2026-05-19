use super::*;

impl std::fmt::Debug for NestedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NestedClass")
            .field("inner", &self.inner())
            .field("outer", &self.outer())
            .finish()
    }
}

impl NestedClass {
    pub fn inner(&self) -> TypeDef {
        self.row_at(0)
    }

    pub fn outer(&self) -> TypeDef {
        self.row_at(1)
    }
}
