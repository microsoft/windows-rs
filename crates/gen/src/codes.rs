use crate::file::*;
use crate::row::Row;
use crate::tables::*;
use crate::TypeReader;

use winrt_gen_macros::type_code;

pub trait Decode {
    fn decode(code: u32, file: u16) -> Self;
}

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
pub enum HasAttribute {
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
pub enum AttributeType {
    MethodDef = 2,
    MemberRef,
}

impl TypeDefOrRef {
    pub fn name<'a>(&self, reader: &'a TypeReader) -> (&'a str, &'a str) {
        match self {
            TypeDefOrRef::TypeDef(value) => value.name(reader),
            TypeDefOrRef::TypeRef(value) => value.name(reader),
            TypeDefOrRef::TypeSpec(_) => panic!("Expected a TypeDef or TypeRef"),
        }
    }

    pub fn resolve(&self, reader: &TypeReader) -> TypeDef {
        match self {
            Self::TypeDef(value) => *value,
            Self::TypeRef(value) => value.resolve(reader),
            Self::TypeSpec(_) => panic!("Expected a TypeDef or TypeRef"),
        }
    }
}
