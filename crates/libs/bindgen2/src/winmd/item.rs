use super::*;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Interface {
    pub def: TypeDef,
}
#[derive(Debug)]
pub struct Class {
    pub def: TypeDef,
}
#[derive(Debug)]
pub struct Enum {
    pub def: TypeDef,
}
#[derive(Debug)]
pub struct Struct {
    pub def: TypeDef,
}
#[derive(Debug)]
pub struct Delegate {
    pub def: TypeDef,
}
#[derive(Debug)]
pub struct CppInterface {
    pub def: TypeDef,
}
#[derive(Debug)]
pub struct CppEnum {
    pub def: TypeDef,
}
#[derive(Debug)]
pub struct CppStruct {
    pub def: TypeDef,
    pub nested: HashMap<&'static str, CppStruct>,
}
#[derive(Debug)]
pub struct CppDelegate {
    pub def: TypeDef,
}
#[derive(Debug)]
pub struct CppConst {
    pub def: TypeDef,
    pub field: Field,
}
#[derive(Debug)]
pub struct CppFn {
    pub def: TypeDef,
    pub method: MethodDef,
}
