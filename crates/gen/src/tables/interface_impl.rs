use super::Attribute;
use crate::codes::{HasAttribute, TypeDefOrRef};
use crate::file::TableIndex;
use crate::row::Row;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct InterfaceImpl(pub Row);

impl InterfaceImpl {
    pub fn interface(self, reader: &TypeReader) -> TypeDefOrRef {
        reader.decode(self.0, 1)
    }

    pub fn attributes(self, reader: &TypeReader) -> impl Iterator<Item = Attribute> {
        reader
            .equal_range(
                self.0.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::InterfaceImpl(self).encode(),
            )
            .map(Attribute)
    }

    pub fn has_attribute(self, reader: &TypeReader, name: (&str, &str)) -> bool {
        self.attributes(reader)
            .any(|attribute| attribute.name(reader) == name)
    }
}
