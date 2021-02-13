use super::*;
macros::table!(ImplMap);

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
