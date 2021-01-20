use super::*;
use crate::{TableIndex, TypeReader};

#[derive(Copy, Clone)]
pub struct MethodDef {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl MethodDef {
    pub fn flags(&self) -> MethodFlags {
        MethodFlags(self.reader.u32(self.row, 2))
    }

    pub fn params(&self) -> impl Iterator<Item = Param> + '_ {
        self.reader
            .list(self.row, TableIndex::Param, 5)
            .map(move |row| Param {
                reader: self.reader,
                row,
            })
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 3)
    }

    pub fn sig(&self) -> Blob {
        self.reader.blob(self.row, 4)
    }

    pub fn category(&self) -> MethodCategory {
        if self.flags().special() {
            let name = self.name();

            if name.starts_with("get") {
                MethodCategory::Get
            } else if name.starts_with("put") {
                MethodCategory::Set
            } else if name.starts_with("add") {
                MethodCategory::Add
            } else if name.starts_with("remove") {
                MethodCategory::Remove
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                MethodCategory::Normal
            }
        } else {
            MethodCategory::Normal
        }
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::MethodDef(*self).encode(),
            )
            .map(move |row| Attribute {
                reader: self.reader,
                row,
            })
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::ImplMap,
                1,
                MemberForwarded::MethodDef(*self).encode(),
            )
            .map(move |row| ImplMap {
                reader: self.reader,
                row,
            })
            .next()
    }
}

impl std::fmt::Debug for MethodDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MethodDef").field("row", &self.row).finish()
    }
}

impl PartialEq for MethodDef {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl Eq for MethodDef {}

impl Ord for MethodDef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for MethodDef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
