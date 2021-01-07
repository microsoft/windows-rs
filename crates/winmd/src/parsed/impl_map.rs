use super::*;
use crate::{TableIndex, TypeReader};

#[derive(Copy, Clone)]
pub struct ImplMap {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl ImplMap {
    pub fn flags(&self) -> u32 {
        self.reader.u32(self.row, 0)
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 2)
    }

    pub fn scope(&self) -> &'static str {
        let index = self.reader.u32(self.row, 3) - 1;
        let row = Row::new(index, TableIndex::ModuleRef, self.row.file_index);

        let module = ModuleRef {
            reader: self.reader,
            row,
        };

        module.name()
    }
}
