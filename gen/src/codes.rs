use crate::*;
use winmd_macros::*;

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
    pub fn name<'a>(&self, r: &'a Reader) -> &'a str {
        match self {
            Self::TypeDef(value) => value.name(r),
            Self::TypeRef(value) => value.name(r),
            Self::TypeSpec(_) => panic!("TypeDefOrRef"),
        }
    }

    pub fn namespace<'a>(&self, r: &'a Reader) -> &'a str {
        match self {
            Self::TypeDef(value) => value.namespace(r),
            Self::TypeRef(value) => value.namespace(r),
            Self::TypeSpec(value) => value.signature(r).definition().namespace(r),
        }
    }

    pub fn resolve(&self, r: &Reader) -> TypeDef {
        match self {
            Self::TypeDef(value) => *value,
            Self::TypeRef(value) => value.resolve(r),
            Self::TypeSpec(_) => panic!("TypeDefOrRef"),
        }
    }
}
