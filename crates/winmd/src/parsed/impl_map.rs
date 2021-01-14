use super::*;
use crate::{TableIndex, TypeReader};

#[derive(Copy, Clone)]
pub struct ImplMap {
    pub reader: &'static TypeReader,
    pub row: Row,
}

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

impl std::fmt::Debug for ImplMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImplMap").field("row", &self.row).finish()
    }
}

impl PartialEq for ImplMap {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for ImplMap {}

impl Ord for ImplMap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for ImplMap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
