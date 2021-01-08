use super::*;
use crate::{TableIndex, TypeReader};

#[derive(Copy, Clone)]
pub struct ImplMap {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl ImplMap {
    pub fn scope(&self) -> ModuleRef {
        let index = self.reader.u32(self.row, 3) - 1;
        let row = Row::new(index, TableIndex::ModuleRef, self.row.file_index);

        ModuleRef {
            reader: self.reader,
            row,
        }
    }
}
