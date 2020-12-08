use super::*;
use crate::{TableIndex, TypeReader};

#[derive(Copy, Clone)]
pub struct InterfaceImpl {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl InterfaceImpl {
    pub fn interface(&self) -> TypeDefOrRef {
        self.reader.decode(self.row, 1)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::InterfaceImpl(*self).encode(),
            )
            .map(move |row| Attribute {
                reader: self.reader,
                row,
            })
    }

    pub fn has_attribute(&self, name: (&str, &str)) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_default(&self) -> bool {
        self.has_attribute(("Windows.Foundation.Metadata", "DefaultAttribute"))
    }
}

impl std::fmt::Debug for InterfaceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InterfaceImpl")
            .field("row", &self.row)
            .finish()
    }
}
