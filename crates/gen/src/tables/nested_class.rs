use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NestedClass(pub Row);

impl NestedClass {
    pub fn nested_type(&self) -> TypeDef {
        Row::new(
            self.0.u32(0) - 1,
            TableIndex::TypeDef,
            self.0.file,
        ).into()
    }

    pub fn enclosing_type(&self) -> TypeDef {
        Row::new(
            self.0.u32(1) - 1,
            TableIndex::TypeDef,
            self.0.file,
        ).into()
    }
}

impl std::fmt::Debug for NestedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedClass")
            .field("nested_type", &self.nested_type())
            .field("enclosing_type", &self.enclosing_type())
            .finish()
    }
}
