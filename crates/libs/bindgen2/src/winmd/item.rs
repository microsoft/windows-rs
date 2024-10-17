use super::*;

// TODO: implement Ord for CppFn manually so that it sorts by library and then name
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Item {
    CppFn(CppFn),

    Class(Class),
    Delegate(Delegate),
    Enum(Enum),
    Interface(Interface),
    Struct(Struct),

    CppConst(CppConst),
    CppEnum(CppEnum),
    CppInterface(CppInterface),
    CppStruct(CppStruct),
    CppDelegate(CppDelegate),
    // TODO: have psuedo items for the core types like PWSTR so that those can be written out for standalone code gen?
}

// TODO: maybe just order on Item directly
// 1. order functions first
// 2. order everything else by name
// Otherwise it looks weird when you have things like LOAD_LIBRARY_FLAGS sorting before BOOL

// impl Ord for Item {
//     fn cmp(&self, other: &Self) -> Ordering {
//     }
// }

// impl PartialOrd for Item {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Interface {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Class {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Enum {
    pub def: TypeDef,
}
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Struct {
    pub def: TypeDef,
}
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Delegate {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppInterface {
    pub def: TypeDef,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CppEnum {
    pub def: TypeDef,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CppStruct {
    pub def: TypeDef,
    pub name: String,
    pub nested: BTreeMap<&'static str, Item>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CppDelegate {
    pub def: TypeDef,
}

impl Delegate {
    pub fn method(&self) -> MethodDef {
        self.def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap()
    }
}

impl CppDelegate {
    pub fn method(&self) -> MethodDef {
        self.def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CppConst {
    pub def: TypeDef,
    pub field: Field,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CppFn {
    pub def: TypeDef,
    pub method: MethodDef,
}

impl Item {
    pub fn generics_mut(&mut self) -> &mut Vec<Type> {
        match self {
            Self::Class(item) => &mut item.generics,
            Self::Interface(item) => &mut item.generics,
            Self::Delegate(item) => &mut item.generics,
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

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

    pub fn name(&self) -> &str {
        match self {
            Self::Class(item) => item.def.name(),
            Self::Delegate(item) => item.def.name(),
            Self::Enum(item) => item.def.name(),
            Self::Interface(item) => item.def.name(),
            Self::Struct(item) => item.def.name(),
            Self::CppDelegate(item) => item.def.name(),
            Self::CppEnum(item) => item.def.name(),
            Self::CppInterface(item) => item.def.name(),
            Self::CppStruct(item) => item.name(),
            Self::CppConst(item) => item.field.name(),
            Self::CppFn(item) => item.method.name(),
        }
    }

    pub fn underlying_type(&self) -> Type {
        match self {
            Self::Struct(item) => item.def.underlying_type(),
            Self::CppEnum(item) => item.def.underlying_type(),
            Self::CppStruct(item) => item.def.underlying_type(),
            rest => panic!("windows-bindgen: {rest:?}"),
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

impl Class {
    pub fn default_interface(&self, generics: &[Type]) -> Option<Type> {
        self.def
            .interface_impls()
            .find(|imp| imp.has_attribute("DefaultAttribute"))
            .map(|imp| imp.ty(generics))
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
