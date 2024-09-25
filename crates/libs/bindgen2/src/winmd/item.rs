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

impl Item {
    // pub fn type_name(&self) -> TypeName {
    //     match self {
    //         Self::Class(item) => item.def.type_name(),
    //         Self::Delegate(item) => item.def.type_name(),
    //         Self::Enum(item) => item.def.type_name(),
    //         Self::Interface(item) => item.def.type_name(),
    //         Self::Struct(item) => item.def.type_name(),

    //         Self::CppConst(item) => TypeName(item.def.namespace(), item.field.name()),
    //         Self::CppDelegate(item) => item.def.type_name(),
    //         Self::CppEnum(item) => item.def.type_name(),
    //         Self::CppFn(item) => TypeName(item.def.namespace(), item.method.name()),
    //         Self::CppInterface(item) => item.def.type_name(),
    //         Self::CppStruct(item) => item.def.type_name(),
    //     }
    // }

    // TODO: after this is done, build a new filter with addition type names?
    // Also use for building cfg attributes for writer.package
    // pub fn dependencies(&self, set: &mut HashSet<Type>) {

    // }
}
