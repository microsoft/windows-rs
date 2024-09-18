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

impl Item {
    pub(crate) fn type_def(&self) -> TypeDef {
        match self {
            Self::Interface(item) => item.def,
            Self::Class(item) => item.def,
            Self::Enum(item) => item.def,
            Self::Struct(item) => item.def,
            Self::Delegate(item) => item.def,
            Self::Cpp(item) => match &item[0] {
                CppItem::Interface(item) => item.def,
                CppItem::Enum(item) => item.def,
                CppItem::Struct(item) => item.def,
                CppItem::Delegate(item) => item.def,
                CppItem::Const(item) => item.def,
                CppItem::Fn(item) => item.def,
            },
        }
    }
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
    pub def: TypeDef,
    pub field: Field,
}
#[derive(Clone, Debug)]
pub struct CppFn {
    pub def: TypeDef,
    pub method: MethodDef,
}

impl Struct {
    pub fn fields(&self) -> impl Iterator<Item = (Field, Type)> + '_ {
        self.def.fields().map(|field| {
            let mut blob = field.signature();
            blob.read_usize();
            blob.read_modifiers();
            let ty = blob.winrt_type_from_blob(&[]);

            (field, ty)
        })
    }
}

impl CppStruct {
    pub fn fields(&self) -> impl Iterator<Item = (Field, Type)> + '_ {
        self.def.fields().map(|field| {
            let mut blob = field.signature();
            blob.read_usize();
            blob.read_modifiers();
            let ty = blob.cpp_type_from_blob(Some(self));

            (field, ty)
        })
    }
}
