mod blob;
mod codes;
mod file;
mod flags;
mod guid;
mod reader;
mod row;
mod tree;
mod type_name;
mod cfg;

use super::*;
pub use blob::*;
pub use codes::*;
pub use file::*;
pub use flags::*;
pub use guid::*;
pub use reader::*;
pub use row::*;
pub use tree::*;
pub use type_name::*;
pub use cfg::*;

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
    pub kind: InterfaceKind,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InterfaceKind {
    None,
    Default,
    Overridable,
    Static,
    Composable,
    Base,
}

#[derive(PartialEq)]
pub enum AsyncKind {
    None,
    Action,
    ActionWithProgress,
    Operation,
    OperationWithProgress,
}

#[derive(PartialEq)]
pub enum TypeKind {
    None,
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
}

pub enum Value {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    TypeDef(TypeDef),
}

pub struct Signature {
    def: MethodDef,
    params: Vec<SignatureParam>,
    return_type: Option<Type>,
}

pub struct SignatureParam {
    def: Param,
    ty: Type,
    array_info: Option<ArrayInfo>,
}
