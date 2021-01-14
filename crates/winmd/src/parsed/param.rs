use super::*;
use crate::{TableIndex, TypeReader};

#[derive(Copy, Clone)]
pub struct Param {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl Param {
    pub fn flags(&self) -> ParamFlags {
        ParamFlags(self.reader.u32(self.row, 0))
    }

    pub fn sequence(&self) -> u32 {
        self.reader.u32(self.row, 1)
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 2)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::Param(*self).encode(),
            )
            .map(move |row| Attribute {
                reader: self.reader,
                row,
            })
    }

    pub fn has_attribute(&self, name: (&str, &str)) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }
}

impl std::fmt::Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Param")
            .field("name", &self.name().to_string())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl PartialEq for Param {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for Param {}

impl Ord for Param {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for Param {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
