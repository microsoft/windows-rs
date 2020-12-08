use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct Constant {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl Constant {
    pub fn value_type(&self) -> ElementType {
        ElementType::from_code(self.reader.u32(self.row, 0))
    }

    pub fn value(&self) -> Blob {
        self.reader.blob(self.row, 2)
    }
}

impl std::fmt::Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constant").field("row", &self.row).finish()
    }
}
