use super::*;
use crate::TypeReader;

// TODO: write macro to generate all of this boilerplate
#[derive(Copy, Clone)]
pub struct NestedClass {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl NestedClass {}

impl std::fmt::Debug for NestedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedClass")
            .field("row", &self.row)
            .finish()
    }
}

impl PartialEq for NestedClass {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for NestedClass {}

impl Ord for NestedClass {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for NestedClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
