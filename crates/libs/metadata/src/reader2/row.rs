#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Row {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}

impl Row {
    pub fn new(row: usize, table: usize, file: usize) -> Self {
        Self { row: row as _, table: table as _, file: file as _ }
    }
    pub fn next(&self) -> Self {
        Self { row: self.row + 1, table: self.table, file: self.file }
    }
}
