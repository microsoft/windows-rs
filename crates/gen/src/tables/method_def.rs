use super::{Attribute, Param, TypeDef};
use crate::blob::Blob;
use crate::codes::HasAttribute;
use crate::file::TableIndex;
use crate::flags::{MethodCategory, MethodFlags};
use crate::row::Row;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct MethodDef(pub Row);

impl MethodDef {
    pub fn flags(self, reader: &TypeReader) -> MethodFlags {
        MethodFlags(reader.u32(self.0, 2))
    }

    pub fn parent(self, reader: &TypeReader) -> TypeDef {
        TypeDef(reader.upper_bound(self.0.file_index, TableIndex::TypeDef, 6, self.0.index))
    }

    pub fn params(self, reader: &TypeReader) -> impl Iterator<Item = Param> {
        reader.list(self.0, TableIndex::Param, 5).map(Param)
    }

    pub fn name(self, reader: &TypeReader) -> &str {
        reader.str(self.0, 3)
    }

    pub fn sig(self, reader: &TypeReader) -> Blob {
        reader.blob(self.0, 4)
    }

    pub fn category(self, reader: &TypeReader) -> MethodCategory {
        if self.flags(reader).special() {
            let name = self.name(reader);

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

    pub fn attributes(self, reader: &TypeReader) -> impl Iterator<Item = Attribute> {
        reader
            .equal_range(
                self.0.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::MethodDef(self).encode(),
            )
            .map(Attribute)
    }

    pub fn find_attribute(self, reader: &TypeReader, name: (&str, &str)) -> Option<Attribute> {
        self.attributes(reader)
            .find(|attribute| attribute.name(reader) == name)
    }
}
