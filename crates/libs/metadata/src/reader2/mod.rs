mod blob;
mod codes;
mod file;
mod flags;
mod reader;
mod row;
mod tree;
mod r#type;
mod type_def_kind;
mod type_name;
mod value;

use super::*;
pub use blob::*;
pub use codes::*;
pub use file::*;
pub use flags::*;
pub use r#type::*;
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
