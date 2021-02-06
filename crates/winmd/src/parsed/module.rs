use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct Module {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl Module {}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module").field("row", &self.row).finish()
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for Module {}

impl Ord for Module {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for Module {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
