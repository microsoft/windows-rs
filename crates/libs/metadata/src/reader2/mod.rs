mod blob;
mod codes;
mod file;
mod flags;
mod guid;
mod reader;
mod row;
mod tree;
mod type_name;

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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ArrayInfo {
    Fixed(usize),
    RelativeLen(usize),
    RelativePtr(usize),
    None,
    Removed,
}

impl ArrayInfo {
    pub fn is_some(&self) -> bool {
        match self {
            Self::Fixed(_) => true,
            Self::RelativeLen(_) => true,
            Self::RelativePtr(_) => true,
            _ => false,
        }
    }
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
    GenericParam(GenericParam),
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
pub enum SignatureKind {
    Query,
    QueryOptional,
    ResultValue,
    ResultVoid,
    ReturnStruct,
    ReturnVoid,
    PreserveSig,
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
    pub def: MethodDef,
    pub params: Vec<SignatureParam>,
    pub return_type: Option<Type>,
}

pub struct SignatureParam {
    pub def: Param,
    pub ty: Type,
    pub array_info: ArrayInfo,
}

#[derive(Default, Clone)]
pub struct Cfg {
    // TODO: use String for now and maybe StringRef if that's too slow
    pub types: BTreeMap<String, BTreeSet<TypeDef>>,
    pub arches: BTreeSet<&'static str>,
}

// TODO: get rid of this
impl Cfg {
    pub fn add_feature(&mut self, feature: &str) {
        self.types.entry(feature.to_string()).or_default();
    }
    pub fn union(&self, other: &Self) -> Self {
        let mut union = Self::default();
        self.types.keys().for_each(|feature| {
            union.types.entry(feature.clone()).or_default();
        });
        other.types.keys().for_each(|feature| {
            union.types.entry(feature.clone()).or_default();
        });
        self.arches.iter().for_each(|arch| {
            union.arches.insert(arch);
        });
        other.arches.iter().for_each(|arch| {
            union.arches.insert(arch);
        });
        union
    }
}
