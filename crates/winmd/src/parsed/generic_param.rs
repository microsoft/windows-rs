use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct GenericParam {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl GenericParam {
    pub fn name<'a>(&self) -> &'a str {
        self.reader.str(self.row, 3)
    }
}

impl std::fmt::Debug for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericParam")
            .field("row", &self.row)
            .finish()
    }
}

impl PartialEq for GenericParam {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for GenericParam {}

impl Ord for GenericParam {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for GenericParam {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
