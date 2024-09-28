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
    pub fn type_name(&self) -> TypeName {
        match self {
            winmd::Item::Class(item) => item.def.type_name(),
            winmd::Item::Delegate(item) => item.def.type_name(),
            winmd::Item::Enum(item) => item.def.type_name(),
            winmd::Item::Interface(item) => item.def.type_name(),
            winmd::Item::Struct(item) => item.def.type_name(),
            winmd::Item::CppConst(item) => item.def.type_name(),
            winmd::Item::CppDelegate(item) => item.def.type_name(),
            winmd::Item::CppEnum(item) => item.def.type_name(),
            winmd::Item::CppFn(item) => item.def.type_name(),
            winmd::Item::CppInterface(item) => item.def.type_name(),
            winmd::Item::CppStruct(item) => item.def.type_name(),
        }
    }

    // TODO: this really should be on the Iterm,TypeDef,MEthodDef, etc so that bindgen can use it for generating cfg attributes easily.
pub fn dependencies(&'static self, dependencies: &mut Dependencies) {
    match self {
        winmd::Item::Class(item) => item.dependencies(dependencies),
        winmd::Item::Delegate(item) => item.dependencies(dependencies),
        winmd::Item::Enum(item) => item.dependencies(dependencies),
        winmd::Item::Interface(item) => item.dependencies(dependencies),
        winmd::Item::Struct(item) => item.dependencies(dependencies),
        winmd::Item::CppConst(item) => item.dependencies(dependencies),
        winmd::Item::CppDelegate(item) => item.dependencies(dependencies),
        winmd::Item::CppEnum(item) => item.dependencies(dependencies),
        winmd::Item::CppFn(item) => item.dependencies(dependencies),
        winmd::Item::CppInterface(item) => item.dependencies(dependencies),
        winmd::Item::CppStruct(item) => item.dependencies(dependencies),
    }
}
}

//
//
//
//
//
// TODO: maybe need these *only* on TypeDef so we can chase dependencies through method signatures?
//
// TODO: and how to deal with limiting dependencies from undesired interface methods?
//
//
//
//

impl Class {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl Delegate {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl Enum {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl Interface {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl Struct {
    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        for field in self.def.fields() {
            field.ty(None).dependencies(dependencies);
        }
    }
}

impl CppConst {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl CppDelegate {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl CppEnum {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl CppFn {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl CppInterface {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {
        
    }
}

impl CppStruct {
    pub fn dependencies(&'static self, dependencies: &mut Dependencies) {
        for field in self.def.fields() {
            field.ty(Some(self)).dependencies(dependencies);
        }
    }
}
