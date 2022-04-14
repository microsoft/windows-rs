mod blob;
mod codes;
mod file;
mod flags;
mod reader;
mod row;
mod tree;
mod type_def_kind;
mod type_name;
mod value;

use super::*;
pub use blob::*;
pub use codes::*;
pub use file::*;
pub use flags::*;
pub use reader::*;
pub use row::*;
pub use tree::*;
pub use type_def_kind::*;
pub use type_name::*;
pub use value::*;

macro_rules! tables {
    ($($name:ident,)*) => ($(
        #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
        pub struct $name(pub Row);
    )*)
}

tables! {
    Attribute,
    ClassLayout,
    Constant,
    Field,
    GenericParam,
    ImplMap,
    InterfaceImpl,
    MemberRef,
    MethodDef,
    ModuleRef,
    Param,
    TypeDef,
    TypeRef,
    TypeSpec,
}

#[derive(Copy, Clone)]
pub enum ArrayInfo {
    Fixed(usize),
    RelativeLen(usize),
    RelativePtr(usize),
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Type {
    Void,
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    USize,
    String,
    GUID,
    IUnknown,
    IInspectable,
    HRESULT,
    PSTR,
    PWSTR,
    PCSTR,
    PCWSTR,
    TypeName,
    GenericParam(Param),
    MethodDef(MethodDef),
    Field(Field),
    TypeDef((TypeDef, Vec<Self>)),
    MutPtr((Box<Self>, usize)),
    ConstPtr((Box<Self>, usize)),
    Win32Array((Box<Self>, usize)),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    WinrtConstRef(Box<Self>),
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Interface {
    pub ty: Type,
    pub default: bool,
    pub overridable: bool,
}
