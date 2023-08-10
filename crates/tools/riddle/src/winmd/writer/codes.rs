#![allow(dead_code, clippy::enum_variant_names)]

/// A `ResolutionScope` is an index into a certain table indicating the scope in which a TypeRef can be resolved.
#[derive(Clone)]
pub enum ResolutionScope {
    Module(u32),
    ModuleRef(u32),
    AssemblyRef(u32),
    TypeRef(u32),
}

impl ResolutionScope {
    pub fn encode(&self) -> u32 {
        match self {
            Self::Module(row) => (row + 1) << 2,
            Self::ModuleRef(row) => ((row + 1) << 2) + 1,
            Self::AssemblyRef(row) => ((row + 1) << 2) + 2,
            Self::TypeRef(row) => ((row + 1) << 2) + 3,
        }
    }
}

/// A `TypeDefOrRef` is an index into a certain table used to locate a type definition.
#[derive(Clone)]
pub enum TypeDefOrRef {
    TypeDef(u32),
    TypeRef(u32),
    TypeSpec(u32),
}

impl TypeDefOrRef {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(row) => (row + 1) << 2,
            Self::TypeRef(row) => ((row + 1) << 2) + 1,
            Self::TypeSpec(row) => ((row + 1) << 2) + 2,
        }
    }
}

/// A `HasConstant` is an index into a certain table used to identify the parent of a row in the `Constant` table.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HasConstant {
    Field(u32),
    Param(u32),
    Property(u32),
}

impl HasConstant {
    pub fn encode(&self) -> u32 {
        match self {
            Self::Field(row) => (row + 1) << 2,
            Self::Param(row) => ((row + 1) << 2) + 1,
            Self::Property(row) => ((row + 1) << 2) + 2,
        }
    }
}

/// A `TypeOrMethodDef` is an index into a certain table used to locate the owner of a generic parameter.
#[derive(Clone)]
pub enum TypeOrMethodDef {
    TypeDef(u32),
}

impl TypeOrMethodDef {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(row) => (row + 1) << 1,
        }
    }
}
