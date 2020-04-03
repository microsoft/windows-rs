use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct InterfaceImpl(pub Row);

impl InterfaceImpl {
    pub fn interface(self, reader: &Reader) -> TypeDefOrRef {
        reader.decode(self.0, 1)
    }

    pub fn attributes(self, reader: &Reader) -> impl Iterator<Item = Attribute> {
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
