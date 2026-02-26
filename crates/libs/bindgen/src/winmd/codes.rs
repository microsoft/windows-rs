use super::*;

pub use windows_metadata::reader::{AttributeType, Decode};

pub type HasAttribute = windows_metadata::reader::HasAttribute<'static>;
pub type MemberRefParent = windows_metadata::reader::MemberRefParent<'static>;
pub type TypeDefOrRef = windows_metadata::reader::TypeDefOrRef<'static>;

pub trait MemberRefParentExt {
    fn type_name(&self) -> TypeName;
}

impl MemberRefParentExt for MemberRefParent {
    fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(row) => row.type_name(),
            Self::TypeRef(row) => row.type_name(),
        }
    }
}

pub trait TypeDefOrRefExt {
    fn type_name(&self) -> TypeName;
    fn reader(&self) -> &'static Reader;
}

impl TypeDefOrRefExt for TypeDefOrRef {
    fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(row) => row.type_name(),
            Self::TypeRef(row) => row.type_name(),
            Self::TypeSpec(_) => panic!("TypeSpec has no type_name"),
        }
    }

    fn reader(&self) -> &'static Reader {
        match self {
            Self::TypeDef(row) => row.reader(),
            Self::TypeRef(row) => row.reader(),
            Self::TypeSpec(row) => row.reader(),
        }
    }
}

