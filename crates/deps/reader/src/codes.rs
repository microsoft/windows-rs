use super::*;

pub trait Decode {
    fn decode(file: &'static File, code: u32) -> Self;
}

#[derive(Clone)]
pub enum TypeDefOrRef {
    TypeDef(TypeDef),
    TypeRef(TypeRef),
    TypeSpec(TypeSpec),
}

impl Decode for TypeDefOrRef {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match code.0 {
            0 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
            1 => Self::TypeRef(TypeRef(Row::new(code.1, TableIndex::TypeRef, file))),
            2 => Self::TypeSpec(TypeSpec(Row::new(code.1, TableIndex::TypeSpec, file))),
            _ => unimplemented!(),
        }
    }
}

impl TypeDefOrRef {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(value) => ((value.row.row + 1) << 2),
            Self::TypeRef(value) => ((value.0.row + 1) << 2) | 1,
            Self::TypeSpec(value) => ((value.0.row + 1) << 2) | 2,
        }
    }
}

#[derive(Clone)]
pub enum TypeOrMethodDef {
    TypeDef(TypeDef),
    MethodDef(MethodDef),
}

impl Decode for TypeOrMethodDef {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 1) - 1), (code >> 1) - 1);
        match code.0 {
            0 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
            1 => Self::MethodDef(MethodDef(Row::new(code.1, TableIndex::MethodDef, file))),
            _ => unimplemented!(),
        }
    }
}

impl TypeOrMethodDef {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(value) => ((value.row.row + 1) << 1),
            Self::MethodDef(value) => ((value.0.row + 1) << 1) | 1,
        }
    }
}

#[derive(Clone)]
pub enum HasAttribute {
    MethodDef(MethodDef),
    Field(Field),
    TypeRef(TypeRef),
    TypeDef(TypeDef),
    Param(Param),
    InterfaceImpl(InterfaceImpl),
    MemberRef(MemberRef),
    TypeSpec(TypeSpec),
    GenericParam(GenericParam),
}

impl Decode for HasAttribute {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 5) - 1), (code >> 5) - 1);
        match code.0 {
            0 => Self::MethodDef(MethodDef(Row::new(code.1, TableIndex::MethodDef, file))),
            1 => Self::Field(Field(Row::new(code.1, TableIndex::Field, file))),
            2 => Self::TypeRef(TypeRef(Row::new(code.1, TableIndex::TypeRef, file))),
            3 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
            4 => Self::Param(Param(Row::new(code.1, TableIndex::Param, file))),
            5 => Self::InterfaceImpl(InterfaceImpl(Row::new(
                code.1,
                TableIndex::InterfaceImpl,
                file,
            ))),
            6 => Self::MemberRef(MemberRef(Row::new(code.1, TableIndex::MemberRef, file))),
            13 => Self::TypeSpec(TypeSpec(Row::new(code.1, TableIndex::TypeSpec, file))),
            19 => Self::GenericParam(GenericParam(Row::new(
                code.1,
                TableIndex::GenericParam,
                file,
            ))),
            _ => unimplemented!(),
        }
    }
}

impl HasAttribute {
    pub fn encode(&self) -> u32 {
        match self {
            Self::MethodDef(value) => ((value.0.row + 1) << 5),
            Self::Field(value) => ((value.0.row + 1) << 5) | 1,
            Self::TypeRef(value) => ((value.0.row + 1) << 5) | 2,
            Self::TypeDef(value) => ((value.row.row + 1) << 5) | 3,
            Self::Param(value) => ((value.0.row + 1) << 5) | 4,
            Self::InterfaceImpl(value) => ((value.0.row + 1) << 5) | 5,
            Self::MemberRef(value) => ((value.0.row + 1) << 5) | 6,
            Self::TypeSpec(value) => ((value.0.row + 1) << 5) | 13,
            Self::GenericParam(value) => ((value.0.row + 1) << 5) | 19,
        }
    }
}

#[derive(Clone)]
pub enum MemberRefParent {
    TypeDef(TypeDef),
    TypeRef(TypeRef),
    MethodDef(MethodDef),
    TypeSpec(TypeSpec),
}

impl Decode for MemberRefParent {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match code.0 {
            0 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
            1 => Self::TypeRef(TypeRef(Row::new(code.1, TableIndex::TypeRef, file))),
            3 => Self::MethodDef(MethodDef(Row::new(code.1, TableIndex::MethodDef, file))),
            4 => Self::TypeSpec(TypeSpec(Row::new(code.1, TableIndex::TypeSpec, file))),
            _ => unimplemented!(),
        }
    }
}

impl MemberRefParent {
    pub fn encode(&self) -> u32 {
        match self {
            Self::TypeDef(value) => ((value.row.row + 1) << 3),
            Self::TypeRef(value) => ((value.0.row + 1) << 3) | 1,
            Self::MethodDef(value) => ((value.0.row + 1) << 3) | 3,
            Self::TypeSpec(value) => ((value.0.row + 1) << 3) | 4,
        }
    }
}

#[derive(Clone)]
pub enum HasConstant {
    Field(Field),
    Param(Param),
}

impl Decode for HasConstant {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 2) - 1), (code >> 2) - 1);
        match code.0 {
            0 => Self::Field(Field(Row::new(code.1, TableIndex::Field, file))),
            1 => Self::Param(Param(Row::new(code.1, TableIndex::Param, file))),
            _ => unimplemented!(),
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

#[derive(Clone)]
pub enum AttributeType {
    MethodDef(MethodDef),
    MemberRef(MemberRef),
}

impl Decode for AttributeType {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match code.0 {
            2 => Self::MethodDef(MethodDef(Row::new(code.1, TableIndex::MethodDef, file))),
            3 => Self::MemberRef(MemberRef(Row::new(code.1, TableIndex::MemberRef, file))),
            _ => unimplemented!(),
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

#[derive(Clone)]
pub enum MemberForwarded {
    Field(Field),
    MethodDef(MethodDef),
}

impl Decode for MemberForwarded {
    fn decode(file: &'static File, code: u32) -> Self {
        let code = (code & ((1 << 1) - 1), (code >> 1) - 1);
        match code.0 {
            0 => Self::Field(Field(Row::new(code.1, TableIndex::Field, file))),
            1 => Self::MethodDef(MethodDef(Row::new(code.1, TableIndex::MethodDef, file))),
            _ => unimplemented!(),
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

impl TypeDefOrRef {
    pub fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(value) => value.type_name(),
            Self::TypeRef(value) => value.type_name(),
            _ => unimplemented!(),
        }
    }

    pub fn resolve(&self, enclosing: Option<&TypeDef>) -> TypeDef {
        match self {
            Self::TypeDef(value) => value.clone(),
            Self::TypeRef(value) => TypeReader::get().expect_type_ref(enclosing, value),
            _ => unimplemented!(),
        }
    }
}

impl MemberRefParent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::TypeDef(value) => value.name(),
            Self::TypeRef(value) => value.name(),
            _ => unimplemented!(),
        }
    }

    pub fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(value) => value.type_name(),
            Self::TypeRef(value) => value.type_name(),
            _ => unimplemented!(),
        }
    }
}
