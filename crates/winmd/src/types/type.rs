use crate::flags::*;
use crate::tables::*;
use crate::types::*;
use crate::Reader;

use proc_macro2::TokenStream;

#[derive(Debug)]
pub enum Type {
    Class(Class),
    Interface(Interface),
    Enum(Enum),
    Struct(Struct),
    Delegate(Delegate),
}

// impl std::fmt::Debug for Type {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "{}.{}", self.name().namespace, self.name().name)
//     }
// }

impl Type {
    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        match def.category(reader) {
            TypeCategory::Interface => Self::Interface(Interface::from_type_def(reader, def)),
            TypeCategory::Class => Self::Class(Class::from_type_def(reader, def)),
            TypeCategory::Enum => Self::Enum(Enum::from_type_def(reader, def)),
            TypeCategory::Struct => Self::Struct(Struct::from_type_def(reader, def)),
            TypeCategory::Delegate => Self::Delegate(Delegate::from_type_def(reader, def)),
        }
    }

    pub fn to_stream(&self) -> TokenStream {
        match self {
            Type::Class(t) => t.to_stream(),
            Type::Interface(t) => t.to_stream(),
            Type::Enum(t) => t.to_stream(),
            Type::Struct(t) => t.to_stream(),
            Type::Delegate(t) => t.to_stream(),
        }
    }

    pub fn name(&self) -> &TypeName {
        match self {
            Type::Class(t) => &t.name,
            Type::Interface(t) => &t.name,
            Type::Enum(t) => &t.name,
            Type::Struct(t) => &t.name,
            Type::Delegate(t) => &t.name,
        }
    }

    // TODO: ideally this would return an iterator to avoid repeated allocations
    pub fn dependencies(&self) -> Vec<TypeDef> {
        match self {
            Type::Class(t) => t.dependencies(),
            Type::Interface(t) => t.dependencies(),
            Type::Enum(_t) => Vec::new(),
            Type::Struct(t) => t.dependencies(),
            Type::Delegate(t) => t.dependencies(),
        }
    }
}
