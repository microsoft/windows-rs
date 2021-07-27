use super::*;

#[derive(Clone)]
pub struct NestedClass(pub Row);

impl NestedClass {
    pub fn nested_type(&self) -> TypeDef {
        Row::new(self.0.u32(0) - 1, TableIndex::TypeDef, self.0.file).into()
    }

    pub fn enclosing_type(&self) -> TypeDef {
        Row::new(self.0.u32(1) - 1, TableIndex::TypeDef, self.0.file).into()
    }
}
