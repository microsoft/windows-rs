use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct MemberRef {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl MemberRef {
    pub fn parent(&self) -> MemberRefParent {
        self.reader.decode(self.row, 0)
    }

    pub fn name(&self) -> &str {
        self.reader.str(self.row, 1)
    }
}

impl std::fmt::Debug for MemberRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberRef").field("row", &self.row).finish()
    }
}

impl PartialEq for MemberRef {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for MemberRef {}

impl Ord for MemberRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for MemberRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
