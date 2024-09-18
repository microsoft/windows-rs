use super::*;

#[derive(Clone, Debug)]
pub enum Item {
    Interface(Interface),
    Class(Class),
    Enum(Enum),
    Struct(Struct),
    Delegate(Delegate),

    // This grouping for Cpp definitions is a little awkward but necessary because Win32 metadata may have:
    // * multiple definitions of the same type for different architectures
    // * mutliple definitions of different types with the same name
    Cpp(Vec<CppItem>),
}

#[derive(Clone, Debug)]
pub enum CppItem {
    Interface(CppInterface),
    Enum(CppEnum),
    Struct(CppStruct),
    Delegate(CppDelegate),
    Const(CppConst),
    Fn(CppFn),
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub def: TypeDef,
}
#[derive(Clone, Debug)]
pub struct Class {
    pub def: TypeDef,
}
#[derive(Clone, Debug)]
pub struct Enum {
    pub def: TypeDef,
}
#[derive(Clone, Debug)]
pub struct Struct {
    pub def: TypeDef,
}
#[derive(Clone, Debug)]
pub struct Delegate {
    pub def: TypeDef,
}
#[derive(Clone, Debug)]
pub struct CppInterface {
    pub def: TypeDef,
}
#[derive(Clone, Debug)]
pub struct CppEnum {
    pub def: TypeDef,
}
#[derive(Clone, Debug)]
pub struct CppStruct {
    pub def: TypeDef,
    pub nested: HashMap<&'static str, CppStruct>,
}
#[derive(Clone, Debug)]
pub struct CppDelegate {
    pub def: TypeDef,
}
#[derive(Clone, Debug)]
pub struct CppConst {
    pub def: Field,
}
#[derive(Clone, Debug)]
pub struct CppFn {
    pub def: MethodDef,

    // The Win32 metadata includes some attributes that assume you know what namespae the MethodDef is defined in.
    pub namespace: &'static str,
}
