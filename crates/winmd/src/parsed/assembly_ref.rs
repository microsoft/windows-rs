use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct AssemblyRef {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl AssemblyRef {}

impl std::fmt::Debug for AssemblyRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssemblyRef")
            .field("row", &self.row)
            .finish()
    }
}

impl PartialEq for AssemblyRef {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for AssemblyRef {}

impl Ord for AssemblyRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for AssemblyRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
