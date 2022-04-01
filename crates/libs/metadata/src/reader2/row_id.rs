#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct RowId {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}
