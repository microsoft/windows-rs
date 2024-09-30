use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Class(Class),
    Delegate(Delegate),
    Enum(Enum),
    Interface(Interface),
    Struct(Struct),

    CppConst(CppConst),
    CppDelegate(CppDelegate),
    CppEnum(CppEnum),
    CppFn(CppFn),
    CppInterface(CppInterface),
    CppStruct(CppStruct),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Interface {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Class {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Enum {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Struct {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Delegate {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq)]
pub struct CppInterface {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq)]
pub struct CppEnum {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq)]
pub struct CppStruct {
    pub def: TypeDef,
    pub nested: HashMap<&'static str, Item>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct CppDelegate {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq)]
pub struct CppConst {
    pub def: TypeDef,
    pub field: Field,
}
#[derive(Debug, PartialEq, Eq)]
pub struct CppFn {
    pub def: TypeDef,
    pub method: MethodDef,
}

impl Item {
    pub fn namespace(&self) -> &'static str {
        match self {
            Item::Class(item) => item.def.namespace(),
            Item::Delegate(item) => item.def.namespace(),
            Item::Enum(item) => item.def.namespace(),
            Item::Interface(item) => item.def.namespace(),
            Item::Struct(item) => item.def.namespace(),
            Item::CppDelegate(item) => item.def.namespace(),
            Item::CppEnum(item) => item.def.namespace(),
            Item::CppInterface(item) => item.def.namespace(),
            Item::CppStruct(item) => item.def.namespace(),
            Item::CppConst(item) => item.def.namespace(),
            Item::CppFn(item) => item.def.namespace(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Item::Class(item) => item.def.name(),
            Item::Delegate(item) => item.def.name(),
            Item::Enum(item) => item.def.name(),
            Item::Interface(item) => item.def.name(),
            Item::Struct(item) => item.def.name(),
            Item::CppDelegate(item) => item.def.name(),
            Item::CppEnum(item) => item.def.name(),
            Item::CppInterface(item) => item.def.name(),
            Item::CppStruct(item) => item.def.name(),
            Item::CppConst(item) => item.field.name(),
            Item::CppFn(item) => item.method.name(),
        }
    }
}
