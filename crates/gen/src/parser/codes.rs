use super::*;
use macros::type_code;

pub trait Decode {
    fn decode(reader: &'static TypeReader, code: u32, file: u16) -> Self;
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

#[type_code(1)]
pub enum MemberForwarded {
    Field,
    MethodDef,
}

#[type_code(2)]
pub enum ResolutionScope {
    Module,
    ModuleRef,
    AssemblyRef,
    TypeRef,
}

impl TypeDefOrRef {
    pub fn namespace(&self) -> &'static str {
        match self {
            Self::TypeDef(value) => value.namespace(),
            Self::TypeRef(value) => value.namespace(),
            _ => unexpected!(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::TypeDef(value) => value.name(),
            Self::TypeRef(value) => value.name(),
            _ => unexpected!(),
        }
    }

    pub fn full_name(&self) -> (&'static str, &'static str) {
        match self {
            Self::TypeDef(value) => value.full_name(),
            Self::TypeRef(value) => value.full_name(),
            _ => unexpected!(),
        }
    }

    pub fn resolve(&self) -> tables::TypeDef {
        match self {
            Self::TypeDef(value) => *value,
            Self::TypeRef(value) => value.resolve(),
            _ => unexpected!(),
        }
    }
}

impl MemberRefParent {
    pub fn full_name(&self) -> (&'static str, &'static str) {
        match self {
            Self::TypeDef(value) => value.full_name(),
            Self::TypeRef(value) => value.full_name(),
            _ => unexpected!(),
        }
    }
}
