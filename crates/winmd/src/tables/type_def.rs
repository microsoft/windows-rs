use super::Attribute;
use crate::codes::{HasAttribute, TypeDefOrRef, TypeOrMethodDef};
use crate::file::TABLE_CUSTOMATTRIBUTE;
use crate::file::{TABLE_FIELD, TABLE_GENERICPARAM, TABLE_INTERFACEIMPL, TABLE_METHODDEF};
use crate::flags::{TypeCategory, TypeFlags};
use crate::row::Row;
use crate::tables::{Field, GenericParam, InterfaceImpl, MethodDef};
use crate::types::Type;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct TypeDef(pub Row);

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
        reader.list(self.0, TABLE_FIELD as u16, 4).map(Field)
    }

    pub fn methods(self, reader: &TypeReader) -> impl Iterator<Item = MethodDef> {
        reader
            .list(self.0, TABLE_METHODDEF as u16, 5)
            .map(MethodDef)
    }

    pub fn generics(self, reader: &TypeReader) -> impl Iterator<Item = GenericParam> {
        reader
            .equal_range(
                self.0.file,
                TABLE_GENERICPARAM,
                2,
                TypeOrMethodDef::TypeDef(self).encode(),
            )
            .map(GenericParam)
    }

    pub fn interfaces(self, reader: &TypeReader) -> impl Iterator<Item = InterfaceImpl> {
        reader
            .equal_range(self.0.file, TABLE_INTERFACEIMPL, 0, self.0.row + 1)
            .map(InterfaceImpl)
    }

    pub fn attributes(self, reader: &TypeReader) -> impl Iterator<Item = Attribute> {
        reader
            .equal_range(
                self.0.file,
                TABLE_CUSTOMATTRIBUTE,
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

    pub fn category(self, reader: &TypeReader) -> TypeCategory {
        debug_assert!(self.flags(reader).windows_runtime());

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

    pub fn into_type(self, reader: &TypeReader) -> Type {
        Type::from_type_def(reader, self)
    }
}
