use super::*;

pub trait Decode {
    fn decode(file: &'static File, code: u32) -> Self;
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TypeDefOrRef {
    TypeDef(tables::TypeDef),
    TypeRef(tables::TypeRef),
    TypeSpec(tables::TypeSpec),
}

impl Decode for TypeDefOrRef {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match code.0 {
            0 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
            1 => Self::TypeRef(tables::TypeRef(Row::new(code.1, TableIndex::TypeRef, file))),
            2 => Self::TypeSpec(tables::TypeSpec(Row::new(
                code.1,
                TableIndex::TypeSpec,
                file,
            ))),
            _ => panic!("type_code"),
        }
    }
}

impl TypeDefOrRef {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(value) => ((value.row().row + 1) << 2),
            Self::TypeRef(value) => ((value.0.row + 1) << 2) | 1,
            Self::TypeSpec(value) => ((value.0.row + 1) << 2) | 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TypeOrMethodDef {
    TypeDef(tables::TypeDef),
    MethodDef(tables::MethodDef),
}

impl Decode for TypeOrMethodDef {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 1) - 1), (code >> 1) - 1);
        match code.0 {
            0 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
            1 => Self::MethodDef(tables::MethodDef(Row::new(
                code.1,
                TableIndex::MethodDef,
                file,
            ))),
            _ => panic!("type_code"),
        }
    }
}

impl TypeOrMethodDef {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(value) => ((value.row().row + 1) << 1),
            Self::MethodDef(value) => ((value.0.row + 1) << 1) | 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HasAttribute {
    MethodDef(tables::MethodDef),
    Field(tables::Field),
    TypeRef(tables::TypeRef),
    TypeDef(tables::TypeDef),
    Param(tables::Param),
    InterfaceImpl(tables::InterfaceImpl),
    MemberRef(tables::MemberRef),
    TypeSpec(tables::TypeSpec),
    GenericParam(tables::GenericParam),
}

impl Decode for HasAttribute {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 5) - 1), (code >> 5) - 1);
        match code.0 {
            0 => Self::MethodDef(tables::MethodDef(Row::new(
                code.1,
                TableIndex::MethodDef,
                file,
            ))),
            1 => Self::Field(tables::Field(Row::new(code.1, TableIndex::Field, file))),
            2 => Self::TypeRef(tables::TypeRef(Row::new(code.1, TableIndex::TypeRef, file))),
            3 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
            4 => Self::Param(tables::Param(Row::new(code.1, TableIndex::Param, file))),
            5 => Self::InterfaceImpl(tables::InterfaceImpl(Row::new(
                code.1,
                TableIndex::InterfaceImpl,
                file,
            ))),
            6 => Self::MemberRef(tables::MemberRef(Row::new(
                code.1,
                TableIndex::MemberRef,
                file,
            ))),
            13 => Self::TypeSpec(tables::TypeSpec(Row::new(
                code.1,
                TableIndex::TypeSpec,
                file,
            ))),
            19 => Self::GenericParam(tables::GenericParam(Row::new(
                code.1,
                TableIndex::GenericParam,
                file,
            ))),
            _ => panic!("type_code"),
        }
    }
}

impl HasAttribute {
    pub fn encode(&self) -> u32 {
        match self {
            Self::MethodDef(value) => ((value.0.row + 1) << 5),
            Self::Field(value) => ((value.0.row + 1) << 5) | 1,
            Self::TypeRef(value) => ((value.0.row + 1) << 5) | 2,
            Self::TypeDef(value) => ((value.row().row + 1) << 5) | 3,
            Self::Param(value) => ((value.0.row + 1) << 5) | 4,
            Self::InterfaceImpl(value) => ((value.0.row + 1) << 5) | 5,
            Self::MemberRef(value) => ((value.0.row + 1) << 5) | 6,
            Self::TypeSpec(value) => ((value.0.row + 1) << 5) | 13,
            Self::GenericParam(value) => ((value.0.row + 1) << 5) | 19,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MemberRefParent {
    TypeDef(tables::TypeDef),
    TypeRef(tables::TypeRef),
    MethodDef(tables::MethodDef),
    TypeSpec(tables::TypeSpec),
}

impl Decode for MemberRefParent {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match code.0 {
            0 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
            1 => Self::TypeRef(tables::TypeRef(Row::new(code.1, TableIndex::TypeRef, file))),
            3 => Self::MethodDef(tables::MethodDef(Row::new(
                code.1,
                TableIndex::MethodDef,
                file,
            ))),
            4 => Self::TypeSpec(tables::TypeSpec(Row::new(
                code.1,
                TableIndex::TypeSpec,
                file,
            ))),
            _ => panic!("type_code"),
        }
    }
}

impl MemberRefParent {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(value) => ((value.row().row + 1) << 3),
            Self::TypeRef(value) => ((value.0.row + 1) << 3) | 1,
            Self::MethodDef(value) => ((value.0.row + 1) << 3) | 3,
            Self::TypeSpec(value) => ((value.0.row + 1) << 3) | 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HasConstant {
    Field(tables::Field),
    Param(tables::Param),
}

impl Decode for HasConstant {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match code.0 {
            0 => Self::Field(tables::Field(Row::new(code.1, TableIndex::Field, file))),
            1 => Self::Param(tables::Param(Row::new(code.1, TableIndex::Param, file))),
            _ => panic!("type_code"),
        }
    }
}

impl HasConstant {
    pub fn encode(&self) -> u32 {
        match self {
            Self::Field(value) => ((value.0.row + 1) << 2),
            Self::Param(value) => ((value.0.row + 1) << 2) | 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AttributeType {
    MethodDef(tables::MethodDef),
    MemberRef(tables::MemberRef),
}

impl Decode for AttributeType {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match code.0 {
            2 => Self::MethodDef(tables::MethodDef(Row::new(
                code.1,
                TableIndex::MethodDef,
                file,
            ))),
            3 => Self::MemberRef(tables::MemberRef(Row::new(
                code.1,
                TableIndex::MemberRef,
                file,
            ))),
            _ => panic!("type_code"),
        }
    }
}

impl AttributeType {
    pub fn encode(&self) -> u32 {
        match self {
            Self::MethodDef(value) => ((value.0.row + 1) << 3) | 2,
            Self::MemberRef(value) => ((value.0.row + 1) << 3) | 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MemberForwarded {
    Field(tables::Field),
    MethodDef(tables::MethodDef),
}

impl Decode for MemberForwarded {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 1) - 1), (code >> 1) - 1);
        match code.0 {
            0 => Self::Field(tables::Field(Row::new(code.1, TableIndex::Field, file))),
            1 => Self::MethodDef(tables::MethodDef(Row::new(
                code.1,
                TableIndex::MethodDef,
                file,
            ))),
            _ => panic!("type_code"),
        }
    }
}

impl MemberForwarded {
    pub fn encode(&self) -> u32 {
        match self {
            Self::Field(value) => ((value.0.row + 1) << 1),
            Self::MethodDef(value) => ((value.0.row + 1) << 1) | 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ResolutionScope {
    Module(tables::Module),
    ModuleRef(tables::ModuleRef),
    AssemblyRef(tables::AssemblyRef),
    TypeRef(tables::TypeRef),
}

impl Decode for ResolutionScope {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match code.0 {
            0 => Self::Module(tables::Module(Row::new(code.1, TableIndex::Module, file))),
            1 => Self::ModuleRef(tables::ModuleRef(Row::new(
                code.1,
                TableIndex::ModuleRef,
                file,
            ))),
            2 => Self::AssemblyRef(tables::AssemblyRef(Row::new(
                code.1,
                TableIndex::AssemblyRef,
                file,
            ))),
            3 => Self::TypeRef(tables::TypeRef(Row::new(code.1, TableIndex::TypeRef, file))),
            _ => panic!("type_code"),
        }
    }
}

impl ResolutionScope {
    pub fn encode(&self) -> u32 {
        match self {
            Self::Module(value) => ((value.0.row + 1) << 2),
            Self::ModuleRef(value) => ((value.0.row + 1) << 2) | 1,
            Self::AssemblyRef(value) => ((value.0.row + 1) << 2) | 2,
            Self::TypeRef(value) => ((value.0.row + 1) << 2) | 3,
        }
    }
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
            Self::TypeDef(value) => value.clone(),
            Self::TypeRef(value) => value.resolve(),
            _ => unexpected!(),
        }
    }
}

impl MemberRefParent {
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
}
