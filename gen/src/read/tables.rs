use super::flags::{MethodAttributes, ParamAttributes, TypeAttributes};
use super::{Code, Reader, Row, RowData, RowIterator};
use crate::codes::*;
use crate::helpers::to_snake;
use crate::signatures::{field_sig, read_unsigned, ArgumentSig, GenericSig, MethodSig, TypeSig};

use winmd_macros::table;

table!(Constant);
table!(CustomAttribute);
table!(Field);
table!(GenericParam);
table!(InterfaceImpl);
table!(MemberRef);
table!(MethodDef);
table!(Param);
table!(TypeDef);
table!(TypeRef);
table!(TypeSpec);

pub enum TypeCategory {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
    Attribute,
    Contract,
}

pub enum ConstantValue {
    I32(i32),
    U32(u32),
}

impl CustomAttribute {
    pub(crate) fn parent(&self, r: &Reader) -> HasCustomAttribute {
        r.decode(&self.row, 0)
    }

    pub(crate) fn constructor(&self, reader: &Reader) -> CustomAttributeType {
        reader.decode(&self.row, 1)
    }

    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> (&'a str, &'a str) {
        match self.constructor(reader) {
            CustomAttributeType::MethodDef(value) => {
                let value = value.parent(reader);
                (value.namespace(reader), value.name(reader))
            }
            CustomAttributeType::MemberRef(value) => match value.parent(reader) {
                MemberRefParent::TypeDef(value) => (value.namespace(reader), value.name(reader)),
                MemberRefParent::TypeRef(value) => (value.namespace(reader), value.name(reader)),
                _ => panic!(),
            },
        }
    }

    pub(crate) fn arguments(&self, reader: &Reader) -> Vec<(String, ArgumentSig)> {
        match self.constructor(reader) {
            CustomAttributeType::MethodDef(value) => ArgumentSig::new(
                reader,
                self.row.file,
                reader.blob(&value.row, 4),
                reader.blob(&self.row, 2),
            ),
            CustomAttributeType::MemberRef(value) => ArgumentSig::new(
                reader,
                self.row.file,
                reader.blob(&value.row, 2),
                reader.blob(&self.row, 2),
            ),
        }
    }
}

impl Field {
    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(&self.row, 1)
    }

    pub(crate) fn signature(&self, reader: &Reader) -> TypeSig {
        field_sig(self, reader)
    }

    pub(crate) fn constants(&self, reader: &Reader) -> RowIterator<Constant> {
        reader.equal_range(self.row.file, 1, HasConstant::Field(*self).encode())
    }
}

impl GenericParam {
    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(&self.row, 3)
    }
}

impl InterfaceImpl {
    pub(crate) fn interface(&self, reader: &Reader) -> TypeDefOrRef {
        reader.decode(&self.row, 1)
    }

    pub(crate) fn attributes(&self, reader: &Reader) -> RowIterator<CustomAttribute> {
        reader.equal_range(
            self.row.file,
            0,
            HasCustomAttribute::InterfaceImpl(*self).encode(),
        )
    }

    pub(crate) fn has_attribute(&self, reader: &Reader, namespace: &str, name: &str) -> bool {
        self.attributes(reader)
            .any(|attribute| attribute.name(reader) == (namespace, name))
    }
}

impl MemberRef {
    pub(crate) fn parent(&self, reader: &Reader) -> MemberRefParent {
        reader.decode(&self.row, 0)
    }

    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(&self.row, 1)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum MethodCategory {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}

impl MethodDef {
    pub(crate) fn flags(&self, reader: &Reader) -> MethodAttributes {
        MethodAttributes(reader.u32(&self.row, 2))
    }

    pub(crate) fn parent(&self, reader: &Reader) -> TypeDef {
        reader.upper_bound(self.row.file, 6, self.row.row)
    }

    pub(crate) fn params(&self, reader: &Reader) -> RowIterator<Param> {
        reader.list(&self.row, 5)
    }

    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(&self.row, 3)
    }

    pub(crate) fn signature(&self, reader: &Reader) -> MethodSig {
        MethodSig::new(reader, self)
    }

    pub(crate) fn category(&self, reader: &Reader) -> MethodCategory {
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

    pub(crate) fn attributes(&self, reader: &Reader) -> RowIterator<CustomAttribute> {
        reader.equal_range(
            self.row.file,
            0,
            HasCustomAttribute::MethodDef(*self).encode(),
        )
    }

    pub(crate) fn find_attribute(
        &self,
        reader: &Reader,
        namespace: &str,
        name: &str,
    ) -> Option<CustomAttribute> {
        self.attributes(reader)
            .find(|attribute| attribute.name(reader) == (namespace, name))
    }
}

impl Param {
    pub(crate) fn flags(&self, reader: &Reader) -> ParamAttributes {
        ParamAttributes(reader.u32(&self.row, 0))
    }

    pub(crate) fn sequence(&self, reader: &Reader) -> u32 {
        reader.u32(&self.row, 1)
    }

    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> String {
        to_snake(reader.str(&self.row, 2))
    }
}

impl TypeDef {
    pub(crate) fn invalid() -> TypeDef {
        Self {
            row: RowData::invalid(),
        }
    }

    pub(crate) fn flags(&self, reader: &Reader) -> TypeAttributes {
        TypeAttributes(reader.u32(&self.row, 0))
    }

    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(&self.row, 1)
    }

    pub(crate) fn namespace<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(&self.row, 2)
    }

    pub(crate) fn extends(&self, reader: &Reader) -> TypeDefOrRef {
        reader.decode(&self.row, 3)
    }

    pub(crate) fn fields(&self, reader: &Reader) -> RowIterator<Field> {
        reader.list(&self.row, 4)
    }

    pub(crate) fn methods(&self, reader: &Reader) -> RowIterator<MethodDef> {
        reader.list(&self.row, 5)
    }

    pub(crate) fn generics(&self, reader: &Reader) -> RowIterator<GenericParam> {
        reader.equal_range(self.row.file, 2, TypeOrMethodDef::TypeDef(*self).encode())
    }

    pub(crate) fn interfaces(&self, reader: &Reader) -> RowIterator<InterfaceImpl> {
        reader.equal_range(self.row.file, 0, self.row.row + 1)
    }

    pub(crate) fn attributes(&self, reader: &Reader) -> RowIterator<CustomAttribute> {
        reader.equal_range(
            self.row.file,
            0,
            HasCustomAttribute::TypeDef(*self).encode(),
        )
    }

    pub(crate) fn has_attribute(&self, reader: &Reader, namespace: &str, name: &str) -> bool {
        self.attributes(reader)
            .any(|attribute| attribute.name(reader) == (namespace, name))
    }

    pub(crate) fn find_attribute(
        &self,
        reader: &Reader,
        namespace: &str,
        name: &str,
    ) -> Option<CustomAttribute> {
        self.attributes(reader)
            .find(|attribute| attribute.name(reader) == (namespace, name))
    }

    pub(crate) fn category(&self, reader: &Reader) -> TypeCategory {
        if self.flags(reader).interface() {
            TypeCategory::Interface
        } else {
            match self.extends(reader).name(reader) {
                "Enum" => TypeCategory::Enum,
                "MulticastDelegate" => TypeCategory::Delegate,
                "ValueType" => {
                    if self.has_attribute(
                        reader,
                        "Windows.Foundation.Metadata",
                        "ApiContractAttribute",
                    ) {
                        TypeCategory::Contract
                    } else {
                        TypeCategory::Struct
                    }
                }
                "Attribute" => TypeCategory::Attribute,
                _ => TypeCategory::Class,
            }
        }
    }

    // TODO: should return BaseIterator
    pub(crate) fn bases(&self, reader: &Reader) -> Vec<TypeDef> {
        let mut bases = Vec::new();
        let mut current = *self;

        loop {
            let extends = current.extends(reader);
            let namespace = extends.namespace(reader);
            let name = extends.name(reader);

            if namespace == "System" && name == "Object" {
                break;
            }

            current = extends.resolve(reader);
            bases.push(current);
        }

        bases
    }
}

impl TypeRef {
    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(&self.row, 1)
    }

    pub(crate) fn namespace<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(&self.row, 2)
    }

    // TODO: panic with "'full name' not found"
    pub(crate) fn resolve(&self, reader: &Reader) -> TypeDef {
        *reader
            .namespaces()
            .get(self.namespace(reader))
            .unwrap()
            .get(self.name(reader))
            .unwrap()
    }
}

impl TypeSpec {
    pub(crate) fn signature(&self, reader: &Reader) -> GenericSig {
        let mut bytes = reader.blob(&self.row, 0);
        read_unsigned(&mut bytes);
        GenericSig::new(self.row.file, &mut bytes)
    }
}
