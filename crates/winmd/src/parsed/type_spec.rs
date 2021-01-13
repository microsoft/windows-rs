use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct TypeSpec {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl TypeSpec {
    pub fn sig(&self) -> Blob {
        self.reader.blob(self.row, 0)
    }
}

impl std::fmt::Debug for TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeSpec").field("row", &self.row).finish()
    }
}

impl PartialEq for TypeSpec {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for TypeSpec {}

impl Ord for TypeSpec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for TypeSpec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
