use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct ModuleRef {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl ModuleRef {
    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 0)
    }
}
