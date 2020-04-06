use super::Attribute;
use crate::codes::{HasAttribute, TypeDefOrRef};
use crate::file::TABLE_CUSTOMATTRIBUTE;
use crate::row::Row;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct InterfaceImpl(pub Row);

impl InterfaceImpl {
    pub fn interface(self, reader: &TypeReader) -> TypeDefOrRef {
        reader.decode(self.0, 1)
    }

    pub fn attributes(self, reader: &TypeReader) -> impl Iterator<Item = Attribute> {
        reader
            .equal_range(
                self.0.file,
                TABLE_CUSTOMATTRIBUTE,
                0,
                HasAttribute::InterfaceImpl(self).encode(),
            )
            .map(Attribute)
    }
}
