use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct TypeSpec {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl TypeSpec {
    pub fn sig(&self) -> Blob {
        self.reader.blob(self.row, 0)
    }
}

impl std::fmt::Debug for TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeSpec").field("row", &self.row).finish()
    }
}
