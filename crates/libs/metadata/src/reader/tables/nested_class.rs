use super::*;

impl std::fmt::Debug for NestedClass<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NestedClass")
            .field("inner", &self.inner())
            .field("outer", &self.outer())
            .finish()
    }
}

impl<'a> NestedClass<'a> {
    pub fn inner(&self) -> TypeDef<'a> {
        self.row(0)
    }

    pub fn outer(&self) -> TypeDef<'a> {
        self.row(1)
    }
}
