use crate::file::TableIndex;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Row {
    pub index: u32,
    pub table_index: TableIndex,
    pub file_index: u16,
}

// TODO: still need strongly-types Row/Table/File/Column to avoid errors

impl Row {
    pub fn new(row: u32, table_index: TableIndex, file: u16) -> Self {
        Self {
            index: row,
            table_index,
            file_index: file,
        }
    }

    pub fn next(self) -> Self {
        Self::new(self.index + 1, self.table_index, self.file_index)
    }
}
