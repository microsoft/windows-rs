#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ScopeKey {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}

impl ScopeKey {
    pub fn new(row: usize, table: usize, file: usize) -> Self {
        Self { row: row as _, table: table as _, file: file as _ }
    }
}
