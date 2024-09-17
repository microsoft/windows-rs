use super::*;

#[derive(Clone)]
pub enum Item {
    Interface(Interface),
    Class(Class),
    Enum(Enum),
    Struct(Struct),
    Delegate(Delegate),

    // The CppXxx variants store a container of definitions to support different architectures.
    CppInterface(Vec<CppInterface>),
    CppEnum(Vec<CppEnum>),
    CppStruct(Vec<CppStruct>),
    CppCallback(Vec<CppCallback>),
    CppConst(Vec<CppConst>),
    CppFn(Vec<CppFn>),
}

#[derive(Clone)]
pub struct Interface {
    pub def: TypeDef,
}
#[derive(Clone)]
pub struct Class {
    pub def: TypeDef,
}
#[derive(Clone)]
pub struct Enum {
    pub def: TypeDef,
}
#[derive(Clone)]
pub struct Struct {
    pub def: TypeDef,
}
#[derive(Clone)]
pub struct Delegate {
    pub def: TypeDef,
}
#[derive(Clone)]
pub struct CppInterface {
    pub def: TypeDef,
}
#[derive(Clone)]
pub struct CppEnum {
    pub def: TypeDef,
}
#[derive(Clone)]
pub struct CppStruct {
    pub def: TypeDef,
    pub nested: HashMap<&'static str, CppStruct>,
}
#[derive(Clone)]
pub struct CppCallback {
    pub def: TypeDef,
}
#[derive(Clone)]
pub struct CppConst {
    pub def: Field,
}
#[derive(Clone)]
pub struct CppFn {
    pub def: MethodDef,

    // The Win32 metadata includes some attributes that assume you know what namespae the MethodDef is defined in.
    pub namespace: &'static str,
}
