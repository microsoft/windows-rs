use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct ModuleRef {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl ModuleRef {
    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 0)
    }
}

impl std::fmt::Debug for ModuleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleRef").field("row", &self.row).finish()
    }
}

impl PartialEq for ModuleRef {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for ModuleRef {}

impl Ord for ModuleRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for ModuleRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
