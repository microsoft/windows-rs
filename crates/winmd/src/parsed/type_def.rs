use super::*;
use crate::{TableIndex, TypeReader};

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct TypeDef(pub Row);

pub struct TypeDef2{pub reader: &'static TypeReader, pub row: Row}

impl TypeDef {
    pub fn flags(self, reader: &TypeReader) -> TypeFlags {
        TypeFlags(reader.u32(self.0, 0))
    }

    pub fn name(self, reader: &TypeReader) -> (&str, &str) {
        (reader.str(self.0, 2), reader.str(self.0, 1))
    }

    pub fn extends(self, reader: &TypeReader) -> TypeDefOrRef {
        reader.decode(self.0, 3)
    }

    pub fn fields(self, reader: &TypeReader) -> impl Iterator<Item = Field> {
        reader.list(self.0, TableIndex::Field, 4).map(Field)
    }

    pub fn methods(self, reader: &TypeReader) -> impl Iterator<Item = MethodDef> {
        reader.list(self.0, TableIndex::MethodDef, 5).map(MethodDef)
    }

    pub fn generics(self, reader: &TypeReader) -> impl Iterator<Item = GenericParam> {
        reader
            .equal_range(
                self.0.file_index,
                TableIndex::GenericParam,
                2,
                TypeOrMethodDef::TypeDef(self).encode(),
            )
            .map(GenericParam)
    }

    pub fn interfaces(self, reader: &TypeReader) -> impl Iterator<Item = InterfaceImpl> {
        reader
            .equal_range(
                self.0.file_index,
                TableIndex::InterfaceImpl,
                0,
                self.0.index + 1,
            )
            .map(InterfaceImpl)
    }

    pub fn attributes(self, reader: &TypeReader) -> impl Iterator<Item = Attribute> {
        reader
            .equal_range(
                self.0.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::TypeDef(self).encode(),
            )
            .map(Attribute)
    }

    pub fn has_attribute(self, reader: &TypeReader, name: (&str, &str)) -> bool {
        self.attributes(reader)
            .any(|attribute| attribute.name(reader) == name)
    }

    pub fn attribute(self, reader: &TypeReader, name: (&str, &str)) -> Attribute {
        self.attributes(reader)
            .find(|attribute| attribute.name(reader) == name)
            .unwrap()
    }

    pub fn is_winrt(self, reader: &TypeReader) -> bool {
        let flags = self.flags(reader);

        if !flags.windows_runtime() {
            false
        } else if flags.interface() {
            true
        } else {
            match self.extends(reader).name(reader) {
                ("System", "ValueType") => !self.has_attribute(
                    reader,
                    ("Windows.Foundation.Metadata", "ApiContractAttribute"),
                ),
                ("System", "Attribute") => false,
                _ => true,
            }
        }
    }

    pub fn category(self, reader: &TypeReader) -> TypeCategory {
        if self.flags(reader).interface() {
            TypeCategory::Interface
        } else {
            match self.extends(reader).name(reader) {
                ("System", "Enum") => TypeCategory::Enum,
                ("System", "MulticastDelegate") => TypeCategory::Delegate,
                ("System", "ValueType") => TypeCategory::Struct,
                _ => TypeCategory::Class,
            }
        }
    }

    pub fn underlying_type(self, reader: &TypeReader) -> ElementType {
        for field in self.fields(reader) {
            for constant in field.constants(reader) {
                return constant.value_type(reader);
            }
        }

        panic!("TypeDef::underlying_type");
    }
}
