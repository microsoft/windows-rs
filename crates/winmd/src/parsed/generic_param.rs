use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct GenericParam {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl GenericParam {
    pub fn name<'a>(&self) -> &'a str {
        self.reader.str(self.row, 3)
    }
}

impl std::fmt::Debug for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericParam")
            .field("row", &self.row)
            .finish()
    }
}
