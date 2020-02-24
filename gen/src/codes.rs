use winmd_macros::type_code;

use crate::read::*;

#[type_code(2)]
pub enum TypeDefOrRef {
    TypeDef,
    TypeRef,
    TypeSpec,
}

#[type_code(1)]
pub enum TypeOrMethodDef {
    TypeDef,
    MethodDef,
}

#[type_code(5)]
pub enum HasCustomAttribute {
    MethodDef,
    Field,
    TypeRef,
    TypeDef,
    Param,
    InterfaceImpl,
    MemberRef,
    TypeSpec = 13,
    GenericParam = 19,
}

#[type_code(3)]
pub enum MemberRefParent {
    TypeDef,
    TypeRef,
    MethodDef = 3,
    TypeSpec,
}

#[type_code(2)]
pub enum HasConstant {
    Field,
    Param,
}

#[type_code(3)]
pub enum CustomAttributeType {
    MethodDef = 2,
    MemberRef,
}

impl TypeDefOrRef {
    pub(crate) fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        match self {
            Self::TypeDef(value) => value.name(reader),
            Self::TypeRef(value) => value.name(reader),
            Self::TypeSpec(_) => panic!("TypeDefOrRef"),
        }
    }

    pub(crate) fn namespace<'a>(&self, reader: &'a Reader) -> &'a str {
        match self {
            Self::TypeDef(value) => value.namespace(reader),
            Self::TypeRef(value) => value.namespace(reader),
            Self::TypeSpec(value) => value.signature(reader).definition().namespace(reader),
        }
    }

    pub(crate) fn resolve(&self, reader: &Reader) -> TypeDef {
        match self {
            Self::TypeDef(value) => *value,
            Self::TypeRef(value) => value.resolve(reader),
            Self::TypeSpec(_) => panic!("TypeDefOrRef"),
        }
    }
}
