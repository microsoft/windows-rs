use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct TypeDef(pub winmd::Row);

impl TypeDef {
    pub fn flags(self, reader: &TypeReader) -> winmd::TypeFlags {
        winmd::TypeFlags(reader.u32(self.0, 0))
    }

    pub fn name(self, reader: &TypeReader) -> (&str, &str) {
        (reader.str(self.0, 2), reader.str(self.0, 1))
    }

    pub fn extends(self, reader: &TypeReader) -> winmd::TypeDefOrRef {
        reader.decode(self.0, 3)
    }

    pub fn fields(self, reader: &TypeReader) -> impl Iterator<Item = winmd::Field> {
        reader
            .list(self.0, winmd::TableIndex::Field, 4)
            .map(winmd::Field)
    }

    pub fn methods(self, reader: &TypeReader) -> impl Iterator<Item = winmd::MethodDef> {
        reader
            .list(self.0, winmd::TableIndex::MethodDef, 5)
            .map(winmd::MethodDef)
    }

    pub fn generics(self, reader: &TypeReader) -> impl Iterator<Item = winmd::GenericParam> {
        reader
            .equal_range(
                self.0.file_index,
                winmd::TableIndex::GenericParam,
                2,
                winmd::TypeOrMethodDef::TypeDef(self).encode(),
            )
            .map(winmd::GenericParam)
    }

    pub fn interfaces(self, reader: &TypeReader) -> impl Iterator<Item = winmd::InterfaceImpl> {
        reader
            .equal_range(
                self.0.file_index,
                winmd::TableIndex::InterfaceImpl,
                0,
                self.0.index + 1,
            )
            .map(winmd::InterfaceImpl)
    }

    pub fn attributes(self, reader: &TypeReader) -> impl Iterator<Item = winmd::Attribute> {
        reader
            .equal_range(
                self.0.file_index,
                winmd::TableIndex::CustomAttribute,
                0,
                winmd::HasAttribute::TypeDef(self).encode(),
            )
            .map(winmd::Attribute)
    }

    pub fn has_attribute(self, reader: &TypeReader, name: (&str, &str)) -> bool {
        self.attributes(reader)
            .any(|attribute| attribute.name(reader) == name)
    }

    pub fn attribute(self, reader: &TypeReader, name: (&str, &str)) -> winmd::Attribute {
        self.attributes(reader)
            .find(|attribute| attribute.name(reader) == name)
            .unwrap()
    }

    pub fn ignore(self, reader: &TypeReader) -> bool {
        let flags = self.flags(reader);

        if !flags.windows_runtime() {
            true
        } else if flags.interface() {
            false
        } else {
            match self.extends(reader).name(reader) {
                ("System", "ValueType") => self.has_attribute(
                    reader,
                    ("Windows.Foundation.Metadata", "ApiContractAttribute"),
                ),
                ("System", "Attribute") => true,
                _ => false,
            }
        }
    }

    pub fn category(self, reader: &TypeReader) -> winmd::TypeCategory {
        debug_assert!(self.flags(reader).windows_runtime());

        if self.flags(reader).interface() {
            winmd::TypeCategory::Interface
        } else {
            match self.extends(reader).name(reader) {
                ("System", "Enum") => winmd::TypeCategory::Enum,
                ("System", "MulticastDelegate") => winmd::TypeCategory::Delegate,
                ("System", "ValueType") => winmd::TypeCategory::Struct,
                _ => winmd::TypeCategory::Class,
            }
        }
    }
}
