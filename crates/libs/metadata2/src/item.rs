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

impl CppStruct {
    pub fn fields(&self) -> impl Iterator<Item = (Field, Type)> + '_ {
        self.def.fields().map(|field| {
            let mut blob = field.signature();
            blob.read_usize();
            blob.read_modifiers();
            let ty = cpp_type_from_blob(&mut blob, Some(self));

            (field, ty)
        })
    }
}

// fn read_type(blob: &mut Blob, generics: &[Type]) -> Type {
//     // WinRT type reader
//     todo!()
// }

fn cpp_type_from_blob(blob: &mut Blob, enclosing: Option<&CppStruct>) -> Type {
    let mut pointers = 0;

    while blob.try_read(ELEMENT_TYPE_PTR as usize) {
        pointers += 1;
    }

    let code = blob.read_usize();

    let mut ty = Type::from_code(code).unwrap_or_else(|| match code as u8 {
        ELEMENT_TYPE_VALUETYPE => cpp_type_from_code(blob.decode(), enclosing),
        ELEMENT_TYPE_ARRAY => {
            let ty = cpp_type_from_blob(blob, enclosing);
            let _rank = blob.read_usize();
            let _count = blob.read_usize();
            let bounds = blob.read_usize();
            Type::Win32Array(Box::new(ty), bounds)
        }
        rest => unimplemented!("{rest:?}"),
    });

    if pointers > 0 {
        ty = Type::MutPtr(Box::new(ty), pointers);
    }

    ty
}

fn cpp_type_from_code(code: TypeDefOrRef, enclosing: Option<&CppStruct>) -> Type {
    let full_name = code.type_name();

    if let Some(enclosing) = enclosing {
        if full_name.namespace().is_empty() {
            return Type::TypeDef(enclosing.nested[full_name.name()].def, vec![]);
        }
    }

    Type::TypeDef(code.resolve(), vec![])
}
