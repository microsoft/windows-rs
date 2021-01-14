use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct TypeRef {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl TypeRef {
    pub fn name(&self) -> (&'static str, &'static str) {
        (self.reader.str(self.row, 2), self.reader.str(self.row, 1))
    }

    pub fn resolve(&self) -> TypeDef {
        self.reader.expect_type_def(self.name())
    }
}

impl std::fmt::Debug for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeRef").field("row", &self.row).finish()
    }
}

impl PartialEq for TypeRef {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for TypeRef {}

impl Ord for TypeRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for TypeRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
