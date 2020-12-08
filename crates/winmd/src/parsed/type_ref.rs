use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct TypeRef {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl TypeRef {
    pub fn name(&self) -> (&'static str, &'static str) {
        (self.reader.str(self.row, 2), self.reader.str(self.row, 1))
    }

    pub fn resolve(&self) -> TypeDef {
        self.reader.resolve_type_def(self.name())
    }
}

impl std::fmt::Debug for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeRef").field("row", &self.row).finish()
    }
}
