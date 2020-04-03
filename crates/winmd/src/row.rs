#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Row {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}

// TODO: still need strongly-types Row/Table/File/Column to avoid errors

impl Row {
    pub fn new(row: u32, table: u16, file: u16) -> Self {
        Self { row, table, file }
    }

    pub fn next(self) -> Self {
        Self::new(self.row + 1, self.table, self.file)
    }
}
