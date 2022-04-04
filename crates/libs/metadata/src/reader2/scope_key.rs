#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ScopeKey {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}
