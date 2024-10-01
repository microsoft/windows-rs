use super::*;

// TODO: implement Ord for CppFn manually so that it sorts by library and then name
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
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
    // TODO: have psuedo items for the core types like PWSTR so that those can be written out for standalone code gen?
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Interface {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Class {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Enum {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Struct {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Delegate {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppInterface {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppEnum {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppStruct {
    pub def: TypeDef,
    pub name: String,
    pub nested: BTreeMap<&'static str, Item>,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppDelegate {
    pub def: TypeDef,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppConst {
    pub def: TypeDef,
    pub field: Field,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppFn {
    pub def: TypeDef,
    pub method: MethodDef,
}

impl Item {
    pub fn namespace(&self) -> &'static str {
        match self {
            Self::Class(item) => item.def.namespace(),
            Self::Delegate(item) => item.def.namespace(),
            Self::Enum(item) => item.def.namespace(),
            Self::Interface(item) => item.def.namespace(),
            Self::Struct(item) => item.def.namespace(),
            Self::CppDelegate(item) => item.def.namespace(),
            Self::CppEnum(item) => item.def.namespace(),
            Self::CppInterface(item) => item.def.namespace(),
            Self::CppStruct(item) => item.def.namespace(),
            Self::CppConst(item) => item.def.namespace(),
            Self::CppFn(item) => item.def.namespace(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Class(item) => item.def.name(),
            Self::Delegate(item) => item.def.name(),
            Self::Enum(item) => item.def.name(),
            Self::Interface(item) => item.def.name(),
            Self::Struct(item) => item.def.name(),
            Self::CppDelegate(item) => item.def.name(),
            Self::CppEnum(item) => item.def.name(),
            Self::CppInterface(item) => item.def.name(),
            Self::CppStruct(item) => item.def.name(),
            Self::CppConst(item) => item.field.name(),
            Self::CppFn(item) => item.method.name(),
        }
    }

    pub fn is_nullable(&self) -> bool {
        matches!(
            self,
            Self::Class(_) | Self::Interface(_) | Self::Delegate(_) | Self::CppInterface(_)
        )
    }

    pub fn is_copyable(&self) -> bool {
        matches!(self, Self::Enum(_) | Self::CppEnum(_))
    }
}

// TODO: put signatures in their own rs file?
impl Item {
    pub fn signature(&self) -> String {
        match self {
            // Self::Class(item) => item.signature(),
            // Self::Delegate(item) => item.signature(),
            // Self::Enum(item) => item.signature(),
            // Self::Interface(item) => item.signature(),
            Self::Struct(item) => item.signature(),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }
}

impl Struct {
    pub fn signature(&self) -> String {
        let mut signature = format!("struct({}.{}", self.def.namespace(), self.def.name());
        for field in self.def.fields() {
            signature.push(';');
            signature.push_str(&field.ty(None).signature());
        }
        signature.push(')');
        signature
    }
}

impl CppStruct {
    pub fn name(&self) -> &str {
        if self.name.is_empty() {
            self.def.name()
        } else {
            &self.name
        }
    }
}