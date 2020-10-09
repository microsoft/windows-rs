use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct InterfaceImpl(pub winmd::Row);

impl InterfaceImpl {
    pub fn interface(self, reader: &TypeReader) -> winmd::TypeDefOrRef {
        reader.decode(self.0, 1)
    }

    pub fn attributes(self, reader: &TypeReader) -> impl Iterator<Item = winmd::Attribute> {
        reader
            .equal_range(
                self.0.file_index,
                winmd::TableIndex::CustomAttribute,
                0,
                winmd::HasAttribute::InterfaceImpl(self).encode(),
            )
            .map(winmd::Attribute)
    }

    pub fn has_attribute(self, reader: &TypeReader, name: (&str, &str)) -> bool {
        self.attributes(reader)
            .any(|attribute| attribute.name(reader) == name)
    }

    pub fn is_default(self, reader: &TypeReader) -> bool {
        self.has_attribute(reader, ("Windows.Foundation.Metadata", "DefaultAttribute"))
    }
}
